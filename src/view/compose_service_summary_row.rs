use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, gio};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposeServiceSummaryRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>
            <child>
              <object class="gtk_box">
                <property name="orientation">vertical</property>
                <property name="hexpand">true</property>
                <property name="spacing">4</property>
                <child>
                  <object class="gtk_label" id="name_label">
                    <property name="halign">start</property>
                    <property name="css-classes">heading</property>
                  </object>
                </child>
                <child>
                  <object class="gtk_label" id="image_label">
                    <property name="halign">start</property>
                    <property name="css-classes">dim-label</property>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="gtk_label" id="status_badge">
                <property name="halign">end</property>
                <property name="valign">center</property>
                <property name="css-classes">status-badge-small</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct ComposeServiceSummaryRow {
        #[template_child]
        pub(super) name_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) image_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) status_badge: gtk::TemplateChild<gtk::Label>,
        #[property(get, set = Self::set_service, nullable)]
        pub(super) service: glib::WeakRef<crate::model::ComposeService>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeServiceSummaryRow {
        const NAME: &'static str = "StacksComposeServiceSummaryRow";
        type Type = super::ComposeServiceSummaryRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ComposeServiceSummaryRow {
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

    impl WidgetImpl for ComposeServiceSummaryRow {}
    impl ListBoxRowImpl for ComposeServiceSummaryRow {}
    
    impl ComposeServiceSummaryRow {
        fn set_service(&self, value: Option<&crate::model::ComposeService>) {
            if self.obj().service().as_ref() == value {
                return;
            }

            if let Some(service) = value {
                service.bind_property("name", &*self.name_label, "label")
                    .sync_create()
                    .build();
                service.bind_property("image", &*self.image_label, "label")
                    .sync_create()
                    .build();
            }

            self.service.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct ComposeServiceSummaryRow(ObjectSubclass<imp::ComposeServiceSummaryRow>)
        @extends gtk::ListBoxRow, gtk::Widget;
}

impl ComposeServiceSummaryRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
