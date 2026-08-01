use anyhow::{Result, anyhow};
use minijinja::{Environment, State, Value};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use walkdir::WalkDir;

use crate::store::ThemeStore;

macro_rules! log_info {
    ($($arg:tt)*) => {{
        println!($($arg)*);
        let _ = std::io::stdout().flush();
    }};
}

// Define custom filters
fn parse_to_hex(color_str: &str) -> String {
    let first_part = color_str.split_whitespace().next().unwrap_or("").trim();
    if first_part.is_empty() {
        return "#ffffff".to_string();
    }

    if first_part.starts_with("rgba(") && first_part.ends_with(')') {
        let hex = &first_part[5..first_part.len() - 1];
        let clean = hex.trim_start_matches('#');
        return format!("#{}", clean);
    }

    if first_part.starts_with("rgb(") && first_part.ends_with(')') {
        let hex = &first_part[4..first_part.len() - 1];
        let clean = hex.trim_start_matches('#');
        return format!("#{}", clean);
    }

    if let Some(hex) = first_part.strip_prefix("0x") {
        if hex.len() == 8 {
            let alpha = &hex[0..2];
            let rgb = &hex[2..8];
            return format!("#{}{}", rgb, alpha);
        } else if hex.len() == 6 {
            return format!("#{}", hex);
        }
    }

    let clean = first_part.trim_start_matches('#');
    if clean.len() == 6 || clean.len() == 8 {
        return format!("#{}", clean);
    }

    first_part.to_string()
}

fn hypr_color_filter(_state: &State, value: Value) -> Result<Value, minijinja::Error> {
    if let Some(color_str) = value.as_str() {
        return Ok(Value::from(parse_to_hex(color_str)));
    }
    Ok(value)
}

fn hypr_color_alpha_filter(
    _state: &State,
    value: Value,
    args: &[Value],
) -> Result<Value, minijinja::Error> {
    let alpha = args.first().and_then(|v| v.as_str()).unwrap_or("ff");
    if let Some(color_str) = value.as_str() {
        let hex = parse_to_hex(color_str);
        let clean = hex.trim_start_matches('#');
        if clean.len() >= 6 {
            let rgb = &clean[0..6];
            return Ok(Value::from(format!("#{}{}", rgb, alpha)));
        }
    }
    Ok(value)
}

fn hypr_mix_color_filter(
    _state: &State,
    value: Value,
    args: &[Value],
) -> Result<Value, minijinja::Error> {
    let base_str = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "mix_color base must be a string",
        )
    })?;

    let mix_val = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "mix_color requires a mix color argument",
        )
    })?;

    let factor = args
        .get(1)
        .and_then(|v| v.to_string().parse::<f64>().ok())
        .unwrap_or(0.07) as f32;

    let base_hex = parse_to_hex(base_str);
    let mix_hex = parse_to_hex(mix_val);

    let clean_base = base_hex.trim_start_matches('#');
    let clean_mix = mix_hex.trim_start_matches('#');

    if clean_base.len() >= 6 && clean_mix.len() >= 6 {
        let r1 = u8::from_str_radix(&clean_base[0..2], 16).unwrap_or(255) as f32;
        let g1 = u8::from_str_radix(&clean_base[2..4], 16).unwrap_or(255) as f32;
        let b1 = u8::from_str_radix(&clean_base[4..6], 16).unwrap_or(255) as f32;

        let r2 = u8::from_str_radix(&clean_mix[0..2], 16).unwrap_or(0) as f32;
        let g2 = u8::from_str_radix(&clean_mix[2..4], 16).unwrap_or(0) as f32;
        let b2 = u8::from_str_radix(&clean_mix[4..6], 16).unwrap_or(0) as f32;

        let r = ((1.0 - factor) * r1 + factor * r2)
            .round()
            .clamp(0.0, 255.0) as u8;
        let g = ((1.0 - factor) * g1 + factor * g2)
            .round()
            .clamp(0.0, 255.0) as u8;
        let b = ((1.0 - factor) * b1 + factor * b2)
            .round()
            .clamp(0.0, 255.0) as u8;

        return Ok(Value::from(format!("#{:02x}{:02x}{:02x}", r, g, b)));
    }

    Ok(value)
}

fn hypr_contrast_fg_filter(
    _state: &State,
    value: Value,
    args: &[Value],
) -> Result<Value, minijinja::Error> {
    let bg_str = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "contrast_fg input must be a string",
        )
    })?;

    let dark_fg = args.first().and_then(|v| v.as_str()).unwrap_or("#121212");
    let light_fg = args.get(1).and_then(|v| v.as_str()).unwrap_or("#ffffff");

    let hex = parse_to_hex(bg_str);
    let clean = hex.trim_start_matches('#');

    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(255) as f32;
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(255) as f32;
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(255) as f32;

        let r_srgb = r / 255.0;
        let g_srgb = g / 255.0;
        let b_srgb = b / 255.0;

        // Compute relative luminance
        let l = 0.2126 * r_srgb + 0.7152 * g_srgb + 0.0722 * b_srgb;
        if l > 0.5 {
            return Ok(Value::from(dark_fg.to_string()));
        } else {
            return Ok(Value::from(light_fg.to_string()));
        }
    }

    Ok(Value::from(light_fg.to_string()))
}

fn hypr_border_filter(_state: &State, value: Value) -> Result<Value, minijinja::Error> {
    if let Some(s) = value.as_str() {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut out = Vec::new();
        for p in parts {
            if p.starts_with('#') {
                let clean = p.trim_start_matches('#');
                if clean.len() >= 6 {
                    let rgb = &clean[0..6];
                    let alpha = if clean.len() == 8 { &clean[6..8] } else { "ff" };
                    out.push(format!("rgba({}{})", rgb, alpha));
                } else {
                    out.push(p.to_string());
                }
            } else {
                out.push(p.to_string());
            }
        }
        return Ok(Value::from(out.join(" ")));
    }
    Ok(value)
}

fn parse_to_hypr_hex(color_str: &str) -> String {
    let clean = color_str.trim();
    if clean.starts_with("rgba(") && clean.ends_with(')') {
        let hex = &clean[5..clean.len() - 1];
        let h = hex.trim_start_matches('#');
        if h.len() == 8 {
            let rgb = &h[0..6];
            let alpha = &h[6..8];
            return format!("0x{}{}", alpha, rgb);
        } else if h.len() == 6 {
            return format!("0xff{}", h);
        }
    }
    if clean.starts_with("rgb(") && clean.ends_with(')') {
        let hex = &clean[4..clean.len() - 1];
        let h = hex.trim_start_matches('#');
        if h.len() == 6 {
            return format!("0xff{}", h);
        }
    }
    if clean.starts_with("0x") {
        return clean.to_string();
    }
    if let Some(h) = clean.strip_prefix('#') {
        if h.len() == 8 {
            let rgb = &h[0..6];
            let alpha = &h[6..8];
            return format!("0x{}{}", alpha, rgb);
        } else if h.len() == 6 {
            return format!("0xff{}", h);
        }
    }
    // Fallback
    clean.to_string()
}

fn hypr_lua_border_filter(_state: &State, value: Value) -> Result<Value, minijinja::Error> {
    if let Some(s) = value.as_str() {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(Value::from("\"\""));
        }
        if parts.len() == 1 {
            let p = parts[0];
            if !p.ends_with("deg") && p.parse::<f64>().is_err() {
                return Ok(Value::from(format!("\"{}\"", parse_to_hypr_hex(p))));
            }
        }
        let mut colors = Vec::new();
        let mut angle = None;
        for p in parts {
            if p.ends_with("deg") {
                let num_str = &p[0..p.len() - 3];
                if let Ok(val) = num_str.parse::<i32>() {
                    angle = Some(val);
                }
            } else if let Ok(val) = p.parse::<i32>() {
                angle = Some(val);
            } else {
                colors.push(format!("\"{}\"", parse_to_hypr_hex(p)));
            }
        }
        let colors_str = format!("{{ {} }}", colors.join(", "));
        if let Some(ang) = angle {
            return Ok(Value::from(format!(
                "{{ colors = {}, angle = {} }}",
                colors_str, ang
            )));
        } else {
            return Ok(Value::from(format!("{{ colors = {} }}", colors_str)));
        }
    }
    Ok(value)
}

fn hypr_shadow_filter(_state: &State, value: Value) -> Result<Value, minijinja::Error> {
    hypr_color_filter(_state, value)
}

pub fn get_hypr_path() -> PathBuf {
    if let Ok(p) = std::env::var("HYPR_PATH") {
        let path = PathBuf::from(p);
        if path.exists() {
            return path;
        }
    }
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".local/share/hypr")
}

fn check_hw(profile: &str) -> bool {
    let hypr_path = get_hypr_path();
    let hw_bin = hypr_path.join("bin/hypr-hw");
    let status = if hw_bin.exists() {
        Command::new(hw_bin).arg("check").arg(profile).status()
    } else {
        Command::new("hypr-hw").arg("check").arg(profile).status()
    };
    status.map(|s| s.success()).unwrap_or(false)
}

pub fn generate_config(
    theme_path_opt: Option<PathBuf>,
    backup: bool,
    updates: Option<Vec<String>>,
    debug: bool,
) -> Result<Vec<PathBuf>> {
    let is_theme_change = theme_path_opt.is_some();
    let hypr_path = get_hypr_path();
    let templates_dir = hypr_path.join("templates");
    let config_dir = PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config");

    let current_theme_state = config_dir.join("hypr/current_theme_path");

    let mut theme_path = theme_path_opt;
    if theme_path.is_none() {
        if let Ok(content) = fs::read_to_string(&current_theme_state) {
            let s = content.trim();
            if !s.is_empty() {
                theme_path = Some(PathBuf::from(s));
            }
        }

        if theme_path.is_none()
            && let Ok(target_theme) = std::env::var("TARGET_THEME")
        {
            let target_theme = target_theme.trim();
            if !target_theme.is_empty() {
                let user_themes_dir = config_dir.join("hypr/themes").join(target_theme);
                let system_themes_dir = hypr_path.join("themes").join(target_theme);

                if user_themes_dir.exists() {
                    theme_path = Some(user_themes_dir);
                } else if system_themes_dir.exists() {
                    theme_path = Some(system_themes_dir);
                } else if PathBuf::from(target_theme).exists() {
                    theme_path = Some(PathBuf::from(target_theme));
                }
            }
        }
    }

    let theme_name = if let Some(ref p) = theme_path {
        p.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    } else {
        "amateur".to_string()
    };

    let has_nvim_override = if let Some(ref p) = theme_path {
        p.join("neovim.theme.lua").exists() || p.join("neovim.lua").exists()
    } else {
        false
    };

    let store = ThemeStore::global();
    let theme_data = store
        .get_theme_data(&theme_name)
        .map_err(|e| anyhow!("Failed to load theme data: {}", e))?;
    let raw_value = theme_data.raw_value.clone();

    // Convert yaml_serde::Value to serde_json::Value to pass to minijinja easily
    let json_value: serde_json::Value = serde_json::to_value(&raw_value)?;

    let mut env = Environment::new();
    env.add_filter("hypr_color", hypr_color_filter);
    env.add_filter("hypr_border", hypr_border_filter);
    env.add_filter("hypr_lua_border", hypr_lua_border_filter);
    env.add_filter("hypr_shadow", hypr_shadow_filter);
    env.add_filter("hypr_color_alpha", hypr_color_alpha_filter);
    env.add_filter("hypr_mix_color", hypr_mix_color_filter);
    env.add_filter("hypr_contrast_fg", hypr_contrast_fg_filter);
    env.add_filter("string", |v: Value| -> String { v.to_string() });
    env.add_filter("split", |v: Value, sep: Option<String>| -> Vec<String> {
        let s = v.to_string();
        if let Some(sep_str) = sep {
            s.split(&sep_str).map(|x| x.to_string()).collect()
        } else {
            s.split_whitespace().map(|x| x.to_string()).collect()
        }
    });

    let templates_dir_clone = templates_dir.clone();
    env.set_loader(move |name| {
        let path = templates_dir_clone.join(name);
        match fs::read_to_string(&path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(minijinja::Error::new(
                minijinja::ErrorKind::TemplateNotFound,
                format!("failed to read template {:?}: {}", path, e),
            )),
        }
    });

    let hypr_path_clone = hypr_path.clone();
    env.add_function("bin", move |name: String| -> String {
        hypr_path_clone
            .join("bin")
            .join(name)
            .to_string_lossy()
            .to_string()
    });

    #[derive(serde::Serialize)]
    struct ColorValue {
        hex: String,
        csv: String,
    }

    env.add_function("Color", move |color_val: Value| -> Value {
        let color_str = if color_val.is_none() || color_val.is_undefined() {
            "".to_string()
        } else if let Some(s) = color_val.as_str() {
            s.to_string()
        } else {
            color_val.to_string()
        };

        if color_str.is_empty() {
            return Value::UNDEFINED;
        }
        let clean = color_str.trim_start_matches('#');
        if clean.len() >= 6
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&clean[0..2], 16),
                u8::from_str_radix(&clean[2..4], 16),
                u8::from_str_radix(&clean[4..6], 16),
            )
        {
            let csv = format!("{}, {}, {}", r, g, b);
            return Value::from_serialize(ColorValue {
                hex: color_str.clone(),
                csv,
            });
        }
        Value::from_serialize(ColorValue {
            hex: color_str.clone(),
            csv: color_str,
        })
    });

    env.add_global("theme", Value::from_serialize(&json_value));

    let mut env_map = std::collections::HashMap::new();
    for (k, v) in std::env::vars() {
        env_map.insert(k, v);
    }

    let accent_color = theme_data.get_raw_str("colors.accent", "");
    let primary_color = theme_data.get_raw_str("colors.primary", "#3584e4");
    let resolved_accent = if accent_color.is_empty() {
        primary_color
    } else {
        accent_color
    };
    let accent_color_hex_str = parse_to_hex(&resolved_accent);

    // Dynamically query hardware detection profiles using the hypr-hw tool
    let context_val = minijinja::context! {
        theme => Value::from_serialize(&json_value),
        env => Value::from_serialize(&env_map),
        HYPR_PATH => hypr_path.to_string_lossy().to_string(),
        is_vm => check_hw("vm"),
        is_vmware => check_hw("vmware"),
        is_qemu => check_hw("qemu"),
        is_virgl => check_hw("virgl"),
        is_arm64 => check_hw("arm64"),
        is_nvidia => check_hw("nvidia"),
        is_apple => check_hw("apple"),
        is_apple_virt => check_hw("apple-virt"),
        is_asus => check_hw("asus"),
        is_asus_rog => check_hw("asus-rog"),
        is_framework => check_hw("framework"),
        is_dell_xps => check_hw("dell-xps"),
        is_surface => check_hw("surface"),
        has_mx_master => check_hw("mx-master"),
        is_laptop => check_hw("laptop"),
        has_battery => check_hw("battery"),
        has_backlight => check_hw("backlight"),
        zen_mode => check_hw("zen-mode"),
        accent_color_hex => accent_color_hex_str,
        has_nvim_override => has_nvim_override,
    };

    if !templates_dir.exists() {
        return Err(anyhow!(
            "Templates directory not found: {:?}",
            templates_dir
        ));
    }

    // Set up transactional sandbox
    let sandbox_dir = std::env::temp_dir().join("hypr-sandbox");
    if sandbox_dir.exists() {
        let _ = fs::remove_dir_all(&sandbox_dir);
    }
    fs::create_dir_all(&sandbox_dir)?;

    struct StagedWrite {
        dest_path: PathBuf,
        sandboxed_path: PathBuf,
    }
    let mut staged_writes: Vec<StagedWrite> = Vec::new();

    let get_sandboxed_path = |dest_path: &std::path::Path| -> PathBuf {
        let home = PathBuf::from(std::env::var("HOME").unwrap_or_default());
        if let Ok(rel) = dest_path.strip_prefix(&home) {
            sandbox_dir.join(rel)
        } else if let Ok(rel) = dest_path.strip_prefix("/") {
            sandbox_dir.join(rel)
        } else {
            sandbox_dir.join(dest_path)
        }
    };

    // Walk templates dir
    for entry in WalkDir::new(&templates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(&templates_dir)?;
            if rel_path.starts_with("partials") {
                continue;
            }

            if let Some(ref targets) = updates {
                let rel_str = rel_path.to_string_lossy();
                let mut matched = false;
                for t in targets {
                    if rel_str.contains(t) {
                        matched = true;
                        break;
                    }
                }
                // Also support TEMPLATE_MAPPING logic loosely by just regenerating everything if not matched for now, or skip
                if !matched && !targets.is_empty() {
                    continue; // Skip this file since it's not in updates
                }
            }

            let rel_path = path.strip_prefix(&templates_dir)?;
            let is_j2 = path.extension().map(|e| e == "j2").unwrap_or(false);

            let dest_rel_path = if is_j2 {
                rel_path.with_extension("")
            } else {
                rel_path.to_path_buf()
            };

            let mut dest_path = config_dir.join(&dest_rel_path);

            let dest_rel_str = dest_rel_path.to_string_lossy();
            if dest_rel_str == "xcompose" {
                dest_path =
                    PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".XCompose");
            } else if dest_rel_str == "walker/elephant.toml" {
                dest_path = config_dir.join("elephant/config.toml");
            } else if dest_rel_str.ends_with(".desktop") {
                dest_path = PathBuf::from(std::env::var("HOME").unwrap_or_default())
                    .join(".local/share/applications")
                    .join(dest_rel_path.file_name().unwrap_or_default());
            } else if dest_rel_str.ends_with(".ttf")
                || dest_rel_str.ends_with(".otf")
                || dest_rel_str.ends_with(".woff")
                || dest_rel_str.ends_with(".woff2")
            {
                dest_path = PathBuf::from(std::env::var("HOME").unwrap_or_default())
                    .join(".local/share/fonts")
                    .join(dest_rel_path.file_name().unwrap_or_default());
            }

            if dest_path.ends_with("gtk-3.0/bookmarks")
                || dest_path
                    .file_name()
                    .map(|n| n == "bookmarks")
                    .unwrap_or_default()
            {
                log_info!(
                    "[WARN] Safeguard: Skipping template rendering/copying for Nautilus bookmarks path: {}",
                    dest_path.display()
                );
                continue;
            }

            let sandboxed_path = get_sandboxed_path(&dest_path);
            if let Some(parent) = sandboxed_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            // Should we skip specific extensions?
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let binary_exts = [
                "png", "jpg", "jpeg", "gif", "webp", "ttf", "otf", "woff", "woff2",
            ];

            if binary_exts.contains(&ext) {
                if let Ok(content) = fs::read(path) {
                    if debug {
                        log_info!(
                            "[INFO] Staging binary asset to sandbox: {}",
                            dest_path.display()
                        );
                    }
                    let _ = fs::write(&sandboxed_path, content);
                    staged_writes.push(StagedWrite {
                        dest_path,
                        sandboxed_path,
                    });
                }
            } else {
                let content = fs::read_to_string(path)?;
                if is_j2 || content.contains("{{") || content.contains("{%") {
                    match env.render_str(&content, context_val.clone()) {
                        Ok(rendered) => {
                            if debug {
                                log_info!(
                                    "[INFO] Staging template rendering to sandbox: {}",
                                    dest_path.display()
                                );
                            }
                            let _ = fs::write(&sandboxed_path, rendered);
                            staged_writes.push(StagedWrite {
                                dest_path,
                                sandboxed_path,
                            });
                        }
                        Err(e) => {
                            let _ = fs::remove_dir_all(&sandbox_dir);
                            return Err(anyhow!("Failed to render template {:?}: {}", rel_path, e));
                        }
                    }
                } else {
                    if debug {
                        log_info!(
                            "[INFO] Staging config asset to sandbox: {}",
                            dest_path.display()
                        );
                    }
                    let _ = fs::copy(path, &sandboxed_path);
                    staged_writes.push(StagedWrite {
                        dest_path,
                        sandboxed_path,
                    });
                }
            }
        }
    }

    // Process theme overrides (files in theme dir that override templates)
    if let Some(ref p) = theme_path
        && p.exists()
    {
        for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let rel_path = path.strip_prefix(p)?;
                if rel_path.to_string_lossy() == "theme.yaml"
                    || rel_path.to_string_lossy() == "wallpaper.jpg"
                {
                    continue;
                }

                let rel_str = rel_path.to_string_lossy();
                if rel_str.contains("backgrounds/") && updates.is_some() {
                    continue;
                }

                if let Some(ref targets) = updates {
                    let mut matched = false;
                    for t in targets {
                        if rel_str.contains(t) {
                            matched = true;
                            break;
                        }
                    }
                    if !matched && !targets.is_empty() {
                        continue;
                    }
                }

                let mut dest_path = config_dir.join(rel_path);
                let rel_str = rel_path.to_string_lossy();
                if rel_str == "alacritty.theme.toml" {
                    dest_path = config_dir.join("alacritty/theme.toml");
                } else if rel_str == "btop.theme" {
                    dest_path = config_dir.join("btop/themes/current.theme");
                } else if rel_str == "chromium.theme" {
                    dest_path = config_dir.join("hypr/chromium.theme");
                } else if rel_str == "ghostty.theme.conf" {
                    dest_path = config_dir.join("ghostty/theme.conf");
                } else if rel_str == "hyprlock.theme.conf" {
                    dest_path = config_dir.join("hypr/hyprlock.theme.conf");
                } else if rel_str == "kitty.theme.conf" {
                    dest_path = config_dir.join("kitty/theme.conf");
                } else if rel_str == "mako.theme.conf" {
                    dest_path = config_dir.join("mako/theme.conf");
                } else if rel_str == "neovim.theme.lua" || rel_str == "neovim.lua" {
                    dest_path = config_dir.join("nvim/lua/plugins/theme.lua");
                } else if rel_str.starts_with("colors/") {
                    dest_path = config_dir.join("nvim").join(rel_path);
                } else if rel_str == "swayosd.theme.css" {
                    dest_path = config_dir.join("swayosd/theme.css");
                } else if rel_str == "walker.theme.css" {
                    dest_path = config_dir.join("walker/theme.css");
                } else if rel_str == "waybar.theme.css" {
                    dest_path = config_dir.join("waybar/theme.css");
                }

                if dest_path.ends_with("gtk-3.0/bookmarks")
                    || dest_path
                        .file_name()
                        .map(|n| n == "bookmarks")
                        .unwrap_or_default()
                {
                    log_info!(
                        "[WARN] Safeguard: Skipping theme override for Nautilus bookmarks path: {}",
                        dest_path.display()
                    );
                    continue;
                }

                let sandboxed_path = get_sandboxed_path(&dest_path);
                if let Some(parent) = sandboxed_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                log_info!(
                    "[INFO] Staging theme override to sandbox: {}",
                    dest_path.display()
                );
                let _ = fs::copy(path, &sandboxed_path);
                staged_writes.push(StagedWrite {
                    dest_path,
                    sandboxed_path,
                });
            }
        }
    }

    // Run validations on the files in the sandbox
    for staged in &staged_writes {
        let file_name = staged
            .dest_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Validate Hyprland Config
        if file_name == "hyprland.lua" || file_name == "hyprland.conf" {
            log_info!("[INFO] Validating Hyprland config: {:?}", staged.dest_path);
            let status = Command::new("Hyprland")
                .args(["--verify-config", "--config"])
                .arg(&staged.sandboxed_path)
                .status();

            match status {
                Ok(s) if s.success() => {
                    log_info!("[OK] Hyprland configuration is valid.");
                }
                Ok(s) => {
                    let _ = fs::remove_dir_all(&sandbox_dir);
                    return Err(anyhow!(
                        "Hyprland configuration validation failed for {:?} with exit status: {}",
                        staged.dest_path,
                        s
                    ));
                }
                Err(e) => {
                    log_info!(
                        "[WARN] Hyprland binary not available or failed to run for validation: {}",
                        e
                    );
                }
            }
        }

        // Validate GTK CSS Config
        if staged.sandboxed_path.exists()
            && staged
                .dest_path
                .extension()
                .map(|e| e == "css")
                .unwrap_or(false)
            && (staged.dest_path.to_string_lossy().contains("gtk-3.0")
                || staged.dest_path.to_string_lossy().contains("gtk-4.0"))
        {
            log_info!("[INFO] Validating GTK CSS config: {:?}", staged.dest_path);
            let python_script = r#"
import sys, gi
gi.require_version('Gtk', '4.0')
from gi.repository import Gtk
provider = Gtk.CssProvider()
has_err = False
def on_err(p, s, e):
    global has_err
    has_err = True
    print(f"CSS Parsing Error: {e.message}", file=sys.stderr)
provider.connect('parsing-error', on_err)
try:
    provider.load_from_path(sys.argv[1])
except Exception as ex:
    has_err = True
    print(f"CSS Load Error: {ex}", file=sys.stderr)
if has_err:
    sys.exit(1)
"#;
            let output = Command::new("python3")
                .args(["-c", python_script])
                .arg(&staged.sandboxed_path)
                .output();

            match output {
                Ok(out) => {
                    if !out.status.success() {
                        let err_msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
                        let _ = fs::remove_dir_all(&sandbox_dir);
                        return Err(anyhow!(
                            "GTK CSS validation failed for {:?}: {}",
                            staged.dest_path,
                            err_msg
                        ));
                    }
                    log_info!("[OK] GTK CSS configuration is valid.");
                }
                Err(e) => {
                    log_info!(
                        "[WARN] python3/GTK4 not available or failed to run for CSS validation: {}",
                        e
                    );
                }
            }
        }
    }

    // Save theme state
    if let Some(ref p) = theme_path
        && let Some(parent) = current_theme_state.parent()
    {
        let _ = fs::create_dir_all(parent);
        let _ = fs::write(&current_theme_state, p.to_string_lossy().as_ref());
    }

    // Validation passed, commit the transactions
    let mut modified_files = Vec::new();
    if is_theme_change {
        modified_files.push(current_theme_state.clone());
    }

    for staged in staged_writes {
        if backup && staged.dest_path.exists() {
            let _ = fs::rename(
                &staged.dest_path,
                format!("{}.bak", staged.dest_path.to_string_lossy()),
            );
        }
        if let Some(parent) = staged.dest_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if debug {
            log_info!("[INFO] Committing file: {}", staged.dest_path.display());
        }
        if fs::rename(&staged.sandboxed_path, &staged.dest_path).is_err() {
            fs::copy(&staged.sandboxed_path, &staged.dest_path)?;
        }
        modified_files.push(staged.dest_path);
    }

    let _ = fs::remove_dir_all(&sandbox_dir);
    Ok(modified_files)
}

pub fn reload_applications(modified_files: Vec<PathBuf>) -> Result<Vec<String>> {
    log_info!("Reloading applications...");
    let mut reloaded = Vec::new();

    // Invalidate ThemeStore cache and reload the active theme name from disk to ensure
    // subsequent configuration/reload steps (like GTK/gsettings) get the newly applied theme's settings.
    ThemeStore::global().force_theme_changed();

    let mut needs_waybar = false;
    let mut needs_hyprland = false;
    let mut needs_swayosd = false;
    let mut needs_mako = false;
    let mut needs_terminal = false;
    let mut needs_external_refresh = false;
    let mut needs_walker = false;
    let mut background_images = Vec::new();

    for f in &modified_files {
        let p = f.to_string_lossy();
        if p.contains("current_theme_path") {
            needs_waybar = true;
            needs_hyprland = true;
            needs_swayosd = true;
            needs_mako = true;
            needs_terminal = true;
            needs_external_refresh = true;
            needs_walker = true;
        }
        if p.contains("waybar/") {
            needs_waybar = true;
        }
        if p.contains("hypr/") {
            needs_hyprland = true;
        }
        if p.contains("swayosd/") {
            needs_swayosd = true;
        }
        if p.contains("mako/") {
            needs_mako = true;
        }
        if p.contains("walker/") || p.contains("elephant/") {
            needs_walker = true;
        }
        if p.contains("alacritty/") || p.contains("kitty/") || p.contains("ghostty/") {
            needs_terminal = true;
        }
        if p.contains("chromium.theme")
            || p.contains("flags.conf")
            || p.contains("theme.yaml")
            || p.contains("obsidian.css")
            || p.contains("vscode.json")
            || p.contains("gtk-")
            || p.contains("settings.ini")
        {
            needs_external_refresh = true;
        }
        if p.contains("/backgrounds/")
            && (p.ends_with(".jpg")
                || p.ends_with(".jpeg")
                || p.ends_with(".png")
                || p.ends_with(".webp"))
        {
            background_images.push(f.clone());
        }
    }

    if needs_waybar {
        log_info!("[INFO] Restarting Waybar...");
        let waybar_script = get_hypr_path().join("bin/hypr-restart-waybar");
        if waybar_script.exists() {
            let _ = Command::new(waybar_script).spawn();
        } else {
            let _ = Command::new("hypr-restart-waybar").spawn();
        }
        reloaded.push("Waybar".to_string());
    }

    if needs_hyprland {
        log_info!("[INFO] Reloading Hyprland...");
        let _ = Command::new("hyprctl").arg("reload").spawn();
        reloaded.push("Hyprland".to_string());

        // Synchronize touchpad state based on theme configuration
        let store = ThemeStore::global();
        let current_theme = store.current_theme();
        if let Ok(data) = store.get_theme_data(&current_theme) {
            let toggle_script = get_hypr_path().join("bin/hypr-toggle-touchpad");
            if toggle_script.exists() {
                let state_arg = if data.input.touchpad_enabled {
                    "on"
                } else {
                    "off"
                };
                log_info!("[INFO] Synchronizing touchpad state to: {}", state_arg);
                let _ = Command::new(toggle_script).arg(state_arg).status();
            }
        }
    }

    if needs_swayosd {
        log_info!("[INFO] Restarting SwayOSD...");
        let swayosd_script = get_hypr_path().join("bin/hypr-restart-swayosd");
        if swayosd_script.exists() {
            let _ = Command::new(swayosd_script).spawn();
        } else {
            let _ = Command::new("hypr-restart-swayosd").spawn();
        }
        reloaded.push("SwayOSD".to_string());
    }

    if needs_mako {
        log_info!("[INFO] Restarting Mako...");
        let mako_script = get_hypr_path().join("bin/hypr-restart-mako");
        if mako_script.exists() {
            let _ = Command::new(mako_script).spawn();
        } else {
            let _ = Command::new("hypr-restart-mako").spawn();
        }
        reloaded.push("Mako".to_string());
    }

    if needs_walker {
        log_info!("[INFO] Restarting Walker...");
        let walker_script = get_hypr_path().join("bin/hypr-restart-walker");
        if walker_script.exists() {
            let _ = Command::new(walker_script).spawn();
        } else {
            let _ = Command::new("hypr-restart-walker").spawn();
        }
        reloaded.push("Walker".to_string());
    }

    if needs_terminal {
        log_info!("[INFO] Refreshing terminal color schemes...");
        let terminal_script = get_hypr_path().join("bin/hypr-restart-terminal");
        if terminal_script.exists() {
            let _ = Command::new(terminal_script).spawn();
        } else {
            let _ = Command::new("hypr-restart-terminal").spawn();
        }
        reloaded.push("Terminals".to_string());
    }

    let home = std::env::var("HOME").unwrap_or_default();
    let current_bg_link = PathBuf::from(&home).join(".config/hypr/current_background");

    let is_theme_change = modified_files
        .iter()
        .any(|f| f.to_string_lossy().contains("current_theme_path"));
    let install_mode = std::env::var("INSTALL_MODE").unwrap_or_default();
    let is_update = install_mode == "UPDATE";

    if !is_update && !background_images.is_empty() && (is_theme_change || !current_bg_link.exists())
    {
        let mut sorted_bg = background_images.clone();
        sorted_bg.sort();
        let first_bg = &sorted_bg[0];
        log_info!(
            "[INFO] Setting wallpaper to first theme background: {}",
            first_bg.display()
        );
        let wallpaper_script = get_hypr_path().join("bin/hypr-wallpaper-set");
        let status = if wallpaper_script.exists() {
            Command::new(wallpaper_script).arg(first_bg).spawn()
        } else {
            Command::new("hypr-wallpaper-set").arg(first_bg).spawn()
        };
        if status.is_ok() {
            reloaded.push("Wallpaper".to_string());
        }
    }

    if needs_external_refresh {
        log_info!("[INFO] Refreshing external application themes...");

        let store = ThemeStore::global();
        let current_theme = store.current_theme();
        if let Ok(data) = store.get_theme_data(&current_theme) {
            let gtk_theme = if data.gtk.color_scheme == "prefer-dark" && data.gtk.theme == "Adwaita"
            {
                "Adwaita-dark".to_string()
            } else {
                data.gtk.theme.clone()
            };

            let _ = Command::new("hyprctl")
                .args([
                    "setcursor",
                    &data.cursor.theme,
                    &data.cursor.size.to_string(),
                ])
                .status();

            let _ = Command::new("gsettings")
                .args(["set", "org.gnome.desktop.interface", "gtk-theme", "Breeze"])
                .status();
            std::thread::sleep(std::time::Duration::from_millis(20));

            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "gtk-theme",
                    &gtk_theme,
                ])
                .status();

            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "icon-theme",
                    &data.gtk.icon_theme,
                ])
                .status();

            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    &data.cursor.theme,
                ])
                .status();

            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-size",
                    &data.cursor.size.to_string(),
                ])
                .status();

            let font_scale = if data.gtk.text_scaling_factor > 0.0 {
                data.gtk.text_scaling_factor
            } else {
                1.0
            };
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "text-scaling-factor",
                    &font_scale.to_string(),
                ])
                .status();

            let scale = if data.gtk.gdk_scale > 0 {
                data.gtk.gdk_scale
            } else {
                1
            };
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "scaling-factor",
                    &scale.to_string(),
                ])
                .status();

            let color_scheme = if data.gtk.color_scheme == "prefer-dark" {
                "prefer-dark"
            } else {
                "default"
            };
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "color-scheme",
                    color_scheme,
                ])
                .status();

            let _ = Command::new("systemctl")
                .args(["--user", "restart", "xdg-desktop-portal-gtk.service"])
                .status();
        }

        let refresh_script = get_hypr_path().join("bin/hypr-theme-refresh");
        let status = if refresh_script.exists() {
            Command::new(refresh_script).status()
        } else {
            Command::new("hypr-theme-refresh").status()
        };
        if status.is_ok() {
            reloaded.push("External App Themes".to_string());
        }
    }

    Ok(reloaded)
}
