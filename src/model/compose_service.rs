use glib::Properties;
use glib::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::ComposeService)]
    pub(crate) struct ComposeService {
        #[property(get, set)]
        pub(super) name: std::cell::RefCell<String>,
        #[property(get, set, nullable)]
        pub(super) live_container: std::cell::RefCell<Option<crate::model::Container>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComposeService {
        const NAME: &'static str = "StacksComposeService";
        type Type = super::ComposeService;
    }

    impl ObjectImpl for ComposeService {
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
    pub(crate) struct ComposeService(ObjectSubclass<imp::ComposeService>);
}

impl ComposeService {
    pub fn new(name: &str) -> Self {
        glib::Object::builder()
            .property("name", name)
            .build()
    }
}
