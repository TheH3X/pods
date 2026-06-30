use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib::Properties;
use gtk::cairo;
use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::NetworkTopologyView)]
    pub(crate) struct NetworkTopologyView {
        #[property(get, set = Self::set_stack, nullable)]
        pub(super) stack: glib::WeakRef<crate::model::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NetworkTopologyView {
        const NAME: &'static str = "StacksNetworkTopologyView";
        type Type = super::NetworkTopologyView;
        type ParentType = gtk::DrawingArea;
    }

    impl ObjectImpl for NetworkTopologyView {
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
            
            let obj = self.obj();
            obj.set_draw_func(glib::clone!(
                #[weak]
                obj,
                move |_, cr, width, height| {
                    obj.draw_topology(cr, width as f64, height as f64);
                }
            ));
        }
    }

    impl WidgetImpl for NetworkTopologyView {}
    impl DrawingAreaImpl for NetworkTopologyView {}

    impl NetworkTopologyView {
        fn set_stack(&self, value: Option<&crate::model::Stack>) {
            if self.obj().stack().as_ref() == value {
                return;
            }

            self.stack.set(value);
            self.obj().queue_draw();
        }
    }
}

glib::wrapper! {
    pub(crate) struct NetworkTopologyView(ObjectSubclass<imp::NetworkTopologyView>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NetworkTopologyView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn draw_topology(&self, cr: &cairo::Context, width: f64, height: f64) {
        // Clear background
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        cr.paint().unwrap();

        let stack = match self.stack() {
            Some(s) => s,
            None => return,
        };

        let services = stack.service_list().n_items();
        let networks = stack.network_list().n_items();

        if services == 0 && networks == 0 {
            // Draw empty state
            cr.set_source_rgba(0.5, 0.5, 0.5, 1.0);
            cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
            cr.set_font_size(14.0);
            let extents = cr.text_extents("No topology data available").unwrap();
            cr.move_to((width - extents.width()) / 2.0, height / 2.0);
            cr.show_text("No topology data available").unwrap();
            return;
        }

        // Extremely basic layout: 
        // Services on the left column, Networks on the right column.
        let service_x = width * 0.25;
        let network_x = width * 0.75;

        // Draw connections (edges) first so they are behind nodes
        // In a real app we'd iterate and draw specific connections.
        // For demonstration of the Cairo integration:
        cr.set_source_rgba(0.3, 0.3, 0.3, 1.0);
        cr.set_line_width(2.0);

        if let (sl, nl) = (stack.service_list(), stack.network_list()) {
            for i in 0..sl.n_items() {
                let sy = height * ((i as f64 + 1.0) / (services as f64 + 1.0));
                for j in 0..nl.n_items() {
                    // Just drawing full mesh for mock
                    let ny = height * ((j as f64 + 1.0) / (networks as f64 + 1.0));
                    cr.move_to(service_x, sy);
                    cr.line_to(network_x, ny);
                    cr.stroke().unwrap();
                }
            }
        }

        // Draw Service Nodes
        cr.set_source_rgba(0.2, 0.6, 1.0, 1.0); // Blue for services
        let sl = stack.service_list(); if true {
            for i in 0..services {
                let sy = height * ((i as f64 + 1.0) / (services as f64 + 1.0));
                
                cr.arc(service_x, sy, 20.0, 0.0, 2.0 * std::f64::consts::PI);
                cr.fill().unwrap();

                if let Some(obj) = sl.item(i) {
                    if let Ok(svc) = obj.downcast::<crate::model::ComposeService>() {
                        cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                        cr.set_font_size(12.0);
                        let name = svc.name();
                        let extents = cr.text_extents(&name).unwrap();
                        cr.move_to(service_x - (extents.width() / 2.0), sy - 25.0);
                        cr.show_text(&name).unwrap();
                        cr.set_source_rgba(0.2, 0.6, 1.0, 1.0);
                    }
                }
            }
        }

        // Draw Network Nodes
        cr.set_source_rgba(0.2, 0.8, 0.2, 1.0); // Green for networks
        let nl = stack.network_list(); if true {
            for j in 0..networks {
                let ny = height * ((j as f64 + 1.0) / (networks as f64 + 1.0));
                
                // Draw rounded rect or circle for networks
                cr.arc(network_x, ny, 15.0, 0.0, 2.0 * std::f64::consts::PI);
                cr.fill().unwrap();

                if let Some(obj) = nl.item(j) {
                    if let Ok(net) = obj.downcast::<crate::model::DockerNetwork>() {
                        cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                        cr.set_font_size(12.0);
                        let name = net.name();
                        let extents = cr.text_extents(&name).unwrap();
                        cr.move_to(network_x - (extents.width() / 2.0), ny - 20.0);
                        cr.show_text(&name).unwrap();
                        cr.set_source_rgba(0.2, 0.8, 0.2, 1.0);
                    }
                }
            }
        }
    }
}
