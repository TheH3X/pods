use std::path::PathBuf;

use crate::model::StackList;
use crate::config;

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
            
            // Auto-refresh when root path changes
            if pspec.name() == "root-path" {
                self.obj().refresh();
            }
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
        let obj = glib::Object::builder()
            .property("stack-list", stack_list)
            .build::<Self>();
            
        // Load default from settings or default path
        let default_path = glib::home_dir().join("opt").join("docker-stacks");
        obj.set_root_path(Some(default_path.to_string_lossy().to_string()));
        
        obj
    }

    pub fn refresh(&self) {
        if let Some(path_str) = self.root_path() {
            let path = PathBuf::from(path_str);
            let stacks = crate::compose::discovery::scan_root(&path).unwrap_or_default();
            if let Some(list) = self.stack_list() {
                list.update_from_scan(stacks);
            }
        }
    }
}
