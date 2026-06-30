use gtk::glib;
use gtk::glib::Properties;
use gtk::glib::prelude::*;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
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
        pub(super) plan_state: std::cell::RefCell<Option<crate::compose::plan::PlanState>>,
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
        let obj: Self = glib::Object::builder().property("name", name).build();

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
            format!("{} error(s)", errors)
        } else if running == total {
            "Running".to_string()
        } else if running > 0 {
            format!("{} running, {} stopped", running, stopped)
        } else {
            "Stopped".to_string()
        }
    }

    /// Initialize the PlanState for this stack from disk.
    pub fn init_plan_state(&self) {
        if let Some(path_str) = self.root_path() {
            let path = std::path::PathBuf::from(path_str);
            if let Ok(dto) = crate::compose::discovery::scan_stack(&path) {
                let plan = crate::compose::plan::PlanState::from_stack(&dto);
                *self.imp().plan_state.borrow_mut() = Some(plan);
                self.set_is_dirty(false);
            } else {
                log::error!("Failed to scan stack for PlanState initialization");
            }
        }
    }

    /// Execute a closure with mutable access to the PlanState.
    /// Triggers is_dirty update.
    pub fn with_plan_state<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut crate::compose::plan::PlanState) -> R,
    {
        let mut borrow = self.imp().plan_state.borrow_mut();
        if let Some(plan) = borrow.as_mut() {
            let res = f(plan);
            self.set_is_dirty(plan.is_dirty());
            Some(res)
        } else {
            None
        }
    }

    /// Execute a closure with read-only access to the PlanState.
    pub fn read_plan_state<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&crate::compose::plan::PlanState) -> R,
    {
        self.imp().plan_state.borrow().as_ref().map(f)
    }

    /// Update container status counts.
    pub fn update_status_counts(&self, running: u32, stopped: u32, errors: u32) {
        self.set_running_count(running);
        self.set_stopped_count(stopped);
        self.set_error_count(errors);
    }
}
