use std::path::PathBuf;

use gtk::glib::Properties;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::glib::prelude::*;
use std::cell::OnceCell;

use crate::model::StackList;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::StackManager)]
    pub(crate) struct StackManager {
        #[property(get, set, construct_only)]
        pub(super) stack_list: OnceCell<StackList>,
        #[property(get, set, nullable)]
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
        let obj: Self = glib::Object::builder()
            .property("stack-list", stack_list)
            .build();

        // Try loading from default path
        let default_path = glib::home_dir().join("opt").join("docker-stacks");
        if default_path.exists() {
            obj.set_root_path(Some(default_path.to_string_lossy().to_string()));
            obj.refresh();
        }

        obj
    }

    /// Rescan the root directory and update the stack list.
    pub fn refresh(&self) {
        if let Some(path_str) = self.root_path() {
            let path = PathBuf::from(path_str);
            match crate::compose::discovery::scan_root(&path) {
                Ok(stacks) => {
                    log::info!("Discovered {} stacks in {}", stacks.len(), path.display());
                    self.stack_list().update_from_scan(stacks);
                }
                Err(e) => {
                    log::error!("Failed to scan stacks directory: {}", e);
                }
            }
        }
    }

    /// Set a new root path and trigger a refresh.
    pub fn set_root_and_refresh(&self, path: &str) {
        self.set_root_path(Some(path.to_string()));
        self.refresh();
    }

    /// Check if a root path is set and valid.
    pub fn has_valid_root(&self) -> bool {
        self.root_path()
            .map(|p| PathBuf::from(p).exists())
            .unwrap_or(false)
    }
}
