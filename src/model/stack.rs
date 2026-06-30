use gtk::glib::Properties;
use gtk::glib::prelude::*;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use std::cell::OnceCell;

use crate::compose::models::StackLayout;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Stack)]
    pub(crate) struct Stack {
        #[property(get, set)]
        pub(super) name: std::cell::RefCell<String>,
        #[property(get, set, construct_only)]
        pub(super) service_list: OnceCell<crate::model::ComposeServiceList>,
        #[property(get, set, construct_only)]
        pub(super) network_list: OnceCell<crate::model::DockerNetworkList>,
        #[property(get, set, nullable)]
        pub(super) root_path: std::cell::RefCell<Option<String>>,
        #[property(get, set, nullable)]
        pub(super) layout_type: std::cell::RefCell<Option<String>>,
        #[property(get, set)]
        pub(super) service_count: std::cell::Cell<u32>,
        #[property(get, set)]
        pub(super) running_count: std::cell::Cell<u32>,
        #[property(get, set)]
        pub(super) stopped_count: std::cell::Cell<u32>,
        #[property(get, set)]
        pub(super) error_count: std::cell::Cell<u32>,
        #[property(get, set)]
        pub(super) is_dirty: std::cell::Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Stack {
        const NAME: &'static str = "Stack";
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
        let obj: Self = glib::Object::builder()
            .property("name", name)
            .build();
            
        let service_list = crate::model::ComposeServiceList::new(&obj);
        obj.imp().service_list.set(service_list).unwrap();

        let network_list = crate::model::DockerNetworkList::new(&obj);
        obj.imp().network_list.set(network_list).unwrap();
        
        obj
    }

    /// Create a Stack from a compose discovery DTO.
    pub fn from_dto(dto: &crate::compose::models::Stack) -> Self {
        let obj = Self::new(&dto.name);
        obj.set_root_path(Some(dto.root_path.to_string_lossy().to_string()));
        obj.set_layout_type(Some(match dto.layout {
            StackLayout::Flat => "flat".to_string(),
            StackLayout::Nested => "nested".to_string(),
        }));
        obj.set_service_count(dto.services.len() as u32);

        obj.service_list().update_from_dtos(dto.services.clone());
        obj.network_list().update_from_dtos(dto.networks.clone());

        obj
    }

    /// Get aggregate status string for display.
    pub fn status_summary(&self) -> String {
        let total = self.service_count();
        let running = self.running_count();
        let stopped = self.stopped_count();
        let errors = self.error_count();

        if errors > 0 {
            format!("{errors} error(s)")
        } else if running == total && total > 0 {
            "All running".to_string()
        } else if stopped == total && total > 0 {
            "All stopped".to_string()
        } else if total == 0 {
            "Empty".to_string()
        } else {
            format!("{running}/{total} running")
        }
    }

    /// Update container status counts.
    pub fn update_status_counts(&self, running: u32, stopped: u32, errors: u32) {
        self.set_running_count(running);
        self.set_stopped_count(stopped);
        self.set_error_count(errors);
    }
}
