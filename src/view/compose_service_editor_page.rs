use glib::Properties;
use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::ComposeServiceEditorPage)]
    #[template(string = r#"
    <interface>
      <template class="StacksComposeServiceEditorPage" parent="adw_navigation_page">
        <child>
          <object class="gtk_box">
            <property name="orientation">vertical</property>
            <child>
              <object class="adw_header_bar">
                <child type="end">
                  <object class="gtk_button" id="save_button">
                    <property name="label">Apply</property>
                    <property name="css-classes">suggested-action</property>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="gtk_scrolled_window">
                <property name="hscrollbar-policy">never</property>
                <property name="vexpand">true</property>
                <child>
                  <object class="adw_clamp">
                    <property name="maximum-size">800</property>
                    <child>
                      <object class="adw_preferences_page">

                        <!-- General section -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">General</property>
                            <child>
                              <object class="gtk_box">
                                <property name="orientation">horizontal</property>
                                <property name="spacing">8</property>
                                <child>
                                  <object class="adw_entry_row" id="image_entry">
                                    <property name="title">Image</property>
                                    <property name="hexpand">true</property>
                                  </object>
                                </child>
                                <child>
                                  <object class="gtk_button" id="auto_populate_button">
                                    <property name="icon-name">edit-find-symbolic</property>
                                    <property name="tooltip-text">Fetch image config and auto-populate ports/volumes</property>
                                    <property name="valign">center</property>
                                    <property name="css-classes">flat</property>
                                  </object>
                                </child>
                              </object>
                            </child>
                            <child>
                              <object class="adw_entry_row" id="container_name_entry">
                                <property name="title">Container Name</property>
                              </object>
                            </child>
                            <child>
                              <object class="adw_combo_row" id="restart_combo">
                                <property name="title">Restart Policy</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Ports section -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">Ports</property>
                            <property name="description">Map host ports to container ports</property>
                            <child>
                              <object class="gtk_list_box" id="ports_list_box">
                                <property name="selection-mode">none</property>
                                <property name="css-classes">boxed-list</property>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_button" id="add_port_button">
                                <property name="label">Add Port Mapping</property>
                                <property name="margin-top">8</property>
                                <property name="halign">start</property>
                                <property name="css-classes">flat</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Volumes section -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">Volumes</property>
                            <property name="description">Bind mounts and named volumes</property>
                            <child>
                              <object class="gtk_list_box" id="volumes_list_box">
                                <property name="selection-mode">none</property>
                                <property name="css-classes">boxed-list</property>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_button" id="add_volume_button">
                                <property name="label">Add Volume</property>
                                <property name="margin-top">8</property>
                                <property name="halign">start</property>
                                <property name="css-classes">flat</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Environment section -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">Environment</property>
                            <property name="description">Container environment variables</property>
                            <child>
                              <object class="gtk_list_box" id="env_list_box">
                                <property name="selection-mode">none</property>
                                <property name="css-classes">boxed-list</property>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_button" id="add_env_button">
                                <property name="label">Add Variable</property>
                                <property name="margin-top">8</property>
                                <property name="halign">start</property>
                                <property name="css-classes">flat</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Labels section -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">Labels</property>
                            <property name="description">Container labels (including dashboard &amp; proxy profiles)</property>
                            <child>
                              <object class="gtk_list_box" id="labels_list_box">
                                <property name="selection-mode">none</property>
                                <property name="css-classes">boxed-list</property>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_box">
                                <property name="orientation">horizontal</property>
                                <property name="spacing">8</property>
                                <property name="margin-top">8</property>
                                <child>
                                  <object class="gtk_button" id="add_label_button">
                                    <property name="label">Add Label</property>
                                    <property name="css-classes">flat</property>
                                  </object>
                                </child>
                                <child>
                                  <object class="gtk_menu_button" id="profile_menu_button">
                                    <property name="label">Apply Profile…</property>
                                    <property name="css-classes">flat</property>
                                  </object>
                                </child>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Extras section -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">Extra Fields</property>
                            <property name="description">Unmodeled compose keys (preserved on write)</property>
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
        pub(super) container_name_entry: gtk::TemplateChild<adw::EntryRow>,
        #[template_child]
        pub(super) restart_combo: gtk::TemplateChild<adw::ComboRow>,
        #[template_child]
        pub(super) ports_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) volumes_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) env_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) labels_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) extras_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) add_port_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) add_volume_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) add_env_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) add_label_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) auto_populate_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) save_button: gtk::TemplateChild<gtk::Button>,
        #[property(get, set = Self::set_service, nullable)]
        pub(super) service: glib::WeakRef<crate::model::ComposeService>,
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

    impl ObjectImpl for ComposeServiceEditorPage {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec);
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            // Set up restart policy combo
            let model = gtk::StringList::new(&["no", "always", "unless-stopped", "on-failure"]);
            self.restart_combo.set_model(Some(&model));
            self.restart_combo.set_selected(2); // Default to "unless-stopped"

            // Add port button
            let ports_list = self.ports_list_box.clone();
            self.add_port_button.connect_clicked(move |_| {
                let row = crate::view::ComposePortRow::new();
                ports_list.append(&row);
            });

            // Add volume button
            let volumes_list = self.volumes_list_box.clone();
            self.add_volume_button.connect_clicked(move |_| {
                let row = crate::view::ComposeVolumeRow::new();
                volumes_list.append(&row);
            });

            // Add env button
            let env_list = self.env_list_box.clone();
            self.add_env_button.connect_clicked(move |_| {
                let row = crate::view::ComposeEnvRow::new();
                env_list.append(&row);
            });

            // Add label button
            let labels_list = self.labels_list_box.clone();
            self.add_label_button.connect_clicked(move |_| {
                let row = crate::view::ComposeEnvRow::new(); // Reuse key=val row
                labels_list.append(&row);
            });

            // Auto-populate button
            self.auto_populate_button.connect_clicked(glib::clone!(
                #[weak(rename_to = page)]
                self,
                move |_| {
                    if let Some(service) = page.obj().service() {
                        let image = service.image().unwrap_or_default();
                        if !image.is_empty() {
                            log::info!("Auto-populating metadata for image: {}", image);
                            // In real environment, this triggers async ImageMetadata::fetch
                            // then populates ports/volumes based on the metadata.
                            
                            // Mocking the result for demonstration
                            let row = crate::view::ComposePortRow::new();
                            page.ports_list_box.append(&row);
                            
                            let v_row = crate::view::ComposeVolumeRow::with_spec("./appdata", "/data");
                            page.volumes_list_box.append(&v_row);
                        }
                    }
                }
            ));
        }
    }

    impl WidgetImpl for ComposeServiceEditorPage {}
    impl adw::subclass::navigation_page::NavigationPageImpl for ComposeServiceEditorPage {}

    impl ComposeServiceEditorPage {
        fn set_service(&self, value: Option<&crate::model::ComposeService>) {
            if self.obj().service().as_ref() == value {
                return;
            }

            if let Some(service) = value {
                // Bind title
                service
                    .bind_property("name", &*self.obj(), "title")
                    .sync_create()
                    .build();

                // Populate image entry
                service
                    .bind_property("image", &*self.image_entry, "text")
                    .sync_create()
                    .bidirectional()
                    .build();

                // Populate container name
                if let Some(cn) = service.container_name() {
                    self.container_name_entry.set_text(&cn);
                }

                // Populate restart policy
                if let Some(restart) = service.restart_policy() {
                    let idx = match restart.as_str() {
                        "no" => 0,
                        "always" => 1,
                        "unless-stopped" => 2,
                        "on-failure" => 3,
                        _ => 2,
                    };
                    self.restart_combo.set_selected(idx);
                }
            }

            self.service.set(value);
        }
    }
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
