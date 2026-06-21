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
}
