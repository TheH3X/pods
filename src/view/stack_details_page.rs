use gtk::glib::Properties;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::StackDetailsPage)]
    #[template(string = r#"
    <interface>
      <template class="StacksStackDetailsPage" parent="adw_navigation_page">
        <child>
          <object class="gtk_box">
            <property name="orientation">vertical</property>

            <!-- Header bar with action buttons -->
            <child>
              <object class="adw_header_bar">
                <child type="end">
                  <object class="gtk_box">
                    <property name="orientation">horizontal</property>
                    <property name="spacing">6</property>
                    <child>
                      <object class="gtk_button" id="edit_button">
                        <property name="icon-name">document-edit-symbolic</property>
                        <property name="tooltip-text">Edit Stack</property>
                      </object>
                    </child>
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
                      <object class="gtk_box">
                        <property name="orientation">vertical</property>
                        <property name="spacing">24</property>
                        <property name="margin-top">24</property>
                        <property name="margin-bottom">24</property>
                        <property name="margin-start">12</property>
                        <property name="margin-end">12</property>

                        <!-- Stack info header -->
                        <child>
                          <object class="gtk_box">
                            <property name="orientation">vertical</property>
                            <property name="spacing">8</property>
                            <child>
                              <object class="gtk_label" id="stack_name_label">
                                <property name="halign">start</property>
                                <property name="css-classes">title-1</property>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_label" id="stack_path_label">
                                <property name="halign">start</property>
                                <property name="css-classes">dim-label</property>
                                <property name="ellipsize">middle</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Stack actions row -->
                        <child>
                          <object class="gtk_box">
                            <property name="orientation">horizontal</property>
                            <property name="spacing">12</property>
                            <property name="halign">start</property>
                            <child>
                              <object class="gtk_button" id="compose_up_button">
                                <property name="css-classes">suggested-action pill</property>
                                <child>
                                  <object class="gtk_box">
                                    <property name="orientation">horizontal</property>
                                    <property name="spacing">6</property>
                                    <child>
                                      <object class="gtk_image">
                                        <property name="icon-name">media-playback-start-symbolic</property>
                                      </object>
                                    </child>
                                    <child>
                                      <object class="gtk_label">
                                        <property name="label">Start</property>
                                      </object>
                                    </child>
                                  </object>
                                </child>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_button" id="compose_down_button">
                                <property name="css-classes">destructive-action pill</property>
                                <child>
                                  <object class="gtk_box">
                                    <property name="orientation">horizontal</property>
                                    <property name="spacing">6</property>
                                    <child>
                                      <object class="gtk_image">
                                        <property name="icon-name">media-playback-stop-symbolic</property>
                                      </object>
                                    </child>
                                    <child>
                                      <object class="gtk_label">
                                        <property name="label">Stop</property>
                                      </object>
                                    </child>
                                  </object>
                                </child>
                              </object>
                            </child>
                            <child>
                              <object class="gtk_button" id="compose_pull_button">
                                <property name="css-classes">pill</property>
                                <child>
                                  <object class="gtk_box">
                                    <property name="orientation">horizontal</property>
                                    <property name="spacing">6</property>
                                    <child>
                                      <object class="gtk_image">
                                        <property name="icon-name">folder-download-symbolic</property>
                                      </object>
                                    </child>
                                    <child>
                                      <object class="gtk_label">
                                        <property name="label">Pull</property>
                                      </object>
                                    </child>
                                  </object>
                                </child>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Services group -->
                        <child>
                          <object class="adw_preferences_group">
                            <property name="title">Services</property>
                            <child>
                              <object class="gtk_list_box" id="services_list_box">
                                <property name="selection-mode">none</property>
                                <property name="css-classes">boxed-list</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Networks group -->
                        <child>
                          <object class="adw_preferences_group" id="networks_group">
                            <property name="title">Networks</property>
                            <child>
                              <object class="gtk_list_box" id="networks_list_box">
                                <property name="selection-mode">none</property>
                                <property name="css-classes">boxed-list</property>
                              </object>
                            </child>
                          </object>
                        </child>

                        <!-- Topology group -->
                        <child>
                          <object class="adw_preferences_group" id="topology_group">
                            <property name="title">Topology</property>
                            <child>
                              <object class="StacksNetworkTopologyView" id="topology_view">
                                <property name="height-request">300</property>
                                <property name="css-classes">card</property>
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
    pub(crate) struct StackDetailsPage {
        #[template_child]
        pub(super) stack_name_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) stack_path_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) services_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) networks_list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) topology_view: gtk::TemplateChild<crate::view::NetworkTopologyView>,
        #[template_child]
        pub(super) compose_up_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) compose_down_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) compose_pull_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) edit_button: gtk::TemplateChild<gtk::Button>,
        #[property(get, set = Self::set_stack, nullable)]
        pub(super) stack: glib::WeakRef<crate::model::Stack>,
    }

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

    impl ObjectImpl for StackDetailsPage {
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

            // Wire up action buttons
            self.compose_up_button.connect_clicked(glib::clone!(
                #[weak(rename_to = page)]
                self,
                move |_| {
                    if let Some(stack) = page.obj().stack() {
                        if let Some(path) = stack.root_path() {
                            log::info!("Starting stack: {}", stack.name());
                            let path_clone = path.clone();
                            crate::rt::Promise::new(async move {
                                crate::compose::cli::run_compose_command(
                                    std::path::Path::new(&path_clone),
                                    crate::compose::cli::ComposeAction::Up { detach: true, build: false },
                                    None
                                ).await
                            }).defer(move |res| {
                                match res {
                                    Ok(_) => log::info!("Stack started successfully"),
                                    Err(e) => log::error!("Failed to start stack: {}", e),
                                }
                            });
                        }
                    }
                }
            ));

            self.compose_down_button.connect_clicked(glib::clone!(
                #[weak(rename_to = page)]
                self,
                move |_| {
                    if let Some(stack) = page.obj().stack() {
                        if let Some(path) = stack.root_path() {
                            log::info!("Stopping stack: {}", stack.name());
                            let path_clone = path.clone();
                            crate::rt::Promise::new(async move {
                                crate::compose::cli::run_compose_command(
                                    std::path::Path::new(&path_clone),
                                    crate::compose::cli::ComposeAction::Down { remove_volumes: false },
                                    None
                                ).await
                            }).defer(move |res| {
                                match res {
                                    Ok(_) => log::info!("Stack stopped successfully"),
                                    Err(e) => log::error!("Failed to stop stack: {}", e),
                                }
                            });
                        }
                    }
                }
            ));

            self.compose_pull_button.connect_clicked(glib::clone!(
                #[weak(rename_to = page)]
                self,
                move |_| {
                    if let Some(stack) = page.obj().stack() {
                        if let Some(path) = stack.root_path() {
                            log::info!("Pulling images for stack: {}", stack.name());
                            let path_clone = path.clone();
                            crate::rt::Promise::new(async move {
                                crate::compose::cli::run_compose_command(
                                    std::path::Path::new(&path_clone),
                                    crate::compose::cli::ComposeAction::Pull,
                                    None
                                ).await
                            }).defer(move |res| {
                                match res {
                                    Ok(_) => log::info!("Stack pulled successfully"),
                                    Err(e) => log::error!("Failed to pull stack: {}", e),
                                }
                            });
                        }
                    }
                }
            ));

            self.edit_button.connect_clicked(glib::clone!(
                #[weak(rename_to = page)]
                self,
                move |_| {
                    if let Some(stack) = page.obj().stack() {
                        log::info!("Edit stack: {}", stack.name());
                        let nav_page = crate::view::StackEditorPage::new();
                        // Assume StackEditorPage has a way to bind the stack
                        nav_page.set_property("stack", &stack);
                        crate::utils::navigation_view(&*page.obj()).push(&nav_page);
                    }
                }
            ));
        }
    }

    impl WidgetImpl for StackDetailsPage {}
    impl adw::subclass::navigation_page::NavigationPageImpl for StackDetailsPage {}

    impl StackDetailsPage {
        fn set_stack(&self, value: Option<&crate::model::Stack>) {
            if self.obj().stack().as_ref() == value {
                return;
            }

            if let Some(stack) = value {
                // Bind name to title and label
                stack
                    .bind_property("name", &*self.obj(), "title")
                    .sync_create()
                    .build();
                stack
                    .bind_property("name", &*self.stack_name_label, "label")
                    .sync_create()
                    .build();

                // Show root path
                if let Some(path) = stack.root_path() {
                    self.stack_path_label.set_label(&path);
                }

                // Bind services list
                let service_list = stack.service_list(); if true {
                    self.services_list_box.bind_model(Some(&service_list), |item| {
                        let service = item.downcast_ref::<crate::model::ComposeService>().unwrap();
                        glib::Object::builder::<crate::view::ComposeServiceSummaryRow>()
                            .property("service", service)
                            .build()
                            .upcast()
                    });
                }

                // Bind networks list
                let network_list = stack.network_list(); if true {
                    self.networks_list_box.bind_model(Some(&network_list), |item| {
                        let network = item.downcast_ref::<crate::model::DockerNetwork>().unwrap();
                        glib::Object::builder::<crate::view::NetworkRow>()
                            .property("network", network)
                            .build()
                            .upcast()
                    });
                    
                    // Show/hide networks group based on emptiness
                    let group = self.networks_list_box.parent().unwrap().downcast::<adw::PreferencesGroup>().unwrap();
                    group.set_visible(network_list.len() > 0);
                    network_list.connect_items_changed(move |model, _, _, _| {
                        group.set_visible(model.n_items() > 0);
                    });
                }

                // Bind stack to topology view
                self.topology_view.set_property("stack", stack);
            }

            self.stack.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct StackDetailsPage(ObjectSubclass<imp::StackDetailsPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl StackDetailsPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
