use gtk::glib::Properties;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::StackServiceRow)]
    #[template(string = r#"
    <interface>
      <template class="StacksStackServiceRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">8</property>
            <property name="margin-bottom">8</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>

            <!-- Dirty indicator -->
            <child>
              <object class="gtk_label" id="dirty_indicator">
                <property name="label">●</property>
                <property name="valign">center</property>
                <property name="visible">false</property>
                <property name="css-classes">accent</property>
              </object>
            </child>

            <!-- Service info -->
            <child>
              <object class="gtk_box">
                <property name="orientation">vertical</property>
                <property name="hexpand">true</property>
                <property name="spacing">2</property>
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

            <!-- Edit arrow -->
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
    pub(crate) struct StackServiceRow {
        #[template_child]
        pub(super) name_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) image_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) dirty_indicator: gtk::TemplateChild<gtk::Label>,
        #[property(get, set = Self::set_service, nullable)]
        pub(super) service: glib::WeakRef<crate::model::ComposeService>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackServiceRow {
        const NAME: &'static str = "StacksStackServiceRow";
        type Type = super::StackServiceRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StackServiceRow {
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

    impl WidgetImpl for StackServiceRow {}
    impl ListBoxRowImpl for StackServiceRow {}

    impl StackServiceRow {
        fn set_service(&self, value: Option<&crate::model::ComposeService>) {
            if self.obj().service().as_ref() == value {
                return;
            }

            if let Some(service) = value {
                service
                    .bind_property("name", &*self.name_label, "label")
                    .sync_create()
                    .build();
                service
                    .bind_property("image", &*self.image_label, "label")
                    .sync_create()
                    .build();
                service
                    .bind_property("is-dirty", &*self.dirty_indicator, "visible")
                    .sync_create()
                    .build();
            }

            self.service.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct StackServiceRow(ObjectSubclass<imp::StackServiceRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl StackServiceRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
