use gtk::glib::Properties;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

use crate::model::Client;

mod imp {
    use super::*;

    #[derive(Debug, Properties)]
    #[properties(wrapper_type = super::StackList)]
    pub(crate) struct StackList {
        #[property(get, set, construct_only, nullable)]
        pub(super) client: glib::WeakRef<Client>,
        // The list model
        pub(super) store: gio::ListStore,
    }

    impl Default for StackList {
        fn default() -> Self {
            Self {
                client: glib::WeakRef::new(),
                store: gio::ListStore::new::<crate::model::Stack>(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackList {
        const NAME: &'static str = "StackList";
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
        glib::Object::builder().property("client", client).build()
    }

    /// Replace all stacks from a scan result.
    pub fn update_from_scan(&self, stacks: Vec<crate::compose::models::Stack>) {
        let store = &self.imp().store;
        let old_len = store.n_items();
        store.remove_all();

        for dto in &stacks {
            let stack_obj = crate::model::Stack::from_dto(dto);
            store.append(&stack_obj);
        }

        // Notify the ListModel about the change
        self.items_changed(0, old_len, store.n_items());
    }

    /// Find a stack by name.
    pub fn find_stack(&self, name: &str) -> Option<crate::model::Stack> {
        let store = &self.imp().store;
        for i in 0..store.n_items() {
            if let Some(obj) = store.item(i) {
                if let Ok(stack) = obj.downcast::<crate::model::Stack>() {
                    if stack.name() == name {
                        return Some(stack);
                    }
                }
            }
        }
        None
    }

    /// Get the number of stacks.
    pub fn len(&self) -> u32 {
        self.imp().store.n_items()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
