use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::models::Keybinding;
use libadwaita as adw;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct EditBindingDialog {
        pub binding: RefCell<Keybinding>,
        pub is_new: RefCell<bool>,

        pub desc_entry: gtk::Entry,
        pub mods_entry: gtk::Entry,
        pub key_entry: gtk::Entry,
        pub dispatcher_row: adw::ComboRow,
        pub arg_entry: gtk::Entry,
        pub category_entry: gtk::Entry,

        pub save_btn: gtk::Button,
        pub delete_btn: gtk::Button,
        pub capture_btn: gtk::Button,
        pub banner: adw::Banner,

        pub is_capturing: RefCell<bool>,
        pub captured_keys: RefCell<bool>,
        pub toast_overlay: adw::ToastOverlay,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EditBindingDialog {
        const NAME: &'static str = "EditBindingDialog";
        type Type = super::EditBindingDialog;
        type ParentType = adw::Window;
    }

    impl ObjectImpl for EditBindingDialog {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_title(Some("Edit Keybinding"));
            obj.set_default_width(500);
            obj.set_modal(true);

            obj.set_content(Some(&self.toast_overlay));

            let toolbar_view = adw::ToolbarView::new();
            self.toast_overlay.set_child(Some(&toolbar_view));

            let header_bar = adw::HeaderBar::new();
            toolbar_view.add_top_bar(&header_bar);

            self.banner
                .set_title("Intercepting all keys... Press ESC to cancel.");
            self.banner.set_revealed(false);
            toolbar_view.add_top_bar(&self.banner);

            let cancel_btn = gtk::Button::with_label("Cancel");
            cancel_btn.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.close();
                }
            ));
            header_bar.pack_start(&cancel_btn);

            self.save_btn.set_label("Save");
            self.save_btn.add_css_class("suggested-action");
            header_bar.pack_end(&self.save_btn);

            self.delete_btn.set_label("Delete");
            self.delete_btn.add_css_class("destructive-action");
            header_bar.pack_end(&self.delete_btn);

            let prefs_page = adw::PreferencesPage::new();
            toolbar_view.set_content(Some(&prefs_page));

            let group = adw::PreferencesGroup::new();
            prefs_page.add(&group);

            // Description
            let desc_row = adw::ActionRow::builder().title("Description").build();
            self.desc_entry.set_valign(gtk::Align::Center);
            self.desc_entry.set_hexpand(true);
            desc_row.add_suffix(&self.desc_entry);
            group.add(&desc_row);

            // Category
            let cat_row = adw::ActionRow::builder().title("Category").build();
            self.category_entry.set_valign(gtk::Align::Center);
            self.category_entry.set_hexpand(true);
            cat_row.add_suffix(&self.category_entry);
            group.add(&cat_row);

            // Modifiers
            let mods_row = adw::ActionRow::builder().title("Modifiers").build();
            self.mods_entry.set_valign(gtk::Align::Center);
            self.mods_entry.set_hexpand(true);
            mods_row.add_suffix(&self.mods_entry);
            group.add(&mods_row);

            // Key
            let key_row = adw::ActionRow::builder().title("Key").build();
            self.key_entry.set_valign(gtk::Align::Center);
            self.key_entry.set_hexpand(true);
            key_row.add_suffix(&self.key_entry);
            group.add(&key_row);

            // Capture Button
            let capture_row = adw::ActionRow::builder().title("Capture Key").build();
            self.capture_btn.set_label("Capture");
            self.capture_btn.set_valign(gtk::Align::Center);
            capture_row.add_suffix(&self.capture_btn);
            group.add(&capture_row);

            self.capture_btn.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.toggle_capture();
                }
            ));

            // Dispatcher
            let dispatchers = [
                "exec",
                "killactive",
                "exit",
                "movefocus",
                "movewindow",
                "swapwindow",
                "centerwindow",
                "workspace",
                "movetoworkspace",
                "movetoworkspacesilent",
                "togglefloating",
                "fullscreen",
                "fakefullscreen",
                "dpms",
                "pin",
                "pseudo",
                "cyclenext",
                "swapnext",
                "tag",
                "focusmonitor",
                "movecurrentworkspacetomonitor",
                "movewindowtoworkspace",
                "movewindowtoworkspacesilent",
                "toggleopaque",
                "movecursor",
                "renameworkspace",
                "setstackel",
                "swapactiveworkspaces",
                "alterzorder",
                "togglegroup",
                "changegroupactive",
                "focuscurrentorlast",
                "lockactivegroup",
                "lockgroups",
                "moveintogroup",
                "moveoutofgroup",
                "movewindoworgroup",
                "movegroupwindow",
                "denywindowfromgroup",
                "setignoregrouplock",
                "global",
                "submap",
                "pass",
            ];
            let mut dispatchers_vec: Vec<_> = dispatchers.iter().map(|s| s.to_string()).collect();
            dispatchers_vec.sort();

            let model = gtk::StringList::new(
                &dispatchers_vec
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
            );
            self.dispatcher_row.set_title("Dispatcher");
            self.dispatcher_row.set_model(Some(&model));
            group.add(&self.dispatcher_row);

            // Argument
            let arg_row = adw::ActionRow::builder().title("Argument").build();
            self.arg_entry.set_valign(gtk::Align::Center);
            self.arg_entry.set_hexpand(true);
            arg_row.add_suffix(&self.arg_entry);
            group.add(&arg_row);

            // Key Controller
            let controller = gtk::EventControllerKey::new();
            controller.set_propagation_phase(gtk::PropagationPhase::Capture);
            let obj_weak = obj.downgrade();
            controller.connect_key_pressed(move |_, keyval, _keycode, state| {
                if let Some(obj) = obj_weak.upgrade() {
                    obj.handle_key_pressed(keyval, state)
                } else {
                    glib::Propagation::Proceed
                }
            });
            let obj_weak2 = obj.downgrade();
            controller.connect_key_released(move |_, keyval, _keycode, state| {
                if let Some(obj) = obj_weak2.upgrade() {
                    obj.handle_key_released(keyval, state);
                }
            });
            obj.add_controller(controller);
        }
    }

    impl WidgetImpl for EditBindingDialog {}
    impl WindowImpl for EditBindingDialog {
        fn close_request(&self) -> glib::Propagation {
            if *self.is_capturing.borrow() {
                self.obj().stop_capture(true);
            }
            glib::Propagation::Proceed
        }
    }
    impl AdwWindowImpl for EditBindingDialog {}
}

glib::wrapper! {
    pub struct EditBindingDialog(ObjectSubclass<imp::EditBindingDialog>)
        @extends adw::Window, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl EditBindingDialog {
    pub fn new(parent: &gtk::Window, binding: Keybinding, is_new: bool) -> Self {
        let obj: Self = glib::Object::new();
        obj.set_transient_for(Some(parent));

        let imp = obj.imp();
        *imp.binding.borrow_mut() = binding.clone();
        *imp.is_new.borrow_mut() = is_new;

        imp.desc_entry.set_text(&binding.desc);
        imp.category_entry.set_text(&binding.category);
        imp.mods_entry.set_text(&binding.mods);
        imp.key_entry.set_text(&binding.key);
        imp.arg_entry.set_text(&binding.arg);

        if is_new {
            obj.set_title(Some("Add Keybinding"));
            imp.delete_btn.set_visible(false);
        } else {
            obj.set_title(Some("Edit Keybinding"));
            imp.delete_btn.set_visible(true);
        }

        // Select dispatcher in combo
        if let Some(model) = imp.dispatcher_row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                if let Some(s) = string_list.string(i)
                    && s == binding.dispatcher
                {
                    imp.dispatcher_row.set_selected(i);
                    break;
                }
            }
        }

        obj
    }

    fn toggle_capture(&self) {
        let imp = self.imp();
        if *imp.is_capturing.borrow() {
            self.stop_capture(true);
        } else {
            self.start_capture();
        }
    }

    fn start_capture(&self) {
        let imp = self.imp();
        *imp.is_capturing.borrow_mut() = true;
        *imp.captured_keys.borrow_mut() = false;
        imp.capture_btn.set_label("Press Key...");
        imp.capture_btn.add_css_class("suggested-action");
        imp.banner.set_revealed(true);

        // Inhibit Hyprland bindings
        let _ = std::process::Command::new("hyprctl")
            .arg("dispatch")
            .arg("submap")
            .arg("empty")
            .status();
    }

    fn stop_capture(&self, cancel: bool) {
        let imp = self.imp();
        imp.capture_btn.set_label("Capture");
        imp.capture_btn.remove_css_class("suggested-action");
        imp.banner.set_revealed(false);

        if cancel {
            // Check if any main modifiers are physically held down
            let mut active_mods = false;
            if let Some(display) = gtk::gdk::Display::default() {
                let seat = display.default_seat();
                if let Some(keyboard) = seat.and_then(|s| s.keyboard()) {
                    let current_state = keyboard.modifier_state();
                    active_mods = current_state.contains(gtk::gdk::ModifierType::SUPER_MASK)
                        || current_state.contains(gtk::gdk::ModifierType::SHIFT_MASK)
                        || current_state.contains(gtk::gdk::ModifierType::ALT_MASK)
                        || current_state.contains(gtk::gdk::ModifierType::CONTROL_MASK);
                }
            }

            if active_mods {
                *imp.captured_keys.borrow_mut() = true;
                return;
            }

            *imp.captured_keys.borrow_mut() = false;
            *imp.is_capturing.borrow_mut() = false;
            let _ = std::process::Command::new("hyprctl")
                .arg("dispatch")
                .arg("submap")
                .arg("reset")
                .status();
        } else {
            *imp.captured_keys.borrow_mut() = true;
        }
    }

    fn handle_key_released(&self, keyval: gtk::gdk::Key, state: gtk::gdk::ModifierType) {
        let imp = self.imp();
        if !*imp.captured_keys.borrow() {
            return;
        }

        let mut active_mods = state;

        // Check if the key released is a modifier and clear it
        match keyval {
            gtk::gdk::Key::Super_L
            | gtk::gdk::Key::Super_R
            | gtk::gdk::Key::Meta_L
            | gtk::gdk::Key::Meta_R => {
                active_mods.remove(gtk::gdk::ModifierType::SUPER_MASK);
            }
            gtk::gdk::Key::Shift_L | gtk::gdk::Key::Shift_R => {
                active_mods.remove(gtk::gdk::ModifierType::SHIFT_MASK);
            }
            gtk::gdk::Key::Control_L | gtk::gdk::Key::Control_R => {
                active_mods.remove(gtk::gdk::ModifierType::CONTROL_MASK);
            }
            gtk::gdk::Key::Alt_L | gtk::gdk::Key::Alt_R => {
                active_mods.remove(gtk::gdk::ModifierType::ALT_MASK);
            }
            _ => {}
        }

        let is_holding = active_mods.contains(gtk::gdk::ModifierType::SUPER_MASK)
            || active_mods.contains(gtk::gdk::ModifierType::SHIFT_MASK)
            || active_mods.contains(gtk::gdk::ModifierType::ALT_MASK)
            || active_mods.contains(gtk::gdk::ModifierType::CONTROL_MASK);

        if !is_holding {
            *imp.captured_keys.borrow_mut() = false;
            *imp.is_capturing.borrow_mut() = false;
            let _ = std::process::Command::new("hyprctl")
                .arg("dispatch")
                .arg("submap")
                .arg("reset")
                .status();
        }
    }

    fn handle_key_pressed(
        &self,
        keyval: gtk::gdk::Key,
        state: gtk::gdk::ModifierType,
    ) -> glib::Propagation {
        let imp = self.imp();
        if !*imp.is_capturing.borrow() {
            return glib::Propagation::Proceed;
        }

        if *imp.captured_keys.borrow() {
            return glib::Propagation::Stop;
        }

        let is_modifier = matches!(
            keyval,
            gtk::gdk::Key::Control_L
                | gtk::gdk::Key::Control_R
                | gtk::gdk::Key::Shift_L
                | gtk::gdk::Key::Shift_R
                | gtk::gdk::Key::Alt_L
                | gtk::gdk::Key::Alt_R
                | gtk::gdk::Key::Super_L
                | gtk::gdk::Key::Super_R
                | gtk::gdk::Key::Meta_L
                | gtk::gdk::Key::Meta_R
        );

        if is_modifier {
            return glib::Propagation::Proceed;
        }

        // Handle Escape cancel during capture
        if keyval == gtk::gdk::Key::Escape
            && !state.contains(gtk::gdk::ModifierType::SUPER_MASK)
            && !state.contains(gtk::gdk::ModifierType::SHIFT_MASK)
            && !state.contains(gtk::gdk::ModifierType::ALT_MASK)
            && !state.contains(gtk::gdk::ModifierType::CONTROL_MASK)
        {
            self.stop_capture(true);
            return glib::Propagation::Stop;
        }

        let name = keyval.name().unwrap_or_else(|| "Unknown".into());
        let mut mods = Vec::new();

        if state.contains(gtk::gdk::ModifierType::SUPER_MASK) {
            mods.push("SUPER");
        }
        if state.contains(gtk::gdk::ModifierType::SHIFT_MASK) {
            mods.push("SHIFT");
        }
        if state.contains(gtk::gdk::ModifierType::ALT_MASK) {
            mods.push("ALT");
        }
        if state.contains(gtk::gdk::ModifierType::CONTROL_MASK) {
            mods.push("CONTROL");
        }

        // Mappings for specific standard keys
        let raw_name = name.to_string();
        let key_name = match raw_name.as_str() {
            "Return" => "RETURN".to_string(),
            "Escape" => "ESCAPE".to_string(),
            "space" => "SPACE".to_string(),
            "minus" => "MINUS".to_string(),
            "equal" => "EQUAL".to_string(),
            "bracketleft" => "BRACKETLEFT".to_string(),
            "bracketright" => "BRACKETRIGHT".to_string(),
            "backslash" => "BACKSLASH".to_string(),
            "semicolon" => "SEMICOLON".to_string(),
            "apostrophe" => "APOSTROPHE".to_string(),
            "comma" => "COMMA".to_string(),
            "period" => "PERIOD".to_string(),
            "slash" => "SLASH".to_string(),
            "grave" => "GRAVE".to_string(),
            "Tab" => "TAB".to_string(),
            "Caps_Lock" => "CAPSLOCK".to_string(),
            "Scroll_Lock" => "SCROLLLOCK".to_string(),
            "Num_Lock" => "NUMLOCK".to_string(),
            "Print" => "PRINT".to_string(),
            "Pause" => "PAUSE".to_string(),
            "Insert" => "INSERT".to_string(),
            "Delete" => "DELETE".to_string(),
            "Home" => "HOME".to_string(),
            "End" => "END".to_string(),
            "Page_Up" => "PAGEUP".to_string(),
            "Page_Down" => "PAGEDOWN".to_string(),
            "Left" => "LEFT".to_string(),
            "Right" => "RIGHT".to_string(),
            "Up" => "UP".to_string(),
            "Down" => "DOWN".to_string(),
            other => other.to_uppercase(),
        };

        imp.mods_entry.set_text(&mods.join(" "));
        imp.key_entry.set_text(&key_name);

        self.stop_capture(false);
        glib::Propagation::Stop
    }

    pub fn connect_response<F: Fn(&Self, &str) + 'static>(&self, f: F) {
        let imp = self.imp();
        let f = std::rc::Rc::new(f);

        let f_save = f.clone();
        imp.save_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                f_save(&obj, "save");
            }
        ));

        let f_delete = f.clone();
        imp.delete_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                f_delete(&obj, "delete");
            }
        ));
    }

    pub fn get_result_binding(&self) -> Keybinding {
        let imp = self.imp();
        let mut b = imp.binding.borrow().clone();
        b.desc = imp.desc_entry.text().to_string();
        b.category = imp.category_entry.text().to_string();
        b.mods = imp.mods_entry.text().to_string();
        b.key = imp.key_entry.text().to_string();
        b.arg = imp.arg_entry.text().to_string();

        if let Some(item) = imp.dispatcher_row.selected_item()
            && let Some(string_obj) = item.downcast_ref::<gtk::StringObject>()
        {
            b.dispatcher = string_obj.string().to_string();
        }

        b
    }
}
