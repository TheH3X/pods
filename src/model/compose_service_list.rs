use glib::Properties;
use glib::subclass::prelude::*;
use gtk::glib;
use gtk::gio;
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
            self.store.set(gio::ListStore::new::<crate::model::ComposeService>());
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
        glib::Object::builder()
            .property("stack", stack)
            .build()
    }

    pub fn update_from_dtos(&self, dtos: Vec<crate::compose::models::ComposeService>) {
        let store = self.imp().store.clone();
        store.remove_all();

        for dto in dtos {
            let image = dto.image.unwrap_or_default();
            let svc = crate::model::ComposeService::new(&dto.name, &image);
            store.append(&svc);
        }
    }
}
