use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposeEnvRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <child>
              <object class="gtk_entry" id="key_entry">
                <property name="placeholder-text">Key</property>
                <property name="hexpand">true</property>
              </object>
            </child>
            <child>
              <object class="gtk_label">
                <property name="label">=</property>
              </object>
            </child>
            <child>
              <object class="gtk_entry" id="value_entry">
                <property name="placeholder-text">Value</property>
                <property name="hexpand">true</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct ComposeEnvRow {
        #[template_child]
        pub(super) key_entry: gtk::TemplateChild<gtk::Entry>,
        #[template_child]
        pub(super) value_entry: gtk::TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeEnvRow {
        const NAME: &'static str = "StacksComposeEnvRow";
        type Type = super::ComposeEnvRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ComposeEnvRow {}
    impl WidgetImpl for ComposeEnvRow {}
    impl ListBoxRowImpl for ComposeEnvRow {}
}

glib::wrapper! {
    pub(crate) struct ComposeEnvRow(ObjectSubclass<imp::ComposeEnvRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl ComposeEnvRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
