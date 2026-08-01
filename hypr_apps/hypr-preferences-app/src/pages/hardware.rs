use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use libadwaita as adw;

glib::wrapper! {
    pub struct HardwarePage(ObjectSubclass<imp::HardwarePage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageSettings for HardwarePage {
    fn setup_ui(&self) {
        let imp = self.imp();

        // Product Name
        if let Some(label) = imp.builder.object::<gtk::Label>("hw_match_label") {
            if let Ok(name) = std::fs::read_to_string("/sys/class/dmi/id/product_name") {
                label.set_label(name.trim());
            } else {
                label.set_label("Unknown Hardware");
            }
        }

        self.setup_button("apple_brightness_btn", "hypr-cmd-apple-display-brightness");
        self.setup_button("asus_audio_btn", "hypr-hw-fix-asus-audio-mixer");
        self.setup_button("fingerprint_btn", "hypr-setup-fingerprint");
        self.setup_button("fido2_btn", "hypr-setup-fido2");
    }

    fn refresh_settings(&self) {
        // Nothing to refresh
    }
}

impl HardwarePage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn setup_button(&self, id: &str, command: &'static str) {
        let imp = self.imp();
        if let Some(btn) = imp.builder.object::<gtk::Button>(id) {
            btn.connect_clicked(move |_| {
                let term = std::env::var("TERMINAL").unwrap_or_else(|_| "alacritty".to_string());
                let bash_cmd = format!("{}; echo 'Press any key to close...'; read -n 1", command);

                let mut cmd = std::process::Command::new(&term);
                if term.contains("gnome-terminal") {
                    cmd.arg("--").arg("bash").arg("-c").arg(&bash_cmd);
                } else {
                    cmd.arg("-e").arg("bash").arg("-c").arg(&bash_cmd);
                }
                let _ = cmd.spawn();
            });
        }
    }
}

pub mod imp {
    use super::*;

    pub struct HardwarePage {
        pub builder: gtk::Builder,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HardwarePage {
        const NAME: &'static str = "HardwarePage";
        type Type = super::HardwarePage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/hardware.ui"));
            Self { builder }
        }
    }

    impl Default for HardwarePage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for HardwarePage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = ["detection_group", "quirks_group", "security_group"];
            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for HardwarePage {}
    impl PreferencesPageImpl for HardwarePage {}
}
