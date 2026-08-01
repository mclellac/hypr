use crate::color_utils::ColorUtils;
use adw::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BorderWidget {
    pub title: String,
    pub theme_key: String,
    rows: Vec<gtk::Widget>,
    is_refreshing: Rc<RefCell<bool>>,

    color_button1: gtk::ColorDialogButton,
    gradient_switch: Option<adw::SwitchRow>,
    angle_row: Option<adw::SpinRow>,
    color_button2: Option<gtk::ColorDialogButton>,
}

impl BorderWidget {
    pub fn new(title: &str, theme_key: &str, allow_gradient: bool) -> Self {
        let is_refreshing = Rc::new(RefCell::new(false));
        let mut rows = Vec::new();

        // Primary Color Row
        let color_row1 = adw::ActionRow::builder().title(title).build();

        let dialog1 = gtk::ColorDialog::builder().with_alpha(true).build();
        let color_button1 = gtk::ColorDialogButton::builder()
            .dialog(&dialog1)
            .valign(gtk::Align::Center)
            .build();

        color_row1.add_suffix(&color_button1);
        rows.push(color_row1.upcast::<gtk::Widget>());

        let mut gradient_switch = None;
        let mut angle_row = None;
        let mut color_button2 = None;

        if allow_gradient {
            // Gradient Toggle
            let sw = adw::SwitchRow::builder()
                .title("Gradient Active")
                .subtitle("Enable gradient border")
                .build();
            rows.push(sw.clone().upcast::<gtk::Widget>());
            gradient_switch = Some(sw);

            // Angle
            let adj = gtk::Adjustment::new(45.0, 0.0, 360.0, 1.0, 10.0, 0.0);
            let ar = adw::SpinRow::builder()
                .title("Gradient Angle")
                .subtitle("Angle (0-360)")
                .adjustment(&adj)
                .build();
            rows.push(ar.clone().upcast::<gtk::Widget>());
            angle_row = Some(ar);

            // Secondary Color
            let color_row2 = adw::ActionRow::builder()
                .title("Gradient End Color")
                .build();
            let dialog2 = gtk::ColorDialog::builder().with_alpha(true).build();
            let cb2 = gtk::ColorDialogButton::builder()
                .dialog(&dialog2)
                .valign(gtk::Align::Center)
                .build();
            color_row2.add_suffix(&cb2);
            rows.push(color_row2.upcast::<gtk::Widget>());
            color_button2 = Some(cb2);
        }

        let widget = Self {
            title: title.to_string(),
            theme_key: theme_key.to_string(),
            rows,
            is_refreshing,
            color_button1,
            gradient_switch,
            angle_row,
            color_button2,
        };

        widget.setup_signals();
        widget.update_visibility();
        widget
    }

    pub fn get_rows(&self) -> &Vec<gtk::Widget> {
        &self.rows
    }

    fn setup_signals(&self) {
        let theme_key = self.theme_key.clone();
        let is_refreshing = self.is_refreshing.clone();

        let sw_clone = self.gradient_switch.clone();
        let ar_clone = self.angle_row.clone();
        let cb2_clone = self.color_button2.clone();

        let update_func = glib::clone!(
            #[weak(rename_to = cb1)]
            self.color_button1,
            #[strong(rename_to = sw)]
            sw_clone,
            #[strong(rename_to = ar)]
            ar_clone,
            #[strong(rename_to = cb2)]
            cb2_clone,
            #[strong]
            is_refreshing,
            #[strong]
            theme_key,
            move || {
                if *is_refreshing.borrow() {
                    return;
                }

                let c1 = ColorUtils::gdkrgba_to_rgba(&cb1.rgba());

                let val = if let (Some(sw), Some(ar), Some(cb2)) = (&sw, &ar, &cb2) {
                    if sw.is_active() {
                        let c2 = ColorUtils::gdkrgba_to_rgba(&cb2.rgba());
                        let angle = ar.value() as i32;
                        format!("{} {} {}deg", c1, c2, angle)
                    } else {
                        c1
                    }
                } else {
                    c1
                };

                ThemeStore::global().stage_update(&theme_key, yaml_serde::Value::String(val));
            }
        );

        self.color_button1.connect_rgba_notify(glib::clone!(
            #[strong]
            update_func,
            move |_| {
                update_func();
            }
        ));

        if let Some(sw) = &self.gradient_switch {
            sw.connect_active_notify(glib::clone!(
                #[strong]
                update_func,
                #[weak(rename_to = _cb1)]
                self.color_button1,
                #[strong(rename_to = ar)]
                ar_clone,
                #[strong(rename_to = cb2)]
                cb2_clone,
                move |sw| {
                    let is_gradient = sw.is_active();
                    if let Some(ar) = &ar {
                        ar.set_visible(is_gradient);
                    }
                    if let Some(cb2) = &cb2
                        && let Some(row) = cb2.parent().and_then(|p| p.parent())
                    {
                        row.set_visible(is_gradient);
                    }
                    update_func();
                }
            ));
        }

        if let Some(ar) = &self.angle_row {
            ar.connect_value_notify(glib::clone!(
                #[strong]
                update_func,
                move |_| {
                    update_func();
                }
            ));
        }

        if let Some(cb2) = &self.color_button2 {
            cb2.connect_rgba_notify(glib::clone!(
                #[strong]
                update_func,
                move |_| {
                    update_func();
                }
            ));
        }
    }

    pub fn update_visibility(&self) {
        if let Some(sw) = &self.gradient_switch {
            let is_gradient = sw.is_active();
            if let Some(ar) = &self.angle_row {
                ar.set_visible(is_gradient);
            }
            if let Some(cb2) = &self.color_button2
                && let Some(row) = cb2.parent().and_then(|p| p.parent())
            {
                row.set_visible(is_gradient);
            }
        }
    }

    pub fn set_value(&self, value: &str) {
        *self.is_refreshing.borrow_mut() = true;

        let parts = ColorUtils::split_hypr_color_string(value);
        let color_parts: Vec<String> = parts
            .iter()
            .filter(|p| ColorUtils::is_valid_color_string(p))
            .cloned()
            .collect();

        let angle_part = parts.iter().find(|p| p.ends_with("deg"));

        if let (Some(sw), Some(ar), Some(cb2)) =
            (&self.gradient_switch, &self.angle_row, &self.color_button2)
        {
            if color_parts.len() > 1 {
                sw.set_active(true);
                self.color_button1
                    .set_rgba(&ColorUtils::rgba_to_gdkrgba(&color_parts[0]));
                cb2.set_rgba(&ColorUtils::rgba_to_gdkrgba(&color_parts[1]));

                if let Some(ap) = angle_part {
                    if let Ok(deg) = ap.trim_end_matches("deg").parse::<f64>() {
                        ar.set_value(deg);
                    }
                } else {
                    ar.set_value(45.0);
                }
            } else {
                sw.set_active(false);
                if !color_parts.is_empty() {
                    self.color_button1
                        .set_rgba(&ColorUtils::rgba_to_gdkrgba(&color_parts[0]));
                }
            }
        } else {
            if !color_parts.is_empty() {
                self.color_button1
                    .set_rgba(&ColorUtils::rgba_to_gdkrgba(&color_parts[0]));
            }
        }

        self.update_visibility();
        *self.is_refreshing.borrow_mut() = false;
    }
}
