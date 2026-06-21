use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposePortRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <child>
              <object class="gtk_entry" id="host_port_entry">
                <property name="placeholder-text">Host</property>
              </object>
            </child>
            <child>
              <object class="gtk_label">
                <property name="label">:</property>
              </object>
            </child>
            <child>
              <object class="gtk_entry" id="container_port_entry">
                <property name="placeholder-text">Container</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct ComposePortRow {
        #[template_child]
        pub(super) host_port_entry: gtk::TemplateChild<gtk::Entry>,
        #[template_child]
        pub(super) container_port_entry: gtk::TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposePortRow {
        const NAME: &'static str = "StacksComposePortRow";
        type Type = super::ComposePortRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ComposePortRow {}
    impl WidgetImpl for ComposePortRow {}
    impl ListBoxRowImpl for ComposePortRow {}
}

glib::wrapper! {
    pub(crate) struct ComposePortRow(ObjectSubclass<imp::ComposePortRow>)
        @extends gtk::ListBoxRow, gtk::Widget;
}

impl ComposePortRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
