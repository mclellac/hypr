use crate::color_utils::ColorUtils;
use crate::pages::PageSettings;
use crate::pages::components::border_widget::BorderWidget;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct LayoutPage(ObjectSubclass<imp::LayoutPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl LayoutPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn debug_log(&self, msg: &str) {
        if std::env::args().any(|x| x == "--debug") {
            println!("[LayoutPage] {}", msg);
        }
    }
}

impl PageSettings for LayoutPage {
    fn setup_ui(&self) {
        let imp = self.imp();
        self.debug_log("Setting up UI models and signals");

        // Connect Signals
        self.connect_switch_signal("allow_tearing_row", "layout.allow_tearing");
        self.connect_combo_signal("layout_name_row", "layout.name");
        self.connect_spin_signal(
            "single_window_aspect_ratio_row",
            "layout.single_window_aspect_ratio",
        );
        self.connect_spin_signal(
            "single_window_aspect_ratio_tolerance_row",
            "layout.single_window_aspect_ratio_tolerance",
        );

        // Workspace Layouts
        for i in 1..=10 {
            let id = format!("ws_layout_{}", i);
            let key = format!("layout.workspace_layouts.{}", i);
            self.connect_combo_signal(&id, &key);
        }

        // Dwindle
        self.connect_switch_signal("preserve_split_row", "layout.dwindle.preserve_split");
        self.connect_switch_signal("smart_split_row", "layout.dwindle.smart_split");
        self.connect_switch_signal("precise_mouse_move_row", "layout.precise_mouse_move");

        // Master
        self.connect_combo_signal("new_status_row", "layout.master_new_status");
        self.connect_spin_signal("mfact_row", "layout.master_mfact");
        self.connect_combo_signal("orientation_row", "layout.master_orientation");
        self.connect_switch_signal("new_on_top_row", "layout.master_new_on_top");
        self.connect_combo_signal("new_on_active_row", "layout.master_new_on_active");

        // Groupbar
        self.connect_switch_signal("groupbar_enabled_row", "layout.groupbar_enabled");
        self.connect_switch_signal("groupbar_gradients_row", "layout.groupbar_gradients");
        self.connect_spin_signal("groupbar_font_size_row", "layout.groupbar_font_size");
        self.connect_spin_signal("groupbar_height_row", "layout.groupbar_height");
        self.connect_switch_signal("groupbar_stacked_row", "layout.groupbar_stacked");
        self.connect_switch_signal(
            "groupbar_render_titles_row",
            "layout.groupbar_render_titles",
        );
        self.connect_switch_signal("groupbar_scrolling_row", "layout.groupbar_scrolling");

        // Groupbar Colors
        if let Some(group) = imp
            .builder
            .object::<adw::PreferencesGroup>("groupbar_group")
        {
            self.create_and_bind_border_widget(
                &group,
                "Active Group Color",
                "layout.groupbar_col_active",
                true,
            );
            self.create_and_bind_border_widget(
                &group,
                "Inactive Group Color",
                "layout.groupbar_col_inactive",
                true,
            );
            self.create_and_bind_border_widget(
                &group,
                "Locked Active Group Color",
                "layout.groupbar_col_locked_active",
                true,
            );
            self.create_and_bind_border_widget(
                &group,
                "Locked Inactive Group Color",
                "layout.groupbar_col_locked_inactive",
                true,
            );

            // Text color button
            if let Some(btn) = imp
                .builder
                .object::<gtk::ColorDialogButton>("groupbar_text_color_button")
            {
                let dialog = gtk::ColorDialog::builder().with_alpha(true).build();
                btn.set_dialog(&dialog);

                btn.connect_rgba_notify(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |btn| {
                        if *obj.imp().is_refreshing.borrow() {
                            return;
                        }
                        let val = ColorUtils::gdkrgba_to_rgba(&btn.rgba());
                        ThemeStore::global().stage_update(
                            "layout.groupbar_text_color",
                            yaml_serde::Value::String(val),
                        );
                    }
                ));
            }
        }

        // Scrolling
        self.connect_switch_signal(
            "scrolling_fullscreen_on_one_column_row",
            "layout.scrolling_fullscreen_on_one_column",
        );
        self.connect_spin_signal(
            "scrolling_column_width_row",
            "layout.scrolling_column_width",
        );
        self.connect_combo_signal(
            "scrolling_focus_fit_method_row",
            "layout.scrolling_focus_fit_method",
        );
        self.connect_switch_signal(
            "scrolling_follow_focus_row",
            "layout.scrolling_follow_focus",
        );
        self.connect_spin_signal(
            "scrolling_follow_min_visible_row",
            "layout.scrolling_follow_min_visible",
        );
        self.connect_combo_signal("scrolling_direction_row", "layout.scrolling_direction");

        self.refresh_settings();
    }

    fn refresh_settings(&self) {
        let imp = self.imp();
        *imp.is_refreshing.borrow_mut() = true;

        let store = ThemeStore::global();
        let theme = store.current_theme();
        let data = store
            .get_theme_data(&theme)
            .unwrap_or_else(|_| std::sync::Arc::new(hypr_config_manager::ThemeConfig::default()));

        self.debug_log(&format!("Refreshing settings for theme: {}", theme));

        self.update_switch_row(
            "allow_tearing_row",
            data.get_raw_bool("layout.allow_tearing", false),
        );
        self.update_combo_row(
            "layout_name_row",
            &data.get_raw_str("layout.name", "dwindle"),
        );
        self.update_spin_row(
            "single_window_aspect_ratio_row",
            data.get_raw_f64("layout.single_window_aspect_ratio", 0.0),
        );
        self.update_spin_row(
            "single_window_aspect_ratio_tolerance_row",
            data.get_raw_f64("layout.single_window_aspect_ratio_tolerance", 0.0),
        );

        for i in 1..=10 {
            let id = format!("ws_layout_{}", i);
            let key = format!("layout.workspace_layouts.{}", i);
            self.update_combo_row(&id, &data.get_raw_str(&key, "none"));
        }

        // Dwindle
        self.update_switch_row(
            "preserve_split_row",
            data.get_raw_bool("layout.dwindle.preserve_split", false),
        );
        self.update_switch_row(
            "smart_split_row",
            data.get_raw_bool("layout.dwindle.smart_split", false),
        );
        self.update_switch_row(
            "precise_mouse_move_row",
            data.get_raw_bool("layout.precise_mouse_move", true),
        );

        // Master
        self.update_combo_row(
            "new_status_row",
            &data.get_raw_str("layout.master_new_status", "master"),
        );
        self.update_spin_row("mfact_row", data.get_raw_f64("layout.master_mfact", 0.55));
        self.update_combo_row(
            "orientation_row",
            &data.get_raw_str("layout.master_orientation", "left"),
        );
        self.update_switch_row(
            "new_on_top_row",
            data.get_raw_bool("layout.master_new_on_top", false),
        );
        self.update_combo_row(
            "new_on_active_row",
            &data.get_raw_str("layout.master_new_on_active", "none"),
        );

        // Groupbar
        self.update_switch_row(
            "groupbar_enabled_row",
            data.get_raw_bool("layout.groupbar_enabled", true),
        );
        self.update_switch_row(
            "groupbar_gradients_row",
            data.get_raw_bool("layout.groupbar_gradients", true),
        );
        self.update_spin_row(
            "groupbar_font_size_row",
            data.get_raw_f64("layout.groupbar_font_size", 10.0),
        );
        self.update_spin_row(
            "groupbar_height_row",
            data.get_raw_f64("layout.groupbar_height", 14.0),
        );
        self.update_switch_row(
            "groupbar_stacked_row",
            data.get_raw_bool("layout.groupbar_stacked", false),
        );
        self.update_switch_row(
            "groupbar_render_titles_row",
            data.get_raw_bool("layout.groupbar_render_titles", true),
        );
        self.update_switch_row(
            "groupbar_scrolling_row",
            data.get_raw_bool("layout.groupbar_scrolling", true),
        );

        // Groupbar Colors
        if let Some(btn) = imp
            .builder
            .object::<gtk::ColorDialogButton>("groupbar_text_color_button")
        {
            btn.set_rgba(&ColorUtils::rgba_to_gdkrgba(
                &data.get_raw_str("layout.groupbar_text_color", "#ffffff"),
            ));
        }

        for bw in imp.border_widgets.borrow().iter() {
            bw.set_value(&data.get_raw_str(&bw.theme_key, ""));
        }

        // Scrolling
        self.update_switch_row(
            "scrolling_fullscreen_on_one_column_row",
            data.get_raw_bool("layout.scrolling_fullscreen_on_one_column", true),
        );
        self.update_spin_row(
            "scrolling_column_width_row",
            data.get_raw_f64("layout.scrolling_column_width", 0.5),
        );
        self.update_combo_row(
            "scrolling_focus_fit_method_row",
            &data.get_raw_str("layout.scrolling_focus_fit_method", "1"),
        );
        self.update_switch_row(
            "scrolling_follow_focus_row",
            data.get_raw_bool("layout.scrolling_follow_focus", true),
        );
        self.update_spin_row(
            "scrolling_follow_min_visible_row",
            data.get_raw_f64("layout.scrolling_follow_min_visible", 0.4),
        );
        self.update_combo_row(
            "scrolling_direction_row",
            &data.get_raw_str("layout.scrolling_direction", "right"),
        );

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl LayoutPage {
    fn create_and_bind_border_widget(
        &self,
        group: &adw::PreferencesGroup,
        title: &str,
        theme_key: &str,
        allow_gradient: bool,
    ) {
        let bw = Rc::new(BorderWidget::new(title, theme_key, allow_gradient));
        for row in bw.get_rows() {
            group.add(row);
        }
        self.imp().border_widgets.borrow_mut().push(bw);
    }

    fn connect_spin_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(self, imp.builder, adw::SpinRow, id, theme_key);
    }

    fn connect_switch_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(
            self,
            imp.builder,
            adw::SwitchRow,
            id,
            theme_key,
            connect_active_notify,
            |w: &adw::SwitchRow| w.is_active(),
            yaml_serde::Value::Bool
        );
    }

    fn connect_combo_signal(&self, id: &str, theme_key: &str) {
        let theme_key = theme_key.to_string();
        let imp = self.imp();
        if let Some(row) = imp.builder.object::<adw::ComboRow>(id) {
            row.connect_selected_item_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    let imp = obj.imp();
                    if *imp.is_refreshing.borrow() {
                        return;
                    }
                    if let Some(item) = row.selected_item()
                        && let Some(string_obj) = item.downcast_ref::<gtk::StringObject>()
                    {
                        let val = string_obj.string().to_string();
                        ThemeStore::global()
                            .stage_update(&theme_key, yaml_serde::Value::String(val));
                    }
                }
            ));
        }
    }

    fn update_combo_row(&self, id: &str, val: &str) {
        if let Some(row) = self.imp().builder.object::<adw::ComboRow>(id)
            && let Some(model) = row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                if string_list.string(i).unwrap() == val {
                    row.set_selected(i);
                    return;
                }
            }
        }
    }

    fn update_spin_row(&self, id: &str, val: f64) {
        if let Some(row) = self.imp().builder.object::<adw::SpinRow>(id) {
            row.set_value(val);
        }
    }

    fn update_switch_row(&self, id: &str, val: bool) {
        if let Some(row) = self.imp().builder.object::<adw::SwitchRow>(id) {
            row.set_active(val);
        }
    }
}

pub mod imp {
    use super::*;

    pub struct LayoutPage {
        pub builder: gtk::Builder,
        pub is_refreshing: Rc<RefCell<bool>>,
        pub border_widgets: RefCell<Vec<Rc<BorderWidget>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LayoutPage {
        const NAME: &'static str = "LayoutPage";
        type Type = super::LayoutPage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/layout.ui"));
            Self {
                builder,
                is_refreshing: Rc::new(RefCell::new(false)),
                border_widgets: RefCell::new(Vec::new()),
            }
        }
    }

    impl Default for LayoutPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for LayoutPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = [
                "general_layout_group",
                "dwindle_group",
                "master_group",
                "groupbar_group",
                "scrolling_group",
            ];

            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for LayoutPage {}
    impl PreferencesPageImpl for LayoutPage {}
}
