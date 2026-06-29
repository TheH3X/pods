use gtk::glib::subclass::prelude::\*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposeVolumeRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">8</property>
            <property name="margin-top">8</property>
            <property name="margin-bottom">8</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <child>
              <object class="gtk_image">
                <property name="icon-name">drive-harddisk-symbolic</property>
                <property name="valign">center</property>
                <property name="css-classes">dim-label</property>
              </object>
            </child>
            <child>
              <object class="gtk_entry" id="host_path_entry">
                <property name="placeholder-text">Host path</property>
                <property name="hexpand">true</property>
              </object>
            </child>
            <child>
              <object class="gtk_label">
                <property name="label">→</property>
                <property name="css-classes">dim-label</property>
              </object>
            </child>
            <child>
              <object class="gtk_entry" id="container_path_entry">
                <property name="placeholder-text">Container path</property>
                <property name="hexpand">true</property>
              </object>
            </child>
            <child>
              <object class="gtk_drop_down" id="options_dropdown">
                <property name="valign">center</property>
              </object>
            </child>
            <child>
              <object class="gtk_button" id="browse_button">
                <property name="icon-name">folder-open-symbolic</property>
                <property name="valign">center</property>
                <property name="tooltip-text">Browse…</property>
                <property name="css-classes">flat</property>
              </object>
            </child>
            <child>
              <object class="gtk_button" id="remove_button">
                <property name="icon-name">user-trash-symbolic</property>
                <property name="valign">center</property>
                <property name="tooltip-text">Remove</property>
                <property name="css-classes">flat destructive-action</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct ComposeVolumeRow {
        #[template_child]
        pub(super) host_path_entry: gtk::TemplateChild<gtk::Entry>,
        #[template_child]
        pub(super) container_path_entry: gtk::TemplateChild<gtk::Entry>,
        #[template_child]
        pub(super) browse_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) remove_button: gtk::TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeVolumeRow {
        const NAME: &'static str = "StacksComposeVolumeRow";
        type Type = super::ComposeVolumeRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ComposeVolumeRow {}
    impl WidgetImpl for ComposeVolumeRow {}
    impl ListBoxRowImpl for ComposeVolumeRow {}
}

glib::wrapper! {
    pub(crate) struct ComposeVolumeRow(ObjectSubclass<imp::ComposeVolumeRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl ComposeVolumeRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Create a row pre-filled with a volume spec.
    pub fn with_spec(host: &str, container: &str) -> Self {
        let row = Self::new();
        row.imp().host_path_entry.set_text(host);
        row.imp().container_path_entry.set_text(container);
        row
    }

    /// Get the volume spec from the row's entries.
    pub fn to_spec(&self) -> String {
        let host = self.imp().host_path_entry.text();
        let container = self.imp().container_path_entry.text();
        format!("{host}:{container}")
    }
}
