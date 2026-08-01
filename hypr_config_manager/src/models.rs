use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Using Option wrapper to allow skipping missing fields during deserialization.
// We provide default implementations for our models so we don't need serde default everywhere.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GtkConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub icon_theme: String,
    #[serde(default)]
    pub cursor_theme: String,
    #[serde(default)]
    pub cursor_size: i32,
    #[serde(default)]
    pub font_name: String,
    #[serde(default)]
    pub qt_theme: String,
    #[serde(default)]
    pub accent_color: String,
    #[serde(default)]
    pub color_scheme: String,
    #[serde(default)]
    pub gdk_scale: i32,
    #[serde(default)]
    pub gdk_dpi_scale: f32,
    #[serde(default)]
    pub qt_scale_factor: f32,
    #[serde(default)]
    pub text_scaling_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputConfig {
    #[serde(default)]
    pub kb_layout: String,
    #[serde(default)]
    pub kb_variant: String,
    #[serde(default)]
    pub kb_model: String,
    #[serde(default)]
    pub kb_options: String,
    #[serde(default)]
    pub kb_rules: String,
    #[serde(default)]
    pub follow_mouse: i32,
    #[serde(default)]
    pub sensitivity: f32,
    #[serde(default)]
    pub scroll_factor: f32,
    #[serde(default)]
    pub touchpad_scroll_factor: f32,
    #[serde(default)]
    pub touchpad_scroll_move: bool,
    #[serde(default)]
    pub accel_profile: String,
    #[serde(default)]
    pub touchpad_natural_scroll: bool,
    #[serde(default)]
    pub touchpad_tap_to_click: bool,
    #[serde(default = "default_true")]
    pub touchpad_enabled: bool,
    #[serde(default)]
    pub main_mod: String,
    #[serde(default)]
    pub solaar_enabled: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    #[serde(default)]
    pub gaps_in: i32,
    #[serde(default)]
    pub gaps_out: i32,
    #[serde(default)]
    pub border_size: i32,
    #[serde(alias = "col.active_border")]
    pub active_border: Option<String>,
    #[serde(alias = "col.inactive_border")]
    pub inactive_border: Option<String>,
    #[serde(default)]
    pub layout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub rounding: i32,
    #[serde(default)]
    pub gaps_in: i32,
    #[serde(default)]
    pub gaps_out: i32,
    #[serde(default)]
    pub border_size: i32,
    #[serde(default)]
    pub active_border: String,
    #[serde(default)]
    pub inactive_border: String,
    #[serde(default)]
    pub nogroup_border: String,
    #[serde(default)]
    pub nogroup_border_active: String,
    #[serde(default)]
    pub allow_tearing: bool,
    #[serde(default)]
    pub shadow_enabled: bool,
    #[serde(default)]
    pub shadow_range: i32,
    #[serde(default)]
    pub shadow_power: i32,
    #[serde(default)]
    pub shadow_color: String,
    #[serde(default)]
    pub shadow_offset: String,
    #[serde(default)]
    pub blur_enabled: bool,
    #[serde(default)]
    pub blur_size: i32,
    #[serde(default)]
    pub blur_passes: i32,
    #[serde(default)]
    pub blur_vibrancy: f32,
    #[serde(default)]
    pub master_mfact: Option<f32>,
    #[serde(default)]
    pub master_new_status: Option<String>,
    #[serde(default)]
    pub master_orientation: Option<String>,
    #[serde(default)]
    pub master_new_on_top: Option<bool>,
    #[serde(default)]
    pub master_new_on_active: Option<bool>,
    #[serde(default)]
    pub dim_inactive: bool,
    #[serde(default)]
    pub dim_strength: f32,
    #[serde(default)]
    pub glow_enabled: bool,
    #[serde(default)]
    pub glow_range: i32,
    #[serde(default)]
    pub glow_color: String,
    #[serde(default)]
    pub animation_style: String,
    #[serde(default)]
    pub animation_windows_curve: String,
    #[serde(default)]
    pub animation_windows_speed: i32,
    #[serde(default)]
    pub animation_fade_speed: i32,
    #[serde(default)]
    pub animation_workspaces_speed: i32,
    #[serde(default)]
    pub animation_windows_enabled: bool,
    #[serde(default)]
    pub animation_windows_out_enabled: bool,
    #[serde(default)]
    pub animation_workspaces_enabled: bool,
    #[serde(default)]
    pub animation_border_enabled: bool,
    #[serde(default)]
    pub animation_fade_enabled: bool,
    #[serde(default)]
    pub animations_enabled: bool,
    #[serde(default)]
    pub smart_gaps: Option<i32>,
    #[serde(default)]
    pub split_width_multiplier: Option<f32>,
    #[serde(default)]
    pub preserve_split: Option<bool>,
    #[serde(default)]
    pub smart_split: Option<bool>,
    #[serde(default)]
    pub force_split: Option<i32>,
    #[serde(default)]
    pub precise_mouse_move: Option<bool>,
    #[serde(default)]
    pub scrolling_column_width: Option<f32>,
    #[serde(default)]
    pub scrolling_direction: Option<String>,
    #[serde(default)]
    pub scrolling_focus_fit_method: Option<i32>,
    #[serde(default)]
    pub scrolling_follow_focus: Option<bool>,
    #[serde(default)]
    pub scrolling_follow_min_visible: Option<f32>,
    #[serde(default)]
    pub scrolling_fullscreen_on_one_column: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecorationConfig {
    #[serde(default)]
    pub rounding: i32,
    #[serde(default)]
    pub drop_shadow: bool,
    #[serde(default)]
    pub shadow_range: i32,
    #[serde(default)]
    pub shadow_render_power: i32,
    #[serde(alias = "col.shadow")]
    pub shadow_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FontsConfig {
    #[serde(default)]
    pub font_family: String,
    #[serde(default)]
    pub font_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub size: i32,
    pub no_hardware: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpacityConfig {
    #[serde(default)]
    pub active: f32,
    #[serde(default)]
    pub inactive: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowSizeConfig {
    #[serde(default)]
    pub creator: String,
    #[serde(default)]
    pub dialog: String,
    #[serde(default)]
    pub editor: String,
    #[serde(default)]
    pub floating: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceAssignmentConfig {
    #[serde(default)]
    pub browsers: bool,
    #[serde(default)]
    pub coding: bool,
    #[serde(default)]
    pub communications: bool,
    #[serde(default)]
    pub media: bool,
    #[serde(default)]
    pub notes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowAnimationConfig {
    #[serde(default)]
    pub browsers: String,
    #[serde(default)]
    pub coding: String,
    #[serde(default)]
    pub communications: String,
    #[serde(default)]
    pub media: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprWindowConfig {
    #[serde(default)]
    pub opacity: OpacityConfig,
    #[serde(default)]
    pub size: WindowSizeConfig,
    #[serde(default)]
    pub workspace_assignment: WorkspaceAssignmentConfig,
    #[serde(default)]
    pub animation: WindowAnimationConfig,
    #[serde(default)]
    pub suppress_maximize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprIdleConfig {
    #[serde(default)]
    pub timeout: i32,
    #[serde(default)]
    pub lock_timeout: i32,
    #[serde(default)]
    pub dpms_timeout: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprSunsetConfig {
    #[serde(default = "default_6000")]
    pub temperature: i32,
}

fn default_6000() -> i32 {
    6000
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprConfig {
    #[serde(default)]
    pub window: HyprWindowConfig,
    #[serde(default)]
    pub idle: HyprIdleConfig,
    #[serde(default)]
    pub sunset: HyprSunsetConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Keybinding {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub mods: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub dispatcher: String,
    #[serde(default)]
    pub arg: String,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorConfig {
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub scale: f32,
    #[serde(default)]
    pub bitdepth: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprglassConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_hyprglass_repo")]
    pub repository: String,
    #[serde(default = "default_glass_opacity")]
    pub glass_opacity: f32,
    #[serde(default = "default_blur_strength")]
    pub blur_strength: f32,
    #[serde(default = "default_blur_iterations")]
    pub blur_iterations: i32,
    #[serde(default = "default_refraction_strength")]
    pub refraction_strength: f32,
    #[serde(default = "default_chromatic_aberration")]
    pub chromatic_aberration: f32,
    #[serde(default = "default_fresnel_strength")]
    pub fresnel_strength: f32,
    #[serde(default = "default_specular_strength")]
    pub specular_strength: f32,
    #[serde(default = "default_edge_thickness")]
    pub edge_thickness: f32,
    #[serde(default = "default_tint_color")]
    pub tint_color: String,
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    #[serde(default = "default_contrast")]
    pub contrast: f32,
    #[serde(default = "default_true")]
    pub manage_window_blur: bool,
    #[serde(default = "default_true")]
    pub layer_waybar: bool,
    #[serde(default = "default_true")]
    pub layer_walker: bool,
    #[serde(default = "default_true")]
    pub layer_mako: bool,
}

fn default_hyprglass_repo() -> String {
    "https://github.com/hyprnux/hyprglass".to_string()
}
fn default_glass_opacity() -> f32 { 0.85 }
fn default_blur_strength() -> f32 { 2.0 }
fn default_blur_iterations() -> i32 { 3 }
fn default_refraction_strength() -> f32 { 0.6 }
fn default_chromatic_aberration() -> f32 { 0.5 }
fn default_fresnel_strength() -> f32 { 0.6 }
fn default_specular_strength() -> f32 { 0.8 }
fn default_edge_thickness() -> f32 { 0.06 }
fn default_tint_color() -> String { "0x8899aa22".to_string() }
fn default_brightness() -> f32 { 0.9 }
fn default_contrast() -> f32 { 1.0 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginsConfig {
    #[serde(default)]
    pub hyprglass: HyprglassConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub gtk: GtkConfig,
    #[serde(default)]
    pub cursor: CursorConfig,
    #[serde(default)]
    pub input: InputConfig,
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub decoration: DecorationConfig,
    #[serde(default)]
    pub animations: AnimationConfig,
    #[serde(default)]
    pub layout: LayoutConfig,
    #[serde(default)]
    pub fonts: FontsConfig,
    #[serde(default)]
    pub hypr: HyprConfig,
    #[serde(default)]
    pub plugins: PluginsConfig,

    #[serde(default)]
    pub colors: HashMap<String, String>,
    #[serde(default)]
    pub keybindings: Vec<Keybinding>,
    #[serde(default)]
    pub monitors: Vec<MonitorConfig>,

    #[serde(flatten)]
    pub raw: HashMap<String, yaml_serde::Value>,
    #[serde(skip)]
    pub raw_value: yaml_serde::Value,
}

impl ThemeConfig {
    pub fn get_raw_value(&self, path: &str) -> Option<&yaml_serde::Value> {
        let mut current = &self.raw_value;
        for part in path.split(".") {
            current = current.get(part)?;
        }
        Some(current)
    }

    pub fn get_raw_i32(&self, path: &str, default: i32) -> i32 {
        self.get_raw_value(path)
            .map(|v| {
                if let Some(i) = v.as_i64() {
                    i as i32
                } else if let Some(f) = v.as_f64() {
                    f as i32
                } else if let Some(b) = v.as_bool() {
                    if b { 1 } else { 0 }
                } else if let Some(s) = v.as_str() {
                    s.parse::<i32>().unwrap_or(default)
                } else {
                    default
                }
            })
            .unwrap_or(default)
    }

    pub fn get_raw_f64(&self, path: &str, default: f64) -> f64 {
        self.get_raw_value(path)
            .map(|v| {
                if let Some(f) = v.as_f64() {
                    f
                } else if let Some(i) = v.as_i64() {
                    i as f64
                } else if let Some(b) = v.as_bool() {
                    if b { 1.0 } else { 0.0 }
                } else if let Some(s) = v.as_str() {
                    s.parse::<f64>().unwrap_or(default)
                } else {
                    default
                }
            })
            .unwrap_or(default)
    }

    pub fn get_raw_str(&self, path: &str, default: &str) -> String {
        self.get_raw_value(path)
            .map(|v| {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else if let Some(i) = v.as_i64() {
                    i.to_string()
                } else if let Some(f) = v.as_f64() {
                    f.to_string()
                } else if let Some(b) = v.as_bool() {
                    b.to_string()
                } else {
                    default.to_string()
                }
            })
            .unwrap_or_else(|| default.to_string())
    }

    pub fn get_raw_bool(&self, path: &str, default: bool) -> bool {
        self.get_raw_value(path)
            .map(|v| {
                if let Some(b) = v.as_bool() {
                    b
                } else if let Some(i) = v.as_i64() {
                    i != 0
                } else if let Some(f) = v.as_f64() {
                    f != 0.0
                } else if let Some(s) = v.as_str() {
                    s == "true" || s == "1" || s == "yes" || s == "on"
                } else {
                    default
                }
            })
            .unwrap_or(default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_monitors() {
        let yaml = r#"
name: test theme
monitors:
  - output: HDMI-A-1
    mode: 1920x1080@60
    position: 0x0
    scale: 1.5
"#;
        let config: ThemeConfig = yaml_serde::from_str(yaml).unwrap();
        assert_eq!(config.monitors.len(), 1);
        assert_eq!(config.monitors[0].output, "HDMI-A-1");
        assert_eq!(config.monitors[0].mode, "1920x1080@60");
        assert_eq!(config.monitors[0].position, "0x0");
        assert_eq!(config.monitors[0].scale, 1.5);
    }

    #[test]
    fn test_get_raw_f64_integer() {
        let yaml = r#"
name: test theme
ghostty:
  font_size: 12
  opacity: 0.95
"#;
        let config_value: yaml_serde::Value = yaml_serde::from_str(yaml).unwrap();
        let mut config: ThemeConfig = yaml_serde::from_value(config_value.clone()).unwrap();
        config.raw_value = config_value;
        assert_eq!(config.get_raw_f64("ghostty.font_size", 0.0), 12.0);
        assert_eq!(config.get_raw_f64("ghostty.opacity", 0.0), 0.95);
    }
}
