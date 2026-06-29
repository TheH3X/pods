use glib::Properties;
use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::ComposeServiceSummaryRow)]
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

            <!-- Status dot -->
            <child>
              <object class="gtk_image" id="status_icon">
                <property name="icon-name">radio-symbolic</property>
                <property name="pixel-size">12</property>
                <property name="valign">center</property>
                <property name="css-classes">dim-label</property>
              </object>
            </child>

            <!-- Name and image info -->
            <child>
              <object class="gtk_box">
                <property name="orientation">vertical</property>
                <property name="hexpand">true</property>
                <property name="spacing">2</property>
                <property name="valign">center</property>
                <child>
                  <object class="gtk_label" id="name_label">
                    <property name="halign">start</property>
                    <property name="css-classes">heading</property>
                    <property name="ellipsize">end</property>
                  </object>
                </child>
                <child>
                  <object class="gtk_label" id="image_label">
                    <property name="halign">start</property>
                    <property name="css-classes">dim-label caption</property>
                    <property name="ellipsize">end</property>
                  </object>
                </child>
              </object>
            </child>

            <!-- Port badges -->
            <child>
              <object class="gtk_label" id="ports_label">
                <property name="halign">end</property>
                <property name="valign">center</property>
                <property name="css-classes">dim-label caption</property>
                <property name="visible">false</property>
              </object>
            </child>

            <!-- Status badge -->
            <child>
              <object class="gtk_label" id="status_badge">
                <property name="halign">end</property>
                <property name="valign">center</property>
                <property name="css-classes">status-badge-small</property>
              </object>
            </child>

            <!-- Navigate arrow -->
            <child>
              <object class="gtk_image">
                <property name="icon-name">go-next-symbolic</property>
                <property name="valign">center</property>
                <property name="css-classes">dim-label</property>
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
        pub(super) ports_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) status_badge: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) status_icon: gtk::TemplateChild<gtk::Image>,
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
                // Bind name and image
                service
                    .bind_property("name", &*self.name_label, "label")
                    .sync_create()
                    .build();
                service
                    .bind_property("image", &*self.image_label, "label")
                    .sync_create()
                    .build();

                // Show ports if available
                let ports = service.ports_display();
                if !ports.is_empty() {
                    self.ports_label.set_label(&ports);
                    self.ports_label.set_visible(true);
                }

                // Bind status
                service
                    .bind_property("status-label", &*self.status_badge, "label")
                    .sync_create()
                    .build();
                service
                    .bind_property("status-css-class", &*self.status_badge, "css-classes")
                    .transform_to(|_, class: String| Some(format!("status-badge-small {class}")))
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
