use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::models::Keybinding;
use hypr_config_manager::{StoreEvent, ThemeStore};
use libadwaita as adw;
use serde::Deserialize;
use std::cell::RefCell;

#[derive(Deserialize, Debug, Clone)]
pub struct SystemBind {
    modmask: u32,
    key: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    dispatcher: String,
    #[serde(default)]
    arg: String,
}

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::KeybindingsEditorWindow)]
    pub struct KeybindingsEditorWindow {
        pub toast_overlay: adw::ToastOverlay,
        pub toolbar_view: adw::ToolbarView,
        pub stack: gtk::Stack,
        pub columns_box: gtk::Box,
        pub search_entry: gtk::SearchEntry,
        pub save_btn: gtk::Button,
        pub add_btn: gtk::Button,

        #[property(get, set)]
        pub n_columns: RefCell<i32>,

        pub all_bindings: RefCell<Vec<Keybinding>>,
        pub columns: RefCell<Vec<gtk::Box>>,
        pub active_system_binds: RefCell<Vec<SystemBind>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KeybindingsEditorWindow {
        const NAME: &'static str = "KeybindingsEditorWindow";
        type Type = super::KeybindingsEditorWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for KeybindingsEditorWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_title(Some("Keybindings Editor"));
            obj.set_default_width(1100);
            obj.set_default_height(700);

            // Toast Overlay
            obj.set_content(Some(&self.toast_overlay));
            self.toast_overlay.set_child(Some(&self.toolbar_view));

            // Header Bar
            let header_bar = adw::HeaderBar::new();
            let window_title = adw::WindowTitle::new("Keybindings Editor", "");
            header_bar.set_title_widget(Some(&window_title));

            self.add_btn.set_icon_name("list-add-symbolic");
            self.add_btn.set_tooltip_text(Some("Add New Keybinding"));
            header_bar.pack_start(&self.add_btn);

            self.search_entry
                .set_placeholder_text(Some("Search keybindings..."));
            self.search_entry.set_max_width_chars(30);
            header_bar.pack_start(&self.search_entry);

            self.save_btn.set_label("Save and Apply");
            self.save_btn.add_css_class("suggested-action");
            self.save_btn.set_sensitive(false);
            header_bar.pack_end(&self.save_btn);

            self.toolbar_view.add_top_bar(&header_bar);

            // Content
            let scrolled = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .vexpand(true)
                .build();

            let clamp = adw::Clamp::builder()
                .maximum_size(3840)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(12)
                .margin_end(12)
                .build();

            self.columns_box
                .set_orientation(gtk::Orientation::Horizontal);
            self.columns_box.set_spacing(40);
            self.columns_box.set_homogeneous(true);
            self.columns_box.set_valign(gtk::Align::Start);

            clamp.set_child(Some(&self.columns_box));
            scrolled.set_child(Some(&clamp));

            self.stack.add_named(&scrolled, Some("list"));

            let empty_state = adw::StatusPage::builder()
                .icon_name("system-search-symbolic")
                .title("No Results Found")
                .description("Try a different search term")
                .build();
            self.stack.add_named(&empty_state, Some("empty"));

            self.toolbar_view.set_content(Some(&self.stack));

            *self.n_columns.borrow_mut() = 3;
            obj.setup_breakpoints();

            // Connect to property change to trigger reflow
            obj.connect_n_columns_notify(move |win| {
                win.reflow_groups();
            });
        }

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for KeybindingsEditorWindow {}
    impl WindowImpl for KeybindingsEditorWindow {}
    impl ApplicationWindowImpl for KeybindingsEditorWindow {}
    impl AdwApplicationWindowImpl for KeybindingsEditorWindow {}
}

glib::wrapper! {
    pub struct KeybindingsEditorWindow(ObjectSubclass<imp::KeybindingsEditorWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl KeybindingsEditorWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window: Self = glib::Object::builder().property("application", app).build();

        window.connect_close_request(glib::clone!(
            #[weak]
            app,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_| {
                app.quit();
                glib::Propagation::Proceed
            }
        ));
        window
    }

    fn setup_breakpoints(&self) {
        // Responsiveness
        let bp_1300 = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
            adw::BreakpointConditionLengthType::MaxWidth,
            1300.0,
            adw::LengthUnit::Px,
        ));
        bp_1300.add_setter(self, "n-columns", Some(&2.to_value()));
        self.add_breakpoint(bp_1300);

        let bp_900 = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
            adw::BreakpointConditionLengthType::MaxWidth,
            900.0,
            adw::LengthUnit::Px,
        ));
        bp_900.add_setter(self, "n-columns", Some(&1.to_value()));
        self.add_breakpoint(bp_900);

        let bp_1600 = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
            adw::BreakpointConditionLengthType::MinWidth,
            1600.0,
            adw::LengthUnit::Px,
        ));
        bp_1600.add_setter(self, "n-columns", Some(&4.to_value()));
        self.add_breakpoint(bp_1600);
    }

    pub fn setup_ui(&self) {
        let imp = self.imp();

        imp.save_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                obj.on_save_clicked();
            }
        ));

        imp.add_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                obj.on_add_clicked();
            }
        ));

        imp.search_entry.connect_search_changed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |entry| {
                obj.on_search_changed(entry.text().to_string());
            }
        ));

        // ThemeStore listener
        let (sender, receiver) = std::sync::mpsc::channel::<StoreEvent>();
        let save_btn_weak = imp.save_btn.downgrade();

        let obj_weak = self.downgrade();
        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            let _obj = match obj_weak.upgrade() {
                Some(o) => o,
                None => return glib::ControlFlow::Break,
            };
            while let Ok(event) = receiver.try_recv() {
                if let Some(btn) = save_btn_weak.upgrade() {
                    let has_changes = ThemeStore::global().has_staged_updates();
                    btn.set_sensitive(has_changes);
                }
                if event == StoreEvent::ThemeChanged {
                    // Refresh if needed
                }
            }
            glib::ControlFlow::Continue
        });

        ThemeStore::global().add_listener(move |event| {
            let _ = sender.send(event);
        });

        self.load_bindings();
    }

    fn load_active_system_binds(&self) {
        let output = std::process::Command::new("hyprctl")
            .args(["-j", "binds"])
            .output();

        let mut list = Vec::new();
        if let Ok(out) = output
            && out.status.success()
            && let Ok(parsed) = serde_json::from_slice::<Vec<SystemBind>>(&out.stdout)
        {
            list = parsed;
        }
        *self.imp().active_system_binds.borrow_mut() = list;
    }

    fn load_bindings(&self) {
        self.load_active_system_binds();
        let store = ThemeStore::global();
        let theme = store.current_theme();
        if let Ok(data) = store.get_theme_data(&theme) {
            *self.imp().all_bindings.borrow_mut() = data.keybindings.clone();
            self.reflow_groups();
        }
    }

    fn reflow_groups(&self) {
        let imp = self.imp();
        let n = *imp.n_columns.borrow();

        // Clear columns
        while let Some(child) = imp.columns_box.first_child() {
            imp.columns_box.remove(&child);
        }

        let mut columns = Vec::new();
        for _ in 0..n {
            let col = gtk::Box::new(gtk::Orientation::Vertical, 24);
            col.set_hexpand(true);
            imp.columns_box.append(&col);
            columns.push(col);
        }

        // Group by category
        let mut categorized: std::collections::BTreeMap<String, Vec<Keybinding>> =
            std::collections::BTreeMap::new();
        for b in imp.all_bindings.borrow().iter() {
            let cat = if b.category.is_empty() {
                "General".to_string()
            } else {
                b.category.clone()
            };
            categorized.entry(cat).or_default().push(b.clone());
        }

        // Simple distribution (Greedy)
        let mut col_heights = vec![0; n as usize];
        for (cat, bindings) in categorized {
            let shortest_idx = col_heights
                .iter()
                .enumerate()
                .min_by_key(|&(_, &h)| h)
                .unwrap()
                .0;

            let group = adw::PreferencesGroup::new();
            group.set_title(&glib::markup_escape_text(&cat));
            group.set_valign(gtk::Align::Start);

            for b in &bindings {
                let row = self.create_binding_row(b);
                group.add(&row);
            }

            columns[shortest_idx].append(&group);
            col_heights[shortest_idx] += 1 + bindings.len();
        }

        *imp.columns.borrow_mut() = columns;
    }

    fn normalize_modmask(&self, modmask: u32) -> String {
        let mut parts = Vec::new();
        if modmask & 1 != 0 {
            parts.push("SHIFT".to_string());
        }
        if modmask & 4 != 0 {
            parts.push("CTRL".to_string());
        }
        if modmask & 8 != 0 {
            parts.push("ALT".to_string());
        }
        if modmask & 64 != 0 {
            parts.push("SUPER".to_string());
        }
        parts.sort();
        parts.join("+")
    }

    fn normalize_system_shortcut(&self, modmask: u32, key: &str) -> String {
        let mods = self.normalize_modmask(modmask);
        let normalized_key = key.trim().to_uppercase();
        if mods.is_empty() {
            normalized_key
        } else {
            format!("{}+{}", mods, normalized_key)
        }
    }

    fn get_system_conflict(&self, b: &Keybinding) -> Option<String> {
        let shortcut = normalize_shortcut(&b.mods, &b.key);
        if b.key.trim().is_empty() {
            return None;
        }

        // Check conflicts in the active system binds
        let active_binds = self.imp().active_system_binds.borrow();
        for sys in active_binds.iter() {
            let sys_shortcut = self.normalize_system_shortcut(sys.modmask, &sys.key);
            if sys_shortcut == shortcut {
                // If the description or dispatcher/arg differs, it's a conflict
                let same_action = (sys.description.trim().to_lowercase()
                    == b.desc.trim().to_lowercase()
                    && !b.desc.is_empty())
                    || (sys.dispatcher.trim() == b.dispatcher.trim()
                        && sys.arg.trim() == b.arg.trim());
                if !same_action {
                    let conflict_desc = if !sys.description.is_empty() {
                        sys.description.clone()
                    } else {
                        format!("{} {}", sys.dispatcher, sys.arg)
                    };
                    return Some(conflict_desc);
                }
            }
        }

        // Check internal conflicts in the edited list (duplicate shortcuts)
        let all_bindings = self.imp().all_bindings.borrow();
        let mut count = 0;
        let mut conflicting_desc = String::new();
        for other in all_bindings.iter() {
            if normalize_shortcut(&other.mods, &other.key) == shortcut {
                count += 1;
                if other.desc != b.desc || other.dispatcher != b.dispatcher || other.arg != b.arg {
                    conflicting_desc = other.desc.clone();
                }
            }
        }
        if count > 1 {
            return Some(if conflicting_desc.is_empty() {
                "Duplicate binding".to_string()
            } else {
                conflicting_desc
            });
        }

        None
    }

    fn create_binding_row(&self, b: &Keybinding) -> adw::ActionRow {
        let title = if b.desc.is_empty() {
            format!("{} {}", b.dispatcher, b.arg).trim().to_string()
        } else {
            b.desc.clone()
        };

        let row = adw::ActionRow::builder()
            .title(glib::markup_escape_text(&title))
            .activatable(true)
            .build();

        if let Some(conflict_msg) = self.get_system_conflict(b) {
            let warning_icon = gtk::Image::builder()
                .icon_name("dialog-warning-symbolic")
                .tooltip_text(format!("Conflict: {}", conflict_msg))
                .build();
            warning_icon.add_css_class("warning");
            row.add_prefix(&warning_icon);
            row.add_css_class("warning-row");
        }

        let label_text = crate::utils::get_display_label(&b.mods, &b.key);
        let label = gtk::Label::builder()
            .label(label_text)
            .css_classes(vec!["dim-label".to_string()])
            .build();
        row.add_suffix(&label);

        row.connect_activated(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong(rename_to = b)]
            b.clone(),
            move |_| {
                obj.on_row_activated(b.clone());
            }
        ));

        row
    }

    fn on_row_activated(&self, b: Keybinding) {
        let dialog =
            crate::edit_dialog::EditBindingDialog::new(self.upcast_ref(), b.clone(), false);
        dialog.connect_response(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong(rename_to = b_old)]
            b.clone(),
            move |dialog, response| {
                if response == "save" {
                    let b_new = dialog.get_result_binding();
                    if let Some(conflict) = obj.check_conflict(Some(&b_old), &b_new) {
                        obj.prompt_overwrite(
                            dialog.upcast_ref(),
                            conflict.clone(),
                            glib::clone!(
                                #[weak]
                                obj,
                                #[strong]
                                b_old,
                                #[strong]
                                b_new,
                                #[weak]
                                dialog,
                                move || {
                                    obj.save_binding_change(
                                        b_old.clone(),
                                        b_new.clone(),
                                        Some(conflict.clone()),
                                    );
                                    dialog.close();
                                }
                            ),
                        );
                    } else {
                        obj.save_binding_change(b_old.clone(), b_new, None);
                        dialog.close();
                    }
                } else if response == "delete" {
                    let alert = adw::AlertDialog::builder()
                        .heading("Delete Keybinding?")
                        .body("Are you sure you want to delete this keybinding?")
                        .build();
                    alert.add_response("cancel", "Cancel");
                    alert.add_response("delete", "Delete");
                    alert.set_default_response(Some("delete")); // Set default response
                    alert.set_response_appearance("delete", adw::ResponseAppearance::Destructive);

                    alert.connect_response(
                        None, // Add the response_id argument
                        glib::clone!(
                            #[weak]
                            obj,
                            #[strong]
                            b_old,
                            #[weak]
                            dialog,
                            move |_, resp| {
                                if resp == "delete" {
                                    obj.delete_binding(b_old.clone());
                                    dialog.close();
                                }
                            }
                        ),
                    );
                    alert.present(Some(dialog));
                }
            }
        ));
        dialog.present();
    }

    fn on_add_clicked(&self) {
        let b = Keybinding {
            r#type: "bindd".to_string(),
            mods: "$mainMod".to_string(),
            key: "".to_string(),
            desc: "New Binding".to_string(),
            dispatcher: "exec".to_string(),
            arg: "".to_string(),
            category: "User Defined".to_string(),
        };
        let dialog = crate::edit_dialog::EditBindingDialog::new(self.upcast_ref(), b, true);
        dialog.connect_response(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |dialog, response| {
                if response == "save" {
                    let b_new = dialog.get_result_binding();
                    if let Some(conflict) = obj.check_conflict(None, &b_new) {
                        obj.prompt_overwrite(
                            dialog.upcast_ref(),
                            conflict.clone(),
                            glib::clone!(
                                #[weak]
                                obj,
                                #[strong]
                                b_new,
                                #[weak]
                                dialog,
                                move || {
                                    obj.append_binding(b_new.clone(), Some(conflict.clone()));
                                    dialog.close();
                                }
                            ),
                        );
                    } else {
                        obj.append_binding(b_new, None);
                        dialog.close();
                    }
                }
            }
        ));
        dialog.present();
    }

    fn check_conflict(
        &self,
        excluding_old: Option<&Keybinding>,
        new_binding: &Keybinding,
    ) -> Option<Keybinding> {
        let new_shortcut = normalize_shortcut(&new_binding.mods, &new_binding.key);
        if new_binding.key.trim().is_empty() {
            return None;
        }

        let bindings = self.imp().all_bindings.borrow();
        for b in bindings.iter() {
            if let Some(old) = excluding_old
                && b.mods == old.mods
                && b.key == old.key
                && b.dispatcher == old.dispatcher
                && b.arg == old.arg
            {
                continue;
            }
            if normalize_shortcut(&b.mods, &b.key) == new_shortcut {
                return Some(b.clone());
            }
        }
        None
    }

    fn prompt_overwrite<F>(&self, parent: &gtk::Window, conflict: Keybinding, on_confirm: F)
    where
        F: Fn() + 'static,
    {
        let on_confirm = std::rc::Rc::new(on_confirm);
        let shortcut = crate::utils::get_display_label(&conflict.mods, &conflict.key);
        let heading = "Keybinding Conflict";
        let body = format!(
            "The shortcut '{}' is already assigned to:\n\"{}\"\n\nDo you want to overwrite it?",
            shortcut, conflict.desc
        );

        let alert = adw::AlertDialog::builder()
            .heading(heading)
            .body(&body)
            .build();

        alert.add_response("cancel", "Cancel");
        alert.add_response("overwrite", "Overwrite");
        alert.set_default_response(Some("overwrite"));
        alert.set_response_appearance("overwrite", adw::ResponseAppearance::Destructive);

        alert.connect_response(
            None,
            glib::clone!(
                #[strong]
                on_confirm,
                move |_, resp| {
                    if resp == "overwrite" {
                        on_confirm();
                    }
                }
            ),
        );
        alert.present(Some(parent));
    }

    fn save_binding_change(
        &self,
        old: Keybinding,
        new: Keybinding,
        remove_conflict: Option<Keybinding>,
    ) {
        let store = ThemeStore::global();
        let theme = store.current_theme();
        if let Ok(data) = store.get_theme_data(&theme) {
            let mut bindings = data.keybindings.clone();

            if let Some(conf) = remove_conflict {
                let conf_shortcut = normalize_shortcut(&conf.mods, &conf.key);
                bindings.retain(|b| normalize_shortcut(&b.mods, &b.key) != conf_shortcut);
            }

            let old_shortcut = normalize_shortcut(&old.mods, &old.key);
            let mut idx = bindings.iter().position(|b| {
                normalize_shortcut(&b.mods, &b.key) == old_shortcut
                    && b.dispatcher.trim() == old.dispatcher.trim()
                    && b.arg.trim() == old.arg.trim()
            });

            if idx.is_none() {
                // Fallback to matching by shortcut only if it uniquely identifies a single binding
                let matches: Vec<usize> = bindings
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| normalize_shortcut(&b.mods, &b.key) == old_shortcut)
                    .map(|(i, _)| i)
                    .collect();
                if matches.len() == 1 {
                    idx = Some(matches[0]);
                }
            }

            if let Some(i) = idx {
                bindings[i] = new;
                store.stage_update("keybindings", yaml_serde::to_value(bindings).unwrap());
                self.load_bindings();
            }
        }
    }

    fn append_binding(&self, b: Keybinding, remove_conflict: Option<Keybinding>) {
        let store = ThemeStore::global();
        let theme = store.current_theme();
        if let Ok(data) = store.get_theme_data(&theme) {
            let mut bindings = data.keybindings.clone();

            if let Some(conf) = remove_conflict {
                let conf_shortcut = normalize_shortcut(&conf.mods, &conf.key);
                bindings.retain(|x| normalize_shortcut(&x.mods, &x.key) != conf_shortcut);
            }

            bindings.push(b);
            store.stage_update("keybindings", yaml_serde::to_value(bindings).unwrap());
            self.load_bindings();
        }
    }

    fn delete_binding(&self, b: Keybinding) {
        let store = ThemeStore::global();
        let theme = store.current_theme();
        if let Ok(data) = store.get_theme_data(&theme) {
            let mut bindings = data.keybindings.clone();
            let b_shortcut = normalize_shortcut(&b.mods, &b.key);
            bindings.retain(|x| {
                !(normalize_shortcut(&x.mods, &x.key) == b_shortcut
                    && x.dispatcher == b.dispatcher
                    && x.arg == b.arg)
            });
            store.stage_update("keybindings", yaml_serde::to_value(bindings).unwrap());
            self.load_bindings();
        }
    }

    fn on_search_changed(&self, text: String) {
        let imp = self.imp();

        let filtered: Vec<Keybinding> = if text.is_empty() {
            imp.all_bindings.borrow().clone()
        } else {
            use fuzzy_matcher::FuzzyMatcher;
            use fuzzy_matcher::skim::SkimMatcherV2;
            let matcher = SkimMatcherV2::default();
            imp.all_bindings
                .borrow()
                .iter()
                .filter(|b| {
                    matcher.fuzzy_match(&b.desc, &text).is_some()
                        || matcher.fuzzy_match(&b.mods, &text).is_some()
                        || matcher.fuzzy_match(&b.key, &text).is_some()
                        || matcher.fuzzy_match(&b.dispatcher, &text).is_some()
                        || matcher.fuzzy_match(&b.arg, &text).is_some()
                        || matcher.fuzzy_match(&b.category, &text).is_some()
                })
                .cloned()
                .collect()
        };

        let original = imp.all_bindings.replace(filtered);
        self.reflow_groups();
        imp.all_bindings.replace(original);

        if imp.columns_box.first_child().is_none() && !text.is_empty() {
            imp.stack.set_visible_child_name("empty");
        } else {
            imp.stack.set_visible_child_name("list");
        }
    }

    fn on_save_clicked(&self) {
        let store = ThemeStore::global();
        match store.commit_theme_updates() {
            Ok((success, msg)) => {
                if success {
                    self.add_toast(&msg);

                    // Immediate unbind logic to match Python version
                    let theme = store.current_theme();
                    if let Ok(data) = store.get_theme_data(&theme) {
                        let main_mod = data.input.main_mod.clone();
                        for b in &data.keybindings {
                            if b.r#type == "unbind" {
                                let mut mods_str = b.mods.clone();
                                if b.mods == "$mainMod ALT" && b.key == "SPACE" && main_mod.to_uppercase() == "ALT" {
                                    mods_str = "ALT SUPER".to_string();
                                }
                                let mods = mods_str
                                    .replace("$mainMod", &main_mod)
                                    .replace("$MAINMOD", &main_mod)
                                    .replace(" ", " + ");
                                let binding_str = if b.key.is_empty() {
                                    mods
                                } else {
                                    format!("{} + {}", mods, b.key)
                                };

                                let _ = std::process::Command::new("hyprctl")
                                    .arg("eval")
                                    .arg(format!("hl.unbind(\"{}\")", binding_str))
                                    .spawn();
                            }
                        }
                    }
                }
            }
            Err(e) => {
                self.add_toast(&format!("Error: {}", e));
            }
        }
    }

    fn add_toast(&self, msg: &str) {
        let toast = adw::Toast::new(msg);
        self.imp().toast_overlay.add_toast(toast);
    }
}

fn normalize_shortcut(mods: &str, key: &str) -> String {
    let main_mod = {
        let theme = ThemeStore::global().current_theme();
        ThemeStore::global()
            .get_theme_data(&theme)
            .map(|data| data.input.main_mod.to_uppercase())
            .unwrap_or_else(|_| "SUPER".to_string())
    };

    let mut mods_str = mods.to_string();
    if mods == "$mainMod ALT" && key == "SPACE" && main_mod == "ALT" {
        mods_str = "ALT SUPER".to_string();
    }

    let mut parts: Vec<String> = mods_str
        .replace("$mainMod", &main_mod)
        .replace("$MAINMOD", &main_mod)
        .split_whitespace()
        .map(|s| s.to_uppercase())
        .collect();
    parts.sort();
    let normalized_key = key.trim().to_uppercase();
    if parts.is_empty() {
        normalized_key
    } else {
        format!("{}+{}", parts.join("+"), normalized_key)
    }
}
