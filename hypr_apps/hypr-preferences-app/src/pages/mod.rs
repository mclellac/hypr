pub mod appearance;
pub mod applications;
pub mod backup;
pub mod components;
pub mod displays;
pub mod hardware;
pub mod input;
pub mod keybindings;
pub mod layout;
pub mod plugins;
pub mod system;
pub mod theme;
pub mod wallpapers;
pub mod windows;

use adw::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

pub trait PageSettings: IsA<gtk::Widget> {
    fn setup_ui(&self);
    fn refresh_settings(&self);
}

#[macro_export]
macro_rules! bind_signal {
    ($obj:ident, $builder:expr, $widget_type:ty, $id:expr, $theme_key:expr, $signal:ident, $val_expr:expr, $store_type:path) => {
        let theme_key = $theme_key.to_string();
        if let Some(widget) = $builder.object::<$widget_type>($id) {
            widget.$signal(glib::clone!(
                #[weak(rename_to = obj)]
                $obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = ($val_expr)(w);
                    hypr_config_manager::ThemeStore::global()
                        .stage_update(&theme_key, $store_type(val.into()));
                }
            ));
        }
    };
    // specifically for numeric conversions where f64/i32 distinction is needed
    ($obj:ident, $builder:expr, adw::SpinRow, $id:expr, $theme_key:expr) => {
        let theme_key = $theme_key.to_string();
        if let Some(widget) = $builder.object::<adw::SpinRow>($id) {
            widget.connect_value_notify(glib::clone!(
                #[weak(rename_to = obj)]
                $obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    let val_int = val as i32;
                    if (val - val_int as f64).abs() < f64::EPSILON {
                        hypr_config_manager::ThemeStore::global()
                            .stage_update(&theme_key, yaml_serde::Value::Number(val_int.into()));
                    } else {
                        hypr_config_manager::ThemeStore::global()
                            .stage_update(&theme_key, yaml_serde::Value::from(val));
                    }
                }
            ));
        }
    };
}
