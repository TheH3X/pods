use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use sourceview5::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksPlanReviewPage" parent="adw_navigation_page">
        <child>
          <object class="gtk_box">
            <property name="orientation">vertical</property>
            <child>
              <object class="adw_header_bar">
                <child type="end">
                  <object class="gtk_button" id="deploy_button">
                    <property name="label">Deploy Plan</property>
                    <property name="css-classes">suggested-action</property>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="gtk_scrolled_window">
                <property name="vexpand">true</property>
                <child>
                  <object class="sourceview_view" id="source_view">
                    <property name="editable">false</property>
                    <property name="monospace">true</property>
                    <property name="show-line-numbers">true</property>
                  </object>
                </child>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct PlanReviewPage {
        #[template_child]
        pub(super) source_view: gtk::TemplateChild<sourceview5::View>,
        #[template_child]
        pub(super) deploy_button: gtk::TemplateChild<gtk::Button>,
        #[property(get, set = Self::set_diff, nullable)]
        pub(super) diff: std::cell::RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PlanReviewPage {
        const NAME: &'static str = "StacksPlanReviewPage";
        type Type = super::PlanReviewPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PlanReviewPage {
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

    impl WidgetImpl for PlanReviewPage {}
    impl adw::subclass::navigation_page::NavigationPageImpl for PlanReviewPage {}

    impl PlanReviewPage {
        fn set_diff(&self, value: Option<String>) {
            if let Some(ref text) = value {
                let buffer = sourceview5::Buffer::new(None);

                // Set language to diff
                if let Some(manager) = sourceview5::LanguageManager::default() {
                    if let Some(lang) = manager.language("diff") {
                        buffer.set_language(Some(&lang));
                    }
                }

                buffer.set_text(text);
                self.source_view.set_buffer(Some(&buffer));
            }
            self.diff.replace(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct PlanReviewPage(ObjectSubclass<imp::PlanReviewPage>)
        @extends adw::NavigationPage, gtk::Widget;
}

impl PlanReviewPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
