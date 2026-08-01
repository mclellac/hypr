use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use libadwaita as adw;

glib::wrapper! {
    pub struct BackupPage(ObjectSubclass<imp::BackupPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageSettings for BackupPage {
    fn setup_ui(&self) {
        // Implementation details for backup/restore
    }

    fn refresh_settings(&self) {
        // Nothing to refresh
    }
}

impl BackupPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

pub mod imp {
    use super::*;

    #[derive(Default)]
    pub struct BackupPage {
        pub group: adw::PreferencesGroup,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BackupPage {
        const NAME: &'static str = "BackupPage";
        type Type = super::BackupPage;
        type ParentType = adw::PreferencesPage;
    }

    impl ObjectImpl for BackupPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_title("Backup and Restore");
            obj.set_icon_name(Some("document-save-symbolic"));

            self.group.set_title("Configuration Management");
            self.group.set_description(Some(
                "Backup your Hyprland configuration or restore from a previous backup.",
            ));

            // Backup Row
            let backup_btn = gtk::Button::builder()
                .label("Backup")
                .valign(gtk::Align::Center)
                .build();

            let backup_row = adw::ActionRow::builder()
                .title("Backup Configuration")
                .subtitle("Create a ZIP archive of your current settings.")
                .build();
            backup_row.add_suffix(&backup_btn);

            self.group.add(&backup_row);

            // Restore Row
            let restore_btn = gtk::Button::builder()
                .label("Restore")
                .valign(gtk::Align::Center)
                .build();

            let restore_row = adw::ActionRow::builder()
                .title("Restore Configuration")
                .subtitle("Restore settings from a backup archive.")
                .build();
            restore_row.add_suffix(&restore_btn);

            self.group.add(&restore_row);

            obj.add(&self.group);
        }
    }

    impl WidgetImpl for BackupPage {}
    impl PreferencesPageImpl for BackupPage {}
}
