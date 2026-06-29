use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksStackEditorPage" parent="adw_navigation_page">
        <child>
          <object class="gtk_box">
            <property name="orientation">vertical</property>
            <child>
              <object class="adw_header_bar">
                <child type="end">
                  <object class="gtk_button" id="review_plan_button">
                    <property name="label">Review Plan</property>
                    <property name="css-classes">suggested-action</property>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="adw_preferences_page">
                <child>
                  <object class="adw_preferences_group">
                    <property name="title">Services</property>
                    <child>
                      <object class="gtk_list_box" id="services_list_box">
                        <property name="selection-mode">none</property>
                        <property name="css-classes">boxed-list</property>
                      </object>
                    </child>
                    <child>
                      <object class="gtk_button" id="add_service_button">
                        <property name="label">Add Service</property>
                        <property name="margin-top">12</property>
                      </object>
                    </child>
                  </object>
                </child>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct StackEditorPage {
        #[template_child]
        pub(super) services_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) add_service_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) review_plan_button: gtk::TemplateChild<gtk::Button>,
        #[property(get, set = Self::set_stack, nullable)]
        pub(super) stack: glib::WeakRef<crate::model::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackEditorPage {
        const NAME: &'static str = "StacksStackEditorPage";
        type Type = super::StackEditorPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StackEditorPage {
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

    impl WidgetImpl for StackEditorPage {}
    impl adw::subclass::navigation_page::NavigationPageImpl for StackEditorPage {}

    impl StackEditorPage {
        fn set_stack(&self, value: Option<&crate::model::Stack>) {
            if self.obj().stack().as_ref() == value {
                return;
            }

            if let Some(stack) = value {
                stack
                    .bind_property("name", &*self.obj(), "title")
                    .sync_create()
                    .build();

                if let Some(service_list) = stack.service_list() {
                    self.services_list_box
                        .bind_model(Some(&service_list), |item| {
                            let service =
                                item.downcast_ref::<crate::model::ComposeService>().unwrap();
                            glib::Object::builder::<crate::view::ComposeServiceSummaryRow>()
                                .property("service", service)
                                .build()
                                .upcast()
                        });
                }
            }

            self.stack.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct StackEditorPage(ObjectSubclass<imp::StackEditorPage>)
        @extends adw::NavigationPage, gtk::Widget;
}

impl StackEditorPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
