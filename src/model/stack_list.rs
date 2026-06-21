use glib::Properties;
use glib::subclass::prelude::*;
use gtk::glib;
use gtk::gio;
use std::cell::OnceCell;

use crate::model::Client;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::StackList)]
    pub(crate) struct StackList {
        #[property(get, set, construct_only)]
        pub(super) client: OnceCell<glib::WeakRef<Client>>,
        // The list model
        pub(super) store: gio::ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackList {
        const NAME: &'static str = "StacksStackList";
        type Type = super::StackList;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for StackList {
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
            self.store.set(gio::ListStore::new::<crate::model::Stack>());
            
            // Wire up container tracking
            if let Some(client) = self.client.get().and_then(|w| w.upgrade()) {
                let store = self.store.clone();
                let container_list = client.container_list();
                
                container_list.connect_items_changed(move |list, _, _, _| {
                    // Update live container links
                    let n_stacks = store.n_items();
                    for i in 0..n_stacks {
                        if let Some(stack_obj) = store.item(i).and_then(|o| o.downcast::<crate::model::Stack>().ok()) {
                            // Find matching containers in list
                            let n_containers = list.n_items();
                            for j in 0..n_containers {
                                if let Some(container) = list.item(j).and_then(|o| o.downcast::<crate::model::Container>().ok()) {
                                    if let (Some(stack_name), Some(svc_name)) = (container.stack_name(), container.compose_service()) {
                                        if stack_name == stack_obj.name() {
                                            // Real implementation would look up ComposeService in Stack
                                            // and set the live container.
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    }

    impl ListModelImpl for StackList {
        fn item_type(&self) -> glib::Type {
            crate::model::Stack::static_type()
        }

        fn n_items(&self) -> u32 {
            self.store.n_items()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.store.item(position)
        }
    }
}

glib::wrapper! {
    pub(crate) struct StackList(ObjectSubclass<imp::StackList>)
        @implements gio::ListModel;
}

impl StackList {
    pub fn new(client: &Client) -> Self {
        glib::Object::builder()
            .property("client", client)
            .build()
    }
    
    pub fn update_from_scan(&self, stacks: Vec<crate::compose::models::Stack>) {
        let store = self.imp().store.clone();
        store.remove_all(); // Simple full replacement for now
        
        for dto in stacks {
            let stack_obj = crate::model::Stack::new(&dto.name);
            if let Some(path) = dto.root_path {
                stack_obj.set_root_path(Some(path.to_string_lossy().to_string()));
            }
            if let Some(service_list) = stack_obj.service_list() {
                service_list.update_from_dtos(dto.services);
            }
            store.append(&stack_obj);
        }
    }
}
