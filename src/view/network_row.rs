use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib::Properties;
use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::NetworkRow)]
    #[template(string = r#"
    <interface>
      <template class="StacksNetworkRow" parent="gtk_list_box_row">
        <child>
          <object class="gtk_box">
            <property name="orientation">horizontal</property>
            <property name="spacing">12</property>
            <property name="margin-top">12</property>
            <property name="margin-bottom">12</property>
            <property name="margin-start">12</property>
            <property name="margin-end">12</property>

            <!-- Network icon -->
            <child>
              <object class="gtk_image">
                <property name="icon-name">network-workgroup-symbolic</property>
                <property name="pixel-size">24</property>
                <property name="valign">center</property>
                <property name="css-classes">dim-label</property>
              </object>
            </child>

            <!-- Name and info -->
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
                  <object class="gtk_box">
                    <property name="orientation">horizontal</property>
                    <property name="spacing">6</property>
                    <child>
                      <object class="gtk_label" id="driver_label">
                        <property name="halign">start</property>
                        <property name="css-classes">dim-label caption</property>
                      </object>
                    </child>
                    <child>
                      <object class="gtk_label" id="subnet_label">
                        <property name="halign">start</property>
                        <property name="css-classes">dim-label caption</property>
                        <property name="visible">false</property>
                      </object>
                    </child>
                  </object>
                </child>
              </object>
            </child>

            <!-- External badge -->
            <child>
              <object class="gtk_label" id="external_badge">
                <property name="label">External</property>
                <property name="halign">end</property>
                <property name="valign">center</property>
                <property name="css-classes">status-badge-small</property>
                <property name="visible">false</property>
              </object>
            </child>

            <!-- Connected services count -->
            <child>
              <object class="gtk_label" id="connected_services_label">
                <property name="halign">end</property>
                <property name="valign">center</property>
                <property name="css-classes">dim-label</property>
                <property name="visible">false</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct NetworkRow {
        #[template_child]
        pub(super) name_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) driver_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) subnet_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) external_badge: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) connected_services_label: gtk::TemplateChild<gtk::Label>,
        #[property(get, set = Self::set_network, nullable)]
        pub(super) network: glib::WeakRef<crate::model::DockerNetwork>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NetworkRow {
        const NAME: &'static str = "StacksNetworkRow";
        type Type = super::NetworkRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NetworkRow {
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

    impl WidgetImpl for NetworkRow {}
    impl ListBoxRowImpl for NetworkRow {}

    impl NetworkRow {
        fn set_network(&self, value: Option<&crate::model::DockerNetwork>) {
            if self.obj().network().as_ref() == value {
                return;
            }

            if let Some(network) = value {
                network.bind_property("name", &*self.name_label, "label")
                    .sync_create()
                    .build();

                // Format driver
                if let Some(driver) = network.driver() {
                    self.driver_label.set_label(&format!("Driver: {}", driver));
                } else {
                    self.driver_label.set_label("Driver: default");
                }

                // Format subnet
                if let Some(subnet) = network.subnet() {
                    self.subnet_label.set_label(&format!("· Subnet: {}", subnet));
                    self.subnet_label.set_visible(true);
                }

                // External badge
                if network.is_external() {
                    self.external_badge.set_visible(true);
                }

                // Connected services
                network.bind_property("connected-services-display", &*self.connected_services_label, "label")
                    .sync_create()
                    .build();
                network.bind_property("connected-services-display", &*self.connected_services_label, "visible")
                    .transform_to(|_, text: String| Some(!text.is_empty()))
                    .sync_create()
                    .build();
            }

            self.network.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct NetworkRow(ObjectSubclass<imp::NetworkRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl NetworkRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
