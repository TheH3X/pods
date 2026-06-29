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
    #[properties(wrapper_type = super::ComposeServiceList)]
    pub(crate) struct ComposeServiceList {
        #[property(get, set, construct_only)]
        pub(super) stack: OnceCell<glib::WeakRef<crate::model::Stack>>,
        pub(super) store: gio::ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeServiceList {
        const NAME: &'static str = "StacksComposeServiceList";
        type Type = super::ComposeServiceList;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for ComposeServiceList {
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

    impl ListModelImpl for ComposeServiceList {
        fn item_type(&self) -> glib::Type {
            crate::model::ComposeService::static_type()
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
    pub(crate) struct ComposeServiceList(ObjectSubclass<imp::ComposeServiceList>)
        @implements gio::ListModel;
}

impl ComposeServiceList {
    pub fn new(stack: &crate::model::Stack) -> Self {
        glib::Object::builder().property("stack", stack).build()
    }

    /// Replace all services from DTOs.
    pub fn update_from_dtos(&self, dtos: Vec<crate::compose::models::ComposeService>) {
        let store = &self.imp().store;
        let old_len = store.n_items();
        store.remove_all();

        for dto in &dtos {
            let svc = crate::model::ComposeService::from_dto(dto);
            store.append(&svc);
        }

        self.items_changed(0, old_len, store.n_items());
    }

    /// Find a service by name.
    pub fn find_service(&self, name: &str) -> Option<crate::model::ComposeService> {
        let store = &self.imp().store;
        for i in 0..store.n_items() {
            if let Some(obj) = store.item(i) {
                if let Ok(svc) = obj.downcast::<crate::model::ComposeService>() {
                    if svc.name() == name {
                        return Some(svc);
                    }
                }
            }
        }
        None
    }

    /// Get the number of services.
    pub fn len(&self) -> u32 {
        self.imp().store.n_items()
    }
}
