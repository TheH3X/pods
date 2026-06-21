use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksStackDetailsPage" parent="adw_navigation_page">
        <child>
          <object class="gtk_box">
            <property name="orientation">vertical</property>
            <child>
              <object class="adw_header_bar" />
            </child>
            <child>
              <object class="gtk_label">
                <property name="label">Stack Details</property>
                <property name="vexpand">true</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct StackDetailsPage {}

    #[glib::object_subclass]
    impl ObjectSubclass for StackDetailsPage {
        const NAME: &'static str = "StacksStackDetailsPage";
        type Type = super::StackDetailsPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StackDetailsPage {}
    impl WidgetImpl for StackDetailsPage {}
    impl adw::subclass::navigation_page::NavigationPageImpl for StackDetailsPage {}
}

glib::wrapper! {
    pub(crate) struct StackDetailsPage(ObjectSubclass<imp::StackDetailsPage>)
        @extends adw::NavigationPage, gtk::Widget;
}

impl StackDetailsPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
