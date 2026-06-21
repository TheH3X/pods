use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};
use std::cell::OnceCell;

use crate::model::StackList;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksStacksPanel" parent="gtk_box">
        <property name="orientation">vertical</property>
        <child>
          <object class="gtk_scrolled_window">
            <property name="hscrollbar-policy">never</property>
            <child>
              <object class="gtk_list_box" id="list_box">
                <property name="selection-mode">none</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct StacksPanel {
        #[template_child]
        pub(super) list_box: gtk::TemplateChild<gtk::ListBox>,
        #[property(get, set = Self::set_stack_list, nullable)]
        pub(super) stack_list: glib::WeakRef<StackList>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StacksPanel {
        const NAME: &'static str = "StacksStacksPanel";
        type Type = super::StacksPanel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StacksPanel {
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

    impl WidgetImpl for StacksPanel {}
    impl BoxImpl for StacksPanel {}
    
    impl StacksPanel {
        fn set_stack_list(&self, value: Option<&StackList>) {
            if self.obj().stack_list().as_ref() == value {
                return;
            }

            if let Some(list) = value {
                self.list_box.bind_model(Some(list), |item| {
                    let stack = item.downcast_ref::<crate::model::Stack>().unwrap();
                    glib::Object::builder::<crate::view::StackRow>()
                        .property("stack", stack)
                        .build()
                        .upcast()
                });
            }

            self.stack_list.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct StacksPanel(ObjectSubclass<imp::StacksPanel>)
        @extends gtk::Box, gtk::Widget;
}

impl StacksPanel {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
