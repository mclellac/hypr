use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use libadwaita as adw;
use std::process::Command;

glib::wrapper! {
    pub struct KeybindingsPage(ObjectSubclass<imp::KeybindingsPage>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageSettings for KeybindingsPage {
    fn setup_ui(&self) {
        let imp = self.imp();
        if let Some(btn) = imp.builder.object::<gtk::Button>("open_editor_btn") {
            btn.connect_clicked(|_| {
                let _ = Command::new("hypr-keybindings-editor").spawn();
            });
        }
    }

    fn refresh_settings(&self) {
        // Nothing to refresh
    }
}

impl KeybindingsPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

pub mod imp {
    use super::*;

    pub struct KeybindingsPage {
        pub builder: gtk::Builder,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KeybindingsPage {
        const NAME: &'static str = "KeybindingsPage";
        type Type = super::KeybindingsPage;
        type ParentType = adw::Bin;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/keybindings.ui"));
            Self { builder }
        }
    }

    impl Default for KeybindingsPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for KeybindingsPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            if let Some(content) = self.builder.object::<gtk::Widget>("page_content") {
                content.unparent();
                obj.set_child(Some(&content));
            }
        }
    }

    impl WidgetImpl for KeybindingsPage {}
    impl BinImpl for KeybindingsPage {}
}
