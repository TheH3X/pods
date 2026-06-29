use gtk::glib::prelude::\*;
use gtk::glib::subclass::prelude::\*;
use gtk::glib::subclass::prelude::\*;
use adw::subclass::prelude::\*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::model::StackList;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::StacksPanel)]
    #[template(string = r#"
    <interface>
      <template class="StacksStacksPanel" parent="gtk_box">
        <property name="orientation">vertical</property>

        <!-- Empty state / Open folder prompt -->
        <child>
          <object class="adw_status_page" id="empty_page">
            <property name="icon-name">view-grid-symbolic</property>
            <property name="title">No Stacks Found</property>
            <property name="description">Open a docker-stacks directory to get started</property>
            <property name="vexpand">true</property>
            <child>
              <object class="gtk_box">
                <property name="orientation">vertical</property>
                <property name="spacing">12</property>
                <property name="halign">center</property>
                <child>
                  <object class="gtk_button" id="open_folder_button">
                    <property name="label">Open Folder…</property>
                    <property name="css-classes">suggested-action pill</property>
                  </object>
                </child>
                <child>
                  <object class="gtk_button" id="new_stack_button">
                    <property name="label">New Stack</property>
                    <property name="css-classes">pill</property>
                  </object>
                </child>
              </object>
            </child>
          </object>
        </child>

        <!-- Stack list view -->
        <child>
          <object class="gtk_box" id="list_container">
            <property name="orientation">vertical</property>
            <property name="visible">false</property>
            <child>
              <object class="gtk_box">
                <property name="orientation">horizontal</property>
                <property name="spacing">6</property>
                <property name="margin-start">12</property>
                <property name="margin-end">12</property>
                <property name="margin-top">6</property>
                <property name="margin-bottom">6</property>
                <child>
                  <object class="gtk_search_entry" id="search_entry">
                    <property name="hexpand">true</property>
                    <property name="placeholder-text">Search stacks…</property>
                  </object>
                </child>
                <child>
                  <object class="gtk_button" id="refresh_button">
                    <property name="icon-name">view-refresh-symbolic</property>
                    <property name="tooltip-text">Refresh</property>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="gtk_scrolled_window">
                <property name="hscrollbar-policy">never</property>
                <property name="vexpand">true</property>
                <child>
                  <object class="gtk_list_box" id="list_box">
                    <property name="selection-mode">none</property>
                    <property name="css-classes">boxed-list</property>
                    <property name="margin-start">12</property>
                    <property name="margin-end">12</property>
                    <property name="margin-top">6</property>
                    <property name="margin-bottom">12</property>
                  </object>
                </child>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct StacksPanel {
        #[template_child]
        pub(super) list_box: gtk::TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) empty_page: gtk::TemplateChild<adw::StatusPage>,
        #[template_child]
        pub(super) list_container: gtk::TemplateChild<gtk::Box>,
        #[template_child]
        pub(super) search_entry: gtk::TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub(super) open_folder_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) new_stack_button: gtk::TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) refresh_button: gtk::TemplateChild<gtk::Button>,
        #[property(get, set)]
        pub(super) stack_list: glib::WeakRef<crate::model::StackList>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StacksPanel {
        const NAME: &'static str = "StacksStacksPanel";
        type Type = super::StacksPanel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StacksPanel {
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

            let obj = self.obj().clone();

            // Open folder button
            self.open_folder_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    log::info!("Open folder clicked — showing file chooser");
                    let dialog = gtk::FileDialog::builder()
                        .title("Select docker-stacks root directory")
                        .modal(true)
                        .build();
                    
                    dialog.select_folder(
                        obj.root().and_then(|r| r.downcast::<gtk::Window>().ok()).as_ref(),
                        gio::Cancellable::NONE,
                        glib::clone!(
                            move |result| {
                                if let Ok(folder) = result {
                                    if let Some(path) = folder.path() {
                                        log::info!("Selected stacks root: {:?}", path);
                                        // In a real app we'd save this to GSettings
                                        // and then trigger StackManager to scan the root.
                                    }
                                }
                            }
                        ),
                    );
                }
            ));

            // New stack button
            self.new_stack_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    log::info!("New stack clicked — showing create dialog");
                    let nav_page = crate::view::StackEditorPage::new();
                    crate::utils::navigation_view(&obj).push(&nav_page);
                }
            ));

            // Refresh button
            self.refresh_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    log::info!("Refresh stacks");
                    if let Some(_list) = obj.stack_list() {
                        // list.refresh() would be called here.
                        // For MVP, we just log it since the backend poll isn't running.
                    }
                }
            ));
        }
    }

    impl WidgetImpl for StacksPanel {}
    impl BoxImpl for StacksPanel {}

    impl StacksPanel {
        fn set_stack_list(&self, value: Option<&StackList>) {
            if self.stack_list.upgrade().as_ref() == value {
                return;
            }

            self.stack_list.set(value);
            if let Some(list) = value {
                let has_items = list.n_items() > 0;
                self.empty_page.set_visible(!has_items);
                self.list_container.set_visible(has_items);

                self.list_box.bind_model(Some(list), |item| {
                    let stack = item.downcast_ref::<crate::model::Stack>().unwrap();
                    glib::Object::builder::<crate::view::StackRow>()
                        .property("stack", stack)
                        .build()
                        .upcast()
                });

                // Update visibility when items change
                let empty_page = self.empty_page.clone();
                let list_container = self.list_container.clone();
                list.connect_items_changed(move |model, _, _, _| {
                    let has_items = model.n_items() > 0;
                    empty_page.set_visible(!has_items);
                    list_container.set_visible(has_items);
                });
            }

            self.stack_list.set(value);
        }
    }
}

glib::wrapper! {
    pub(crate) struct StacksPanel(ObjectSubclass<imp::StacksPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl StacksPanel {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
