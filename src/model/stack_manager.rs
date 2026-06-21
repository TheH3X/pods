use glib::Properties;
use glib::subclass::prelude::*;
use gtk::glib;
use gtk::gio;
use std::cell::OnceCell;

use crate::model::StackList;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::StackManager)]
    pub(crate) struct StackManager {
        #[property(get, set, construct_only)]
        pub(super) stack_list: OnceCell<StackList>,
        #[property(get, set)]
        pub(super) root_path: std::cell::RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StackManager {
        const NAME: &'static str = "StacksStackManager";
        type Type = super::StackManager;
    }

    impl ObjectImpl for StackManager {
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
    pub(crate) struct StackManager(ObjectSubclass<imp::StackManager>);
}

impl StackManager {
    pub fn new(stack_list: &StackList) -> Self {
        glib::Object::builder()
            .property("stack-list", stack_list)
            .build()
    }
}
