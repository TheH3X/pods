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
        #[property(get, set)]
        pub(super) image: std::cell::RefCell<String>,
        #[property(get, set, nullable)]
        pub(super) container_name: std::cell::RefCell<Option<String>>,
        #[property(get, set, nullable)]
        pub(super) restart_policy: std::cell::RefCell<Option<String>>,
        #[property(get, set)]
        pub(super) ports_display: std::cell::RefCell<String>,
        #[property(get, set)]
        pub(super) volumes_display: std::cell::RefCell<String>,
        #[property(get, set)]
        pub(super) status_label: std::cell::RefCell<String>,
        #[property(get, set)]
        pub(super) status_css_class: std::cell::RefCell<String>,
        #[property(get, set)]
        pub(super) is_dirty: std::cell::Cell<bool>,
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
    pub fn new(name: &str, image: &str) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("image", image)
            .build()
    }

    /// Create from a compose DTO model, formatting display properties.
    pub fn from_dto(dto: &crate::compose::models::ComposeService) -> Self {
        let image = dto.image.clone().unwrap_or_default();
        let obj = Self::new(&dto.name, &image);

        obj.set_container_name(dto.container_name.clone());
        obj.set_restart_policy(dto.restart.clone());

        // Format ports for display
        if !dto.ports.is_empty() {
            obj.set_ports_display(dto.ports.join(", "));
        }

        // Format volumes for display
        if !dto.volumes.is_empty() {
            obj.set_volumes_display(
                dto.volumes
                    .iter()
                    .map(|v| {
                        // Shorten long paths
                        if v.len() > 40 {
                            format!("...{}", &v[v.len() - 37..])
                        } else {
                            v.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }

        // Default status
        obj.set_status_label("unknown".to_string());
        obj.set_status_css_class("dim-label".to_string());

        obj
    }

    /// Update the live status based on a linked container.
    pub fn update_live_status(&self) {
        if let Some(_container) = self.live_container() {
            // When live cross-referencing is wired:
            // Use container.status() to set status_label and CSS class
            self.set_status_label("linked".to_string());
            self.set_status_css_class("container-status-running".to_string());
        } else {
            self.set_status_label("no container".to_string());
            self.set_status_css_class("dim-label".to_string());
        }
    }
}
