use glib::Properties;
use glib::subclass::prelude::*;
use gtk::glib;
use gtk::gio;
use std::cell::OnceCell;
use std::path::PathBuf;

use crate::compose::models::StackLayout;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Stack)]
    pub(crate) struct Stack {
        #[property(get, set)]
        pub(super) name: std::cell::RefCell<String>,
        // Add root_path, layout, services, networks properties later
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Stack {
        const NAME: &'static str = "StacksStack";
        type Type = super::Stack;
    }

    impl ObjectImpl for Stack {
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
    pub(crate) struct Stack(ObjectSubclass<imp::Stack>);
}

impl Stack {
    pub fn new(name: &str) -> Self {
        glib::Object::builder()
            .property("name", name)
            .build()
    }
}
