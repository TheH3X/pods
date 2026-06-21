use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksStackRow" parent="gtk_list_box_row">
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
    pub(crate) struct StackRow {
        #[template_child]
        pub(super) name_label: gtk::TemplateChild<gtk::Label>,
        #[property(get, set = Self::set_stack, nullable)]
        pub(super) stack: glib::WeakRef<crate::model::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackRow {
        const NAME: &'static str = "StacksStackRow";
        type Type = super::StackRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StackRow {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec);
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for StackRow {}
    impl ListBoxRowImpl for StackRow {}
    
    impl StackRow {
        fn set_stack(&self, value: Option<&crate::model::Stack>) {
            if self.obj().stack().as_ref() == value {
                return;
            }

            if let Some(stack) = value {
                stack.bind_property("name", &*self.name_label, "label")
                    .sync_create()
                    .build();
            }

            self.stack.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct StackRow(ObjectSubclass<imp::StackRow>)
        @extends gtk::ListBoxRow, gtk::Widget;
}

impl StackRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
