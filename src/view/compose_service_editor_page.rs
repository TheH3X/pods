use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposeServiceEditorPage" parent="adw_navigation_page">
        <child>
          <object class="gtk_box">
            <property name="orientation">vertical</property>
            <child>
              <object class="adw_header_bar" />
            </child>
            <child>
              <object class="adw_preferences_page">
                <child>
                  <object class="adw_preferences_group">
                    <property name="title">General</property>
                    <child>
                      <object class="adw_entry_row" id="image_entry">
                        <property name="title">Image</property>
                      </object>
                    </child>
                  </object>
                </child>
                <child>
                  <object class="adw_preferences_group">
                    <property name="title">Ports</property>
                    <child>
                      <object class="gtk_list_box" id="ports_list_box">
                        <property name="selection-mode">none</property>
                        <property name="css-classes">boxed-list</property>
                      </object>
                    </child>
                  </object>
                </child>
                <child>
                  <object class="adw_preferences_group">
                    <property name="title">Environment</property>
                    <child>
                      <object class="gtk_list_box" id="env_list_box">
                        <property name="selection-mode">none</property>
                        <property name="css-classes">boxed-list</property>
                      </object>
                    </child>
                  </object>
                </child>
                <child>
                  <object class="adw_preferences_group">
                    <property name="title">Extras</property>
                    <child>
                      <object class="gtk_list_box" id="extras_list_box">
                        <property name="selection-mode">none</property>
                        <property name="css-classes">boxed-list</property>
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
    pub(crate) struct ComposeServiceEditorPage {
        #[template_child]
        pub(super) image_entry: gtk::TemplateChild<adw::EntryRow>,
        #[template_child]
        pub(super) ports_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) env_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) extras_list_box: gtk::TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeServiceEditorPage {
        const NAME: &'static str = "StacksComposeServiceEditorPage";
        type Type = super::ComposeServiceEditorPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ComposeServiceEditorPage {}
    impl WidgetImpl for ComposeServiceEditorPage {}
    impl adw::subclass::navigation_page::NavigationPageImpl for ComposeServiceEditorPage {}
}

glib::wrapper! {
    pub(crate) struct ComposeServiceEditorPage(ObjectSubclass<imp::ComposeServiceEditorPage>)
        @extends adw::NavigationPage, gtk::Widget;
}

impl ComposeServiceEditorPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
