use glib::Properties;
use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::StackRow)]
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

            <!-- Stack icon -->
            <child>
              <object class="gtk_image" id="stack_icon">
                <property name="icon-name">view-grid-symbolic</property>
                <property name="pixel-size">24</property>
                <property name="valign">center</property>
              </object>
            </child>

            <!-- Name + service count -->
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
                  <object class="gtk_label" id="subtitle_label">
                    <property name="halign">start</property>
                    <property name="css-classes">dim-label caption</property>
                    <property name="ellipsize">end</property>
                  </object>
                </child>
              </object>
            </child>

            <!-- Status badges -->
            <child>
              <object class="gtk_box" id="status_box">
                <property name="orientation">horizontal</property>
                <property name="spacing">4</property>
                <property name="valign">center</property>
                <child>
                  <object class="gtk_label" id="running_badge">
                    <property name="css-classes">status-badge-small container-status-running</property>
                    <property name="visible">false</property>
                  </object>
                </child>
                <child>
                  <object class="gtk_label" id="stopped_badge">
                    <property name="css-classes">status-badge-small container-status-not-running</property>
                    <property name="visible">false</property>
                  </object>
                </child>
                <child>
                  <object class="gtk_label" id="error_badge">
                    <property name="css-classes">status-badge-small container-status-dead</property>
                    <property name="visible">false</property>
                  </object>
                </child>
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
    pub(crate) struct StackRow {
        #[template_child]
        pub(super) name_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) subtitle_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) running_badge: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) stopped_badge: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) error_badge: gtk::TemplateChild<gtk::Label>,
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
                // Bind name
                stack
                    .bind_property("name", &*self.name_label, "label")
                    .sync_create()
                    .build();

                // Build subtitle from service count and layout
                let service_count = stack.service_count();
                let layout = stack.layout_type().unwrap_or_default();
                let layout_str = if layout == "nested" { "nested" } else { "flat" };
                self.subtitle_label.set_label(&format!(
                    "{} service{} · {} layout",
                    service_count,
                    if service_count == 1 { "" } else { "s" },
                    layout_str,
                ));

                // Status badges
                let running = stack.running_count();
                let stopped = stack.stopped_count();
                let errors = stack.error_count();

                if running > 0 {
                    self.running_badge.set_label(&running.to_string());
                    self.running_badge.set_visible(true);
                }
                if stopped > 0 {
                    self.stopped_badge.set_label(&stopped.to_string());
                    self.stopped_badge.set_visible(true);
                }
                if errors > 0 {
                    self.error_badge.set_label(&errors.to_string());
                    self.error_badge.set_visible(true);
                }
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
