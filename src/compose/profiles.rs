use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Known label profile templates for common dashboard/proxy integrations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelProfile {
    pub name: String,
    pub description: String,
    pub labels: IndexMap<String, String>,
}

/// Built-in homepage-dashboard label profiles.
pub fn homepage_profiles() -> Vec<LabelProfile> {
    vec![
        LabelProfile {
            name: "Homepage".to_string(),
            description: "gethomepage.dev dashboard labels".to_string(),
            labels: IndexMap::from([
                ("homepage.group".to_string(), "Services".to_string()),
                ("homepage.name".to_string(), "{{service_name}}".to_string()),
                (
                    "homepage.icon".to_string(),
                    "{{service_name}}.png".to_string(),
                ),
                (
                    "homepage.href".to_string(),
                    "http://{{service_name}}:{{port}}".to_string(),
                ),
                (
                    "homepage.description".to_string(),
                    "{{service_name}} service".to_string(),
                ),
            ]),
        },
        LabelProfile {
            name: "Homarr".to_string(),
            description: "Homarr dashboard labels".to_string(),
            labels: IndexMap::from([
                ("homarr.name".to_string(), "{{service_name}}".to_string()),
                (
                    "homarr.icon".to_string(),
                    "{{service_name}}.svg".to_string(),
                ),
            ]),
        },
        LabelProfile {
            name: "Organizr".to_string(),
            description: "Organizr tab labels".to_string(),
            labels: IndexMap::from([
                (
                    "organizr.tab.name".to_string(),
                    "{{service_name}}".to_string(),
                ),
                (
                    "organizr.tab.icon".to_string(),
                    "{{service_name}}".to_string(),
                ),
            ]),
        },
    ]
}

/// Built-in Traefik reverse-proxy label profiles.
pub fn traefik_profiles() -> Vec<LabelProfile> {
    vec![
        LabelProfile {
            name: "Traefik HTTP".to_string(),
            description: "Basic Traefik HTTP router".to_string(),
            labels: IndexMap::from([
                ("traefik.enable".to_string(), "true".to_string()),
                (
                    "traefik.http.routers.{{service_name}}.rule".to_string(),
                    "Host(`{{service_name}}.{{domain}}`)".to_string(),
                ),
                (
                    "traefik.http.routers.{{service_name}}.entrypoints".to_string(),
                    "web".to_string(),
                ),
                (
                    "traefik.http.services.{{service_name}}.loadbalancer.server.port".to_string(),
                    "{{port}}".to_string(),
                ),
            ]),
        },
        LabelProfile {
            name: "Traefik HTTPS".to_string(),
            description: "Traefik HTTPS router with TLS".to_string(),
            labels: IndexMap::from([
                ("traefik.enable".to_string(), "true".to_string()),
                (
                    "traefik.http.routers.{{service_name}}.rule".to_string(),
                    "Host(`{{service_name}}.{{domain}}`)".to_string(),
                ),
                (
                    "traefik.http.routers.{{service_name}}.entrypoints".to_string(),
                    "websecure".to_string(),
                ),
                (
                    "traefik.http.routers.{{service_name}}.tls".to_string(),
                    "true".to_string(),
                ),
                (
                    "traefik.http.routers.{{service_name}}.tls.certresolver".to_string(),
                    "letsencrypt".to_string(),
                ),
                (
                    "traefik.http.services.{{service_name}}.loadbalancer.server.port".to_string(),
                    "{{port}}".to_string(),
                ),
            ]),
        },
    ]
}

/// Apply a label profile to a service's labels, replacing template variables.
///
/// Template variables:
/// - `{{service_name}}` — the service name
/// - `{{port}}` — the first exposed port (or "80" default)
/// - `{{domain}}` — the domain (caller-provided)
pub fn apply_profile_labels(
    service_labels: &mut IndexMap<String, String>,
    profile: &LabelProfile,
    service_name: &str,
    port: &str,
    domain: &str,
) {
    for (key_template, val_template) in &profile.labels {
        let key = render_template(key_template, service_name, port, domain);
        let val = render_template(val_template, service_name, port, domain);
        service_labels.insert(key, val);
    }
}

/// Apply homepage-specific labels to a service.
pub fn apply_homepage_labels(
    service_labels: &mut IndexMap<String, String>,
    service_name: &str,
    port: &str,
) {
    let profiles = homepage_profiles();
    if let Some(profile) = profiles.first() {
        apply_profile_labels(service_labels, profile, service_name, port, "");
    }
}

/// Apply Traefik-specific labels to a service.
pub fn apply_traefik_labels(
    service_labels: &mut IndexMap<String, String>,
    service_name: &str,
    port: &str,
    domain: &str,
    use_https: bool,
) {
    let profiles = traefik_profiles();
    let profile = if use_https {
        profiles.get(1)
    } else {
        profiles.first()
    };

    if let Some(profile) = profile {
        apply_profile_labels(service_labels, profile, service_name, port, domain);
    }
}

/// Load all available label profiles (both built-in and custom).
pub fn load_profiles() -> Vec<LabelProfile> {
    let mut profiles = Vec::new();
    profiles.extend(homepage_profiles());
    profiles.extend(traefik_profiles());
    profiles
}

/// Replace template variables in a string.
fn render_template(template: &str, service_name: &str, port: &str, domain: &str) -> String {
    template
        .replace("{{service_name}}", service_name)
        .replace("{{port}}", port)
        .replace("{{domain}}", domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_profiles_returns_all() {
        let profiles = load_profiles();
        assert!(profiles.len() >= 4); // Homepage, Homarr, Organizr, Traefik HTTP, Traefik HTTPS
        assert!(profiles.iter().any(|p| p.name == "Homepage"));
        assert!(profiles.iter().any(|p| p.name == "Traefik HTTP"));
        assert!(profiles.iter().any(|p| p.name == "Traefik HTTPS"));
    }

    #[test]
    fn test_render_template_substitutes_all_vars() {
        let result = render_template(
            "{{service_name}}.{{domain}}:{{port}}",
            "myapp",
            "8080",
            "example.com",
        );
        assert_eq!(result, "myapp.example.com:8080");
    }

    #[test]
    fn test_apply_homepage_labels() {
        let mut labels = IndexMap::new();
        apply_homepage_labels(&mut labels, "grafana", "3000");
        assert!(labels.contains_key("homepage.name"));
        assert_eq!(labels["homepage.name"], "grafana");
        assert!(labels.contains_key("homepage.href"));
        assert!(labels["homepage.href"].contains("grafana"));
    }

    #[test]
    fn test_apply_traefik_labels_http() {
        let mut labels = IndexMap::new();
        apply_traefik_labels(&mut labels, "myapp", "8080", "example.com", false);
        assert!(labels.contains_key("traefik.enable"));
        assert_eq!(labels["traefik.enable"], "true");
        // HTTP should use "web" entrypoint, not "websecure"
        let entrypoints_key = labels
            .keys()
            .find(|k| k.contains("entrypoints"))
            .unwrap()
            .clone();
        assert_eq!(labels[&entrypoints_key], "web");
    }

    #[test]
    fn test_apply_traefik_labels_https() {
        let mut labels = IndexMap::new();
        apply_traefik_labels(&mut labels, "myapp", "443", "example.com", true);
        let entrypoints_key = labels
            .keys()
            .find(|k| k.contains("entrypoints"))
            .unwrap()
            .clone();
        assert_eq!(labels[&entrypoints_key], "websecure");
        // HTTPS should include TLS config
        assert!(labels.keys().any(|k| k.contains(".tls")));
    }

    #[test]
    fn test_profile_labels_contains_service_name() {
        let profiles = load_profiles();
        let homepage = profiles.iter().find(|p| p.name == "Homepage").unwrap();
        let mut labels = IndexMap::new();
        apply_profile_labels(&mut labels, homepage, "jellyfin", "8096", "home.local");
        // Every label value with service_name template should be substituted
        for val in labels.values() {
            assert!(!val.contains("{{service_name}}"), "Template not substituted: {}", val);
        }
        for key in labels.keys() {
            assert!(!key.contains("{{service_name}}"), "Template key not substituted: {}", key);
        }
    }
}

