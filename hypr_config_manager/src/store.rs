use crate::models::ThemeConfig;
use anyhow::{Result, anyhow};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use yaml_serde;

type StoreListener = Box<dyn Fn(StoreEvent) + Send + Sync>;

pub struct ThemeStore {
    staged_updates: RwLock<HashMap<String, yaml_serde::Value>>,
    current_theme_name: RwLock<String>,
    last_name_check: RwLock<Instant>,
    theme_data_cache: RwLock<HashMap<String, Arc<ThemeConfig>>>,
    listeners: RwLock<Vec<StoreListener>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreEvent {
    ThemeChanged,
    SettingsStaged,
    UpdatesCleared,
    CommitRequested,
}

impl ThemeStore {
    fn new() -> Self {
        let store = Self {
            staged_updates: RwLock::new(HashMap::new()),
            current_theme_name: RwLock::new(String::new()),
            last_name_check: RwLock::new(Instant::now() - Duration::from_secs(10)),
            theme_data_cache: RwLock::new(HashMap::new()),
            listeners: RwLock::new(Vec::new()),
        };
        store.load_current_theme_name();
        store
    }

    pub fn global() -> &'static Self {
        static INSTANCE: Lazy<ThemeStore> = Lazy::new(ThemeStore::new);
        &INSTANCE
    }

    fn load_current_theme_name(&self) {
        let path = self.get_current_theme_state_path();
        let name = if path.exists() {
            std::fs::read_to_string(&path)
                .map(|s| s.trim().to_string())
                .map(|s| {
                    PathBuf::from(s)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
                .unwrap_or_else(|_| {
                    std::env::var("TARGET_THEME").unwrap_or_else(|_| "amateur".to_string())
                })
        } else {
            std::env::var("TARGET_THEME").unwrap_or_else(|_| "amateur".to_string())
        };

        let mut name_lock = self.current_theme_name.write().unwrap();
        *name_lock = name;
        let mut check_lock = self.last_name_check.write().unwrap();
        *check_lock = Instant::now();
    }

    pub fn current_theme(&self) -> String {
        let now = Instant::now();
        let last_check = *self.last_name_check.read().unwrap();

        if now.duration_since(last_check) > Duration::from_secs(1) {
            self.load_current_theme_name();
        }

        self.current_theme_name.read().unwrap().clone()
    }

    fn get_current_theme_state_path(&self) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".config/hypr/current_theme_path")
    }

    pub fn add_listener<F>(&self, callback: F)
    where
        F: Fn(StoreEvent) + Send + Sync + 'static,
    {
        self.listeners.write().unwrap().push(Box::new(callback));
    }

    fn notify_listeners(&self, event: StoreEvent) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener(event);
        }
    }

    pub fn stage_update(&self, key: &str, value: yaml_serde::Value) {
        let mut staged = self.staged_updates.write().unwrap();
        if staged.get(key) == Some(&value) {
            return;
        }
        if std::env::args().any(|x| x == "--debug") {
            println!("[ThemeStore] Staging update: {} = {:?}", key, value);
        }
        staged.insert(key.to_string(), value);
        drop(staged);
        self.theme_data_cache.write().unwrap().clear();
        self.notify_listeners(StoreEvent::SettingsStaged);
    }

    pub fn get_staged_updates(&self) -> HashMap<String, yaml_serde::Value> {
        self.staged_updates.read().unwrap().clone()
    }

    pub fn has_staged_updates(&self) -> bool {
        !self.staged_updates.read().unwrap().is_empty()
    }

    pub fn clear_staged_updates(&self) {
        let mut staged = self.staged_updates.write().unwrap();
        if staged.is_empty() {
            return;
        }
        staged.clear();
        drop(staged);
        self.theme_data_cache.write().unwrap().clear();
        self.notify_listeners(StoreEvent::UpdatesCleared);
    }

    pub fn get_theme_data(&self, theme_name: &str) -> Result<Arc<ThemeConfig>> {
        {
            let cache = self.theme_data_cache.read().unwrap();
            if let Some(data) = cache.get(theme_name) {
                return Ok(Arc::clone(data));
            }
        }

        let data = self.load_theme_data(theme_name)?;
        let arc_data = Arc::new(data);

        let mut cache = self.theme_data_cache.write().unwrap();
        cache.insert(theme_name.to_string(), Arc::clone(&arc_data));
        Ok(arc_data)
    }

    fn load_theme_data(&self, theme_name: &str) -> Result<ThemeConfig> {
        let defaults_str = include_str!("defaults.yaml");
        let mut config_value: yaml_serde::Value = yaml_serde::from_str(defaults_str)?;

        if let Ok(theme_path) = self.get_theme_path(theme_name) {
            let theme_yaml_path = theme_path.join("theme.yaml");
            if theme_yaml_path.exists() {
                let theme_str = std::fs::read_to_string(theme_yaml_path)?;
                let theme_value: yaml_serde::Value = yaml_serde::from_str(&theme_str)?;
                self.deep_merge(&mut config_value, theme_value);
            }
        }

        // Merge user preferences templates/hypr/settings.yaml if exists
        let settings_yaml_path =
            crate::templater::get_hypr_path().join("templates/hypr/settings.yaml");
        if let Ok(settings_str) = std::fs::read_to_string(&settings_yaml_path)
            && let Ok(settings_value) = yaml_serde::from_str::<yaml_serde::Value>(&settings_str)
        {
            self.deep_merge(&mut config_value, settings_value);
        }

        // Apply staged updates so they are visible immediately in queries
        let staged = self.get_staged_updates();
        for (key, value) in staged {
            self.set_nested_value(&mut config_value, &key, value);
        }

        let mut config: ThemeConfig = yaml_serde::from_value(config_value.clone())?;

        // Dynamically auto-detect color scheme if it wasn't explicitly set in the theme.yaml
        // Or if it was set to the default "prefer-dark" but the background color is clearly light.
        if config.gtk.color_scheme == "prefer-dark" || config.gtk.color_scheme.is_empty() {
            let colors_key = yaml_serde::Value::String("colors".to_string());
            let background_key = yaml_serde::Value::String("background".to_string());
            let bg_str = config_value
                .as_mapping()
                .and_then(|m| m.get(&colors_key))
                .and_then(|c| c.as_mapping())
                .and_then(|c_m| c_m.get(&background_key))
                .and_then(|bg_val| bg_val.as_str());

            if let Some(_bg_str) = bg_str.filter(|s| is_light_color(s)) {
                config.gtk.color_scheme = "prefer-light".to_string();
                self.set_nested_value(
                    &mut config_value,
                    "gtk.color_scheme",
                    yaml_serde::Value::String("prefer-light".to_string()),
                );
            }
        }

        config.raw_value = config_value;
        Ok(config)
    }

    fn deep_merge(&self, target: &mut yaml_serde::Value, source: yaml_serde::Value) {
        match (target, source) {
            (yaml_serde::Value::Mapping(target_map), yaml_serde::Value::Mapping(source_map)) => {
                for (key, value) in source_map {
                    if let Some(target_value) = target_map.get_mut(&key) {
                        self.deep_merge(target_value, value);
                    } else {
                        target_map.insert(key, value);
                    }
                }
            }
            (target, source) => {
                *target = source;
            }
        }
    }

    fn get_targets_for_updates(&self, updates: &HashMap<String, yaml_serde::Value>) -> Vec<String> {
        let mut targets = std::collections::HashSet::new();
        let mut regenerate_all = false;

        for key in updates.keys() {
            if key.starts_with("colors.") || key.starts_with("fonts.") {
                regenerate_all = true;
                break;
            } else if key.starts_with("gtk.") || key.starts_with("cursor.") {
                targets.insert("gtk-3.0".to_string());
                targets.insert("gtk-4.0".to_string());
                if key.starts_with("cursor.") {
                    targets.insert("hypr".to_string());
                }
            } else if key.starts_with("hypr.")
                || key.starts_with("input.")
                || key.starts_with("general.")
                || key.starts_with("decoration.")
                || key.starts_with("animations.")
                || key.starts_with("layout.")
                || key.starts_with("monitors")
            {
                targets.insert("hypr".to_string());
                if key.starts_with("layout.rounding")
                    || key.starts_with("layout.border_size")
                    || key.starts_with("decoration.rounding")
                {
                    targets.insert("waybar".to_string());
                    targets.insert("walker".to_string());
                    targets.insert("swayosd".to_string());
                    targets.insert("mako".to_string());
                }
            } else if key.starts_with("kitty.") {
                targets.insert("kitty".to_string());
            } else if key.starts_with("alacritty.") {
                targets.insert("alacritty".to_string());
            } else if key.starts_with("ghostty.") {
                targets.insert("ghostty".to_string());
            } else if key.starts_with("mako.") {
                targets.insert("mako".to_string());
            } else if key.starts_with("swayosd.") {
                targets.insert("swayosd".to_string());
            } else if key.starts_with("walker.") {
                targets.insert("walker".to_string());
            } else if key.starts_with("waybar.") {
                targets.insert("waybar".to_string());
            } else if key.starts_with("btop.") {
                targets.insert("btop".to_string());
            } else if key.starts_with("fastfetch.") {
                targets.insert("fastfetch".to_string());
            } else if key.starts_with("starship.") {
                targets.insert("starship.toml".to_string());
            } else if key.starts_with("xcompose.") {
                targets.insert("xcompose".to_string());
            }
        }

        if regenerate_all || targets.is_empty() {
            Vec::new()
        } else {
            targets.into_iter().collect()
        }
    }

    pub fn commit_theme_updates(&self) -> Result<(bool, String)> {
        self.notify_listeners(StoreEvent::CommitRequested);
        let updates = self.get_staged_updates();
        if updates.is_empty() {
            return Ok((true, "No changes to save.".to_string()));
        }

        let theme_name = self.current_theme();
        let targets = self.get_targets_for_updates(&updates);

        let mut theme_updates = HashMap::new();
        let mut user_updates = HashMap::new();

        for (key, value) in updates {
            if is_theme_setting(&key) {
                theme_updates.insert(key, value);
            } else {
                user_updates.insert(key, value);
            }
        }

        // Commit Theme Updates to theme.yaml
        if !theme_updates.is_empty() {
            let theme_path = self.get_theme_path(&theme_name)?;
            let theme_yaml_path = theme_path.join("theme.yaml");

            if std::env::args().any(|x| x == "--debug") {
                println!(
                    "[ThemeStore] Writing theme updates to: {:?}",
                    theme_yaml_path
                );
                for (k, v) in &theme_updates {
                    println!("  {} = {:?}", k, v);
                }
            }

            let mut theme_yaml_value: yaml_serde::Value = if theme_yaml_path.exists() {
                let content = std::fs::read_to_string(&theme_yaml_path)?;
                yaml_serde::from_str(&content)?
            } else {
                yaml_serde::Value::Mapping(yaml_serde::Mapping::new())
            };

            for (key, value) in theme_updates {
                self.set_nested_value(&mut theme_yaml_value, &key, value);
            }

            let updated_theme_content = yaml_serde::to_string(&theme_yaml_value)?;
            std::fs::write(&theme_yaml_path, updated_theme_content)?;
        }

        // Commit User Updates to settings.yaml
        if !user_updates.is_empty() {
            let settings_yaml_path =
                crate::templater::get_hypr_path().join("templates/hypr/settings.yaml");
            if let Some(parent) = settings_yaml_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            if std::env::args().any(|x| x == "--debug") {
                println!(
                    "[ThemeStore] Writing user updates to: {:?}",
                    settings_yaml_path
                );
                for (k, v) in &user_updates {
                    println!("  {} = {:?}", k, v);
                }
            }

            let mut settings_yaml_value: yaml_serde::Value = if settings_yaml_path.exists() {
                let content = std::fs::read_to_string(&settings_yaml_path)?;
                yaml_serde::from_str(&content)?
            } else {
                yaml_serde::Value::Mapping(yaml_serde::Mapping::new())
            };

            for (key, value) in user_updates {
                self.set_nested_value(&mut settings_yaml_value, &key, value);
            }

            let updated_settings_content = yaml_serde::to_string(&settings_yaml_value)?;
            std::fs::write(&settings_yaml_path, updated_settings_content)?;
        }

        let targets_arg = targets;

        self.clear_staged_updates();

        std::thread::spawn(move || {
            let is_debug = std::env::args().any(|x| x == "--debug");
            let mut templater_path = None;
            if let Some(sibling) = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|parent| parent.join("hypr-theme-templater")))
                .filter(|sibling| sibling.exists())
            {
                templater_path = Some(sibling);
            }
            let path = templater_path.unwrap_or_else(|| {
                crate::templater::get_hypr_path().join("bin/hypr-theme-templater")
            });

            let mut cmd = if path.exists() {
                std::process::Command::new(path)
            } else {
                std::process::Command::new("hypr-theme-templater")
            };

            for target in &targets_arg {
                cmd.arg("--targets").arg(target);
            }

            if is_debug {
                println!("[ThemeStore] Executing templater command: {:?}", cmd);
            }

            match cmd.output() {
                Ok(output) => {
                    if is_debug {
                        println!("[ThemeStore] Templater status: {}", output.status);
                        let out_str = String::from_utf8_lossy(&output.stdout);
                        let err_str = String::from_utf8_lossy(&output.stderr);
                        if !out_str.is_empty() {
                            println!("[ThemeStore] Templater stdout:\n{}", out_str);
                        }
                        if !err_str.is_empty() {
                            eprintln!("[ThemeStore] Templater stderr:\n{}", err_str);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ThemeStore] Failed to execute templater: {}", e);
                }
            }

            ThemeStore::global().force_theme_changed();
        });

        Ok((true, format!("Theme {} and settings updated", theme_name)))
    }

    pub fn force_theme_changed(&self) {
        self.theme_data_cache.write().unwrap().clear();
        self.load_current_theme_name();
        self.notify_listeners(StoreEvent::ThemeChanged);
    }

    fn set_nested_value(
        &self,
        yaml: &mut yaml_serde::Value,
        key_path: &str,
        value: yaml_serde::Value,
    ) {
        let parts: Vec<&str> = key_path.split('.').collect();
        let mut current = yaml;

        for &part in &parts[..parts.len() - 1] {
            if !current.is_mapping() {
                *current = yaml_serde::Value::Mapping(yaml_serde::Mapping::new());
            }
            current = match current {
                yaml_serde::Value::Mapping(map) => {
                    let key = yaml_serde::Value::String(part.to_string());
                    map.entry(key)
                        .or_insert(yaml_serde::Value::Mapping(yaml_serde::Mapping::new()))
                }
                _ => unreachable!(),
            };
        }

        if !current.is_mapping() {
            *current = yaml_serde::Value::Mapping(yaml_serde::Mapping::new());
        }
        if let Some(map) = current.as_mapping_mut() {
            map.insert(
                yaml_serde::Value::String(parts.last().unwrap().to_string()),
                value,
            );
        }
    }

    fn get_theme_path(&self, theme_name: &str) -> Result<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        let user_theme = PathBuf::from(&home)
            .join(".config/hypr/themes")
            .join(theme_name);
        if user_theme.exists() {
            return Ok(user_theme);
        }

        let system_theme = PathBuf::from("/usr/local/share/hypr/themes").join(theme_name);
        if system_theme.exists() {
            return Ok(system_theme);
        }

        let local_theme = PathBuf::from(&home)
            .join(".local/share/hypr/themes")
            .join(theme_name);
        if local_theme.exists() {
            return Ok(local_theme);
        }

        Err(anyhow!("Theme {} not found", theme_name))
    }
}

fn is_light_color(hex: &str) -> bool {
    let clean = hex.trim_start_matches('#');
    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(255);

        let r_srgb = r as f32 / 255.0;
        let g_srgb = g as f32 / 255.0;
        let b_srgb = b as f32 / 255.0;

        // Compute relative luminance
        let l = 0.2126 * r_srgb + 0.7152 * g_srgb + 0.0722 * b_srgb;
        l > 0.5
    } else {
        false
    }
}

fn is_theme_setting(key: &str) -> bool {
    key == "name"
        || key == "author"
        || key == "version"
        || key == "description"
        || key == "wallpaper"
        || key.starts_with("colors.")
        || key == "colors"
        || key.starts_with("fonts.font_family")
        || key.starts_with("fonts.font_style")
        || key == "gtk.theme"
        || key == "gtk.icon_theme"
        || key == "gtk.cursor_theme"
        || key == "gtk.accent_color"
        || key == "cursor.theme"
        || key == "layout.active_border"
        || key == "layout.inactive_border"
        || key == "layout.nogroup_border"
        || key == "layout.nogroup_border_active"
        || key == "layout.group_border_active"
        || key == "layout.group_border_inactive"
        || key == "layout.group_border_locked_active"
        || key == "layout.group_border_locked_inactive"
        || key == "layout.groupbar_col_active"
        || key == "layout.groupbar_col_inactive"
        || key == "layout.groupbar_col_locked_active"
        || key == "layout.groupbar_col_locked_inactive"
        || key == "layout.groupbar_text_color"
        || key == "layout.shadow_color"
        || key == "layout.glow_color"
        || key.starts_with("layout.animation")
        || key == "layout.animations_enabled"
        || key.starts_with("waybar.colors.")
        || key.starts_with("modules.waybar.colors.")
        || key == "nvim.colorscheme"
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn test_theme_store_routing_and_merge() {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_home = std::env::temp_dir().join(format!("hypr_test_home_{}", duration));

        // Save the original home directory and HYPR_PATH
        let original_home = std::env::var("HOME").ok();
        let original_hypr_path = std::env::var("HYPR_PATH").ok();

        unsafe {
            std::env::set_var("HOME", &temp_home);
        }

        // Set up the directories
        let config_dir = temp_home.join(".config/hypr");
        let themes_dir = config_dir.join("themes/test_theme");
        let local_share_dir = temp_home.join(".local/share/hypr");
        let settings_dir = local_share_dir.join("templates/hypr");

        std::fs::create_dir_all(&themes_dir).unwrap();
        std::fs::create_dir_all(&settings_dir).unwrap();

        // Write theme.yaml
        let theme_yaml_content = r##"
name: test_theme
colors:
  primary: "#111111"
"##;
        std::fs::write(themes_dir.join("theme.yaml"), theme_yaml_content).unwrap();

        // Write settings.yaml
        let settings_yaml_content = r##"
layout:
  gaps_in: 10
"##;
        std::fs::write(settings_dir.join("settings.yaml"), settings_yaml_content).unwrap();

        // Write current_theme_path pointing to test_theme
        std::fs::write(
            config_dir.join("current_theme_path"),
            themes_dir.to_str().unwrap(),
        )
        .unwrap();

        // Set HYPR_PATH to local_share_dir so templater/store points to temp path
        unsafe {
            std::env::set_var("HYPR_PATH", &local_share_dir);
        }

        // Force instantiate ThemeStore
        let store = ThemeStore::new();

        // Check initial current theme name
        assert_eq!(store.current_theme(), "test_theme");

        // Test loading and merging
        let theme_data = store.get_theme_data("test_theme").unwrap();
        assert_eq!(
            theme_data.get_raw_str("colors.primary", ""),
            "#111111".to_string()
        );

        let gaps_val = theme_data.get_raw_value("layout.gaps_in").unwrap();
        assert_eq!(gaps_val.as_i64(), Some(10));

        // Stage update for layout.gaps_in (user preference)
        store.stage_update("layout.gaps_in", yaml_serde::Value::Number(20.into()));

        // Stage update for colors.primary (theme preference)
        store.stage_update(
            "colors.primary",
            yaml_serde::Value::String("#222222".to_string()),
        );

        // Verify stage updates are returned in queries immediately
        let updated_data = store.get_theme_data("test_theme").unwrap();
        assert_eq!(
            updated_data
                .get_raw_value("layout.gaps_in")
                .unwrap()
                .as_i64(),
            Some(20)
        );
        assert_eq!(
            updated_data.get_raw_str("colors.primary", ""),
            "#222222".to_string()
        );

        // Commit changes!
        let _ = store.commit_theme_updates().unwrap();

        // Verify that colors.primary is updated in themes_dir/theme.yaml
        let theme_content = std::fs::read_to_string(themes_dir.join("theme.yaml")).unwrap();
        let parsed_theme: yaml_serde::Value = yaml_serde::from_str(&theme_content).unwrap();
        assert_eq!(
            parsed_theme
                .as_mapping()
                .unwrap()
                .get(yaml_serde::Value::String("colors".to_string()))
                .unwrap()
                .as_mapping()
                .unwrap()
                .get(yaml_serde::Value::String("primary".to_string()))
                .unwrap()
                .as_str(),
            Some("#222222")
        );

        // Verify that layout.gaps_in is updated in settings_dir/settings.yaml
        let settings_content = std::fs::read_to_string(settings_dir.join("settings.yaml")).unwrap();
        let parsed_settings: yaml_serde::Value = yaml_serde::from_str(&settings_content).unwrap();
        assert_eq!(
            parsed_settings
                .as_mapping()
                .unwrap()
                .get(yaml_serde::Value::String("layout".to_string()))
                .unwrap()
                .as_mapping()
                .unwrap()
                .get(yaml_serde::Value::String("gaps_in".to_string()))
                .unwrap()
                .as_i64(),
            Some(20)
        );

        // Cleanup temp home
        let _ = std::fs::remove_dir_all(&temp_home);

        // Restore environment variables
        unsafe {
            if let Some(h) = original_home {
                std::env::set_var("HOME", h);
            } else {
                std::env::remove_var("HOME");
            }
            if let Some(p) = original_hypr_path {
                std::env::set_var("HYPR_PATH", p);
            } else {
                std::env::remove_var("HYPR_PATH");
            }
        }
    }
}
