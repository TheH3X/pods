use gtk::glib::Properties;
use gtk::glib::subclass::prelude::\*;
use gio::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use std::cell::OnceCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::DockerNetworkList)]
    pub(crate) struct DockerNetworkList {
        #[property(get, set, construct_only)]
        pub(super) stack: OnceCell<glib::WeakRef<crate::model::Stack>>,
        pub(super) store: gio::ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DockerNetworkList {
        const NAME: &'static str = "StacksDockerNetworkList";
        type Type = super::DockerNetworkList;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for DockerNetworkList {
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

    impl ListModelImpl for DockerNetworkList {
        fn item_type(&self) -> glib::Type {
            crate::model::DockerNetwork::static_type()
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
    pub(crate) struct DockerNetworkList(ObjectSubclass<imp::DockerNetworkList>)
        @implements gio::ListModel;
}

impl DockerNetworkList {
    pub fn new(stack: &crate::model::Stack) -> Self {
        glib::Object::builder()
            .property("stack", stack)
            .build()
    }

    pub fn update_from_dtos(&self, dtos: Vec<crate::compose::models::Network>) {
        let store = &self.imp().store;
        let old_len = store.n_items();
        store.remove_all();

        for dto in &dtos {
            let net = crate::model::DockerNetwork::from_dto(dto);
            store.append(&net);
        }

        self.items_changed(0, old_len, store.n_items());
    }

    pub fn len(&self) -> u32 {
        self.imp().store.n_items()
    }
}
