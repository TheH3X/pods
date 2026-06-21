use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposeExtraFieldRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <child>
              <object class="gtk_label" id="field_label">
                <property name="hexpand">true</property>
                <property name="halign">start</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct ComposeExtraFieldRow {
        #[template_child]
        pub(super) field_label: gtk::TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeExtraFieldRow {
        const NAME: &'static str = "StacksComposeExtraFieldRow";
        type Type = super::ComposeExtraFieldRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ComposeExtraFieldRow {}
    impl WidgetImpl for ComposeExtraFieldRow {}
    impl ListBoxRowImpl for ComposeExtraFieldRow {}
}

glib::wrapper! {
    pub(crate) struct ComposeExtraFieldRow(ObjectSubclass<imp::ComposeExtraFieldRow>)
        @extends gtk::ListBoxRow, gtk::Widget;
}

impl ComposeExtraFieldRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
