use gtk::glib;
use gtk::glib::Properties;
use gtk::glib::prelude::*;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::DockerNetwork)]
    pub(crate) struct DockerNetwork {
        #[property(get, set)]
        pub(super) name: std::cell::RefCell<String>,
        #[property(get, set, nullable)]
        pub(super) driver: std::cell::RefCell<Option<String>>,
        #[property(get, set, nullable)]
        pub(super) subnet: std::cell::RefCell<Option<String>>,
        #[property(get, set)]
        pub(super) is_external: std::cell::Cell<bool>,
        #[property(get, set)]
        pub(super) connected_services_display: std::cell::RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DockerNetwork {
        const NAME: &'static str = "DockerNetwork";
        type Type = super::DockerNetwork;
    }

    impl ObjectImpl for DockerNetwork {
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
}

glib::wrapper! {
    pub(crate) struct DockerNetwork(ObjectSubclass<imp::DockerNetwork>);
}

impl DockerNetwork {
    pub fn new(name: &str) -> Self {
        glib::Object::builder().property("name", name).build()
    }

    /// Create from a compose DTO network model.
    pub fn from_dto(dto: &crate::compose::models::Network) -> Self {
        let obj = Self::new(&dto.name);
        obj.set_driver(dto.driver.clone());
        obj.set_is_external(dto.external.unwrap_or(false));
        obj.set_subnet(dto.subnet());
        obj
    }
}
