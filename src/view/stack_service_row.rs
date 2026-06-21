use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksStackServiceRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <child>
              <object class="gtk_label" id="name_label">
                <property name="hexpand">true</property>
                <property name="halign">start</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct StackServiceRow {
        #[template_child]
        pub(super) name_label: gtk::TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackServiceRow {
        const NAME: &'static str = "StacksStackServiceRow";
        type Type = super::StackServiceRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StackServiceRow {}
    impl WidgetImpl for StackServiceRow {}
    impl ListBoxRowImpl for StackServiceRow {}
}

glib::wrapper! {
    pub(crate) struct StackServiceRow(ObjectSubclass<imp::StackServiceRow>)
        @extends gtk::ListBoxRow, gtk::Widget;
}

impl StackServiceRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
