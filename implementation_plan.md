# Stacks — Implementation Plan
## (Fork of `TheH3X/pods` → App renamed "Stacks")

---

## Finalized Design Decisions

| # | Question | Decision |
|---|----------|----------|
| Q1 | Stack-based vision principles | **Confirmed**: per-service dirs, plan/deploy buffer, raw-merge, appdata portability, label profiles |
| Q2 | Target scope | **C — Everything is a stack**: single containers are still treated as single-service stacks |
| Q3 | Language | **Hybrid (C)**: Rust for all models/UI/discovery; shell to `docker compose` CLI for stack lifecycle ops. Best for upstream Pods sync |
| Q4 | App name | **"Stacks"** (repo stays `TheH3X/pods` as the git source) |
| Q5 | Features | **All except validation policies**. See details below |
| Q6 | Engine support | **Docker only for now**: Podman/Docker API for runtime (start/stop/inspect/logs/terminal); `docker compose` CLI for stack-level ops (up/down/pull) |

### Q5 Feature Details

| Feature | In scope | Notes |
|---------|----------|-------|
| Per-service nested directory layout | ✅ | Core philosophy |
| Flat (monolithic) compose file support | ✅ | Required for existing single-file stacks |
| Plan/Deploy in-memory buffer | ✅ | Core philosophy |
| Bind mount normalization → appdata/ | **Partially** | Each service dir must contain all its binds/mounts for portability. No forced rewriting — present as a migration suggestion |
| Homepage label profiles | ✅ | |
| Traefik label profiles | ✅ | |
| Registry browser (Docker Hub, GHCR) | ✅ | Leverage Pods' existing image search |
| Image metadata introspection | ✅ | Pods already has OCI/registry data access; surface it in service editor |
| Filesystem tree visualization | ✅ | |
| Network editor | ✅ | **Scoped to Docker networks**: visualize inter-service communication flows + external port exposure. Subnet/IP assignment |
| YAML diff view (plan vs. disk) | ✅ | |
| Service CRUD | ✅ | Add/remove/rename/duplicate |
| Validation policies | ❌ | Out of scope |
| Backup/snapshots on deploy | ✅ | |

---

## Architecture Overview

### "Everything is a Stack" Paradigm

In Stacks, there are no free-floating containers. The runtime world is organized:

```
Stack "monitoring"              Docker engine (live)
├── grafana/                ←→  monitoring-grafana-1      [running ✓]
│   ├── compose.yml             monitoring-grafana-1      [healthy]
│   └── appdata/
├── prometheus/             ←→  monitoring-prometheus-1   [running ✓]
│   ├── compose.yml
│   └── appdata/
└── alertmanager/           ←→  monitoring-alertmanager-1 [exited ✗ code=1]
    ├── compose.yml

Stack "web" (FLAT layout)   ←→  web-nginx-1               [running ✓]
└── docker-compose.yml          web-php-1                  [running ✓]

Orphan containers                my-test-container         [running]
(no compose definition found)    → Prompt: "Import into a stack?"
```

Orphan containers (detected, not part of any discovered stack) are shown in a special "Unmanaged" bucket and the user is offered to import them into a new or existing stack.

### System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    GTK4 UI (Rust + Blueprint)                   │
│                                                                 │
│  Window                                                         │
│  └── NavigationSplitView                                        │
│      ├── Sidebar: [Stacks] [Images] [Volumes] [Info]            │
│      └── Content area:                                          │
│          ├── StacksPanel (list of all stacks)                   │
│          │   └── StackDetailsPage                               │
│          │       ├── ServicesGroup (live status per service)    │
│          │       ├── NetworksGroup (topology visualization)     │
│          │       └── ActionsGroup (up/down/pull/logs)           │
│          ├── StackEditorPage (plan/deploy workflow)             │
│          │   ├── ComposeServiceEditorPage                       │
│          │   ├── PlanReviewPage (YAML diff)                     │
│          │   └── FilesystemTreePage                             │
│          ├── ImagesPanel (existing Pods, enhanced)              │
│          └── VolumesPanel (existing Pods)                       │
└─────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────┐
│                     Model Layer (Rust GObjects)                  │
│                                                                 │
│  Client (existing)                                              │
│  ├── ContainerList (existing) ←──cross-ref──→ ComposeServiceList│
│  ├── ImageList (existing)     ←──lookup──→    ComposeService    │
│  └── StackManager (NEW)                                         │
│      └── StackList → Stack → ComposeServiceList → ComposeService│
│                         └── NetworkList → DockerNetwork         │
│                         └── PlanState (in-memory edit buffer)   │
└─────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────┐
│                    Engine/Compose Layer (Rust)                   │
│                                                                 │
│  Engine (existing: Docker API via Bollard)                      │
│  ├── containers() / images() / volumes()  ← runtime ops        │
│  └── ComposeEngine (NEW)                                        │
│      ├── discovery::scan_root()    → finds docker-stacks trees  │
│      ├── yaml_io::read_stack()     → parse compose YAML         │
│      ├── yaml_io::write_stack()    → write with raw-merge       │
│      ├── plan::PlanState           → in-memory diff/edit buffer │
│      ├── normalization::normalize()→ appdata portability check  │
│      ├── profiles::apply_labels()  → homepage/traefik labels    │
│      ├── deploy::write_to_disk()   → atomic file writes         │
│      └── cli::run_compose()        → shells `docker compose`    │
└─────────────────────────────────────────────────────────────────┘
```

### Why This Approach Maximizes Upstream Sync

- **Upstream Pods changes** to `src/engine/`, `src/model/`, `src/view/` affect containers, images, pods, volumes — our stack code lives in `src/compose/` (new directory) and `src/model/stack*.rs`, `src/view/stack*.rs`, `src/view/compose_*.rs`
- **Merge strategy**: Git rebase/merge of upstream Pods commits will have minimal conflicts since we add, not modify, the majority of code
- **Touch points** that will conflict with upstream: `src/view/client_view.{rs,blp}` (add Stacks to sidebar), `src/model/client.rs` (add stack_manager), `src/view/window.rs` (app name), `src/application.rs` (app id/name). These are small and well-defined

---

## Phase 0 — Fork Setup & App Identity

**Goal**: Clean up the repo to reflect the new app identity without breaking any existing functionality.

### Tasks

#### [MODIFY] `src/application.rs`
- Change app display name to "Stacks"
- Update about dialog (copyright, developers)

#### [MODIFY] `src/config.rs.in`
- Update `APP_ID` to `com.github.TheH3X.Stacks` (or keep Pods ID for now — see Q4 note)
- Update `APP_NAME`

#### [MODIFY] `README.md`
- New README describing "Stacks" — its purpose, the stack-based philosophy, installation

#### [MODIFY] `meson.build`, `data/` appstream, desktop files
- Update app name, description, icon references

#### [MODIFY] `Cargo.toml`
- Update `name = "stacks"`, authors, version → `0.1.0`

#### Remove unneeded Pods-specific identity files
- `.typos.toml`, `Pods.doap` → replaced with Stacks equivalents
- Translation files (po/) — reset to en-only for now

---

## Phase 1 — Compose Module: Stack Discovery & Models

**Goal**: Port compose-generator's core data model and filesystem discovery to Rust. No UI yet.

### New Module: `src/compose/`

```
src/compose/
├── mod.rs          — module root, re-exports
├── models.rs       — Stack, ComposeService, Network, BindMount, PortMapping, EnvVar, BindRewrite, LabelAddition
├── discovery.rs    — scan_root(), scan_stack(), focused_stack(), detect_layout (FLAT/NESTED)
├── yaml_io.rs      — read_compose_file(), write_compose_file() with raw-value preservation
├── normalization.rs— check_bind_portability(), suggest_appdata_moves()
├── profiles.rs     — apply_homepage_labels(), apply_traefik_labels(), load_profiles()
├── plan.rs         — PlanState, ServiceEdit, dirty tracking, unified diff generation
├── deploy.rs       — write_to_disk(), build_service_doc(), build_stack_doc(), build_manifest()
├── backup.rs       — create_snapshot(), prune_old_snapshots()
├── cli.rs          — run_compose_command() (shells to `docker compose`)
├── registry.rs     — DockerHub v2 API, GHCR API, metadata cache
└── image_meta.rs   — OCI config parsing (declared volumes, exposed ports, env defaults)
```

#### Key Design: YAML Round-Tripping

Unlike Python's `ruamel.yaml`, Rust's `serde_yaml` does not preserve comments. Strategy:

1. **Read**: Parse with `serde_yaml` into `serde_json::Value` (preserving unknown keys in an `extras: Map<String, Value>` field)
2. **Write**: Serialize back to YAML, with a deterministic key order (modeled keys first, extras appended verbatim)
3. **Comments**: Emit a header comment `# Managed by Stacks — do not edit modeled keys manually` so users know which parts are managed
4. **Raw-merge**: Unknown compose keys stored in `extras` are always written back unchanged

This gives us ~95% of `ruamel.yaml`'s behavior (no inline comment preservation, but full structure preservation).

### GObject Model Wrappers

| File | GObject | Properties |
|------|---------|-----------|
| `src/model/stack.rs` | `Stack` | name, root_path, layout, services (ListModel), networks (ListModel), is_dirty, plan_state |
| `src/model/stack_list.rs` | `StackList` | client (weak), items via ListModel |
| `src/model/compose_service.rs` | `ComposeService` | name, folder, image, container_name, restart, ports, volumes, networks, labels, extras, is_dirty, live_container (weak, cross-ref) |
| `src/model/compose_service_list.rs` | `ComposeServiceList` | stack (weak), items |
| `src/model/stack_manager.rs` | `StackManager` | root_path, stack_list, cfg (AppConfig) |
| `src/model/docker_network.rs` | `DockerNetwork` | name, driver, subnet, external, connected_services |

### `src/model/client.rs` Changes
```rust
// Add to Client struct:
#[property(get = Self::stack_manager, nullable)]
pub(super) stack_manager: OnceCell<Option<model::StackManager>>,
```
Stack manager is optional — if no docker-stacks root is found, the Stacks panel shows a "Open folder" prompt.

---

## Phase 2 — Stacks Panel: List & Status View

**Goal**: Add "Stacks" as the primary panel in the sidebar. Shows discovered stacks with live container status indicators.

### Sidebar Rework

Current sidebar: `[Containers] [Pods] [Images] [Volumes] [Info]`

New sidebar (Docker-only, stack-first):
```
[Stacks]   ← primary, selected by default
[Images]   ← existing Pods panel  
[Volumes]  ← existing Pods panel
[Info]     ← existing Pods panel
```

> Note: "Containers" and "Pods" panels are replaced by the Stacks view. Individual containers are accessible by drilling into a stack → service. Orphan containers (no stack) appear in Stacks panel under an "Unmanaged" group.

### New View Files

| File | Description |
|------|------------|
| `src/view/stacks_panel.{rs,blp}` | Panel: search bar, list of stacks, "Open folder" / "New stack" actions |
| `src/view/stack_row.{rs,blp}` | Row: stack name, service count, aggregate status (all running / N stopped / has error) |
| `src/view/stack_details_page.{rs,blp}` | Detail page: services group (with live badges), network topology, stack actions |
| `src/view/compose_service_summary_row.{rs,blp}` | Row: service name, image, live status badge (reuses Pods' ContainerStatus), port badges |

### Live Status in Stack View

Each `ComposeService` GObject has a `live_container` weak-ref property populated during the cross-reference pass. Status badge mirrors Pods' existing container status icons/CSS classes.

### Modified Files

| File | Change |
|------|--------|
| `src/view/client_view.{rs,blp}` | Replace sidebar [0]=Containers, [1]=Pods with [0]=Stacks; remove containers_panel, pods_panel template children; add stacks_panel |
| `src/view/mod.rs` | Export new view types |

---

## Phase 3 — Stack Editor & Plan/Deploy Workflow

**Goal**: Full CRUD + plan/deploy. This is the core compose-generator philosophy ported to GTK4/Rust.

### New View Files

| File | Description |
|------|------------|
| `src/view/stack_editor_page.{rs,blp}` | Stack editor: service list (add/remove/reorder), network editor tab, plan review tab |
| `src/view/compose_service_editor_page.{rs,blp}` | Service editor: image (with registry browser), ports, volumes, env vars, labels, extra fields |
| `src/view/plan_review_page.{rs,blp}` | Plan review: per-service diff view (using SourceView5), file change tree, Deploy button |
| `src/view/add_service_page.{rs,blp}` | Add service wizard: image search → name → auto-populated ports/volumes from OCI metadata |
| `src/view/compose_volume_row.{rs,blp}` | Inline bind mount editor row (host path, container path, options) |
| `src/view/compose_port_row.{rs,blp}` | Inline port mapping editor row |
| `src/view/compose_env_row.{rs,blp}` | Inline env var row |
| `src/view/compose_extra_field_row.{rs,blp}` | Inline extra-field row (unmodeled keys) |
| `src/view/filesystem_tree_page.{rs,blp}` | Expandable directory tree with bind mount annotations |
| `src/view/yaml_editor_page.{rs,blp}` | Read/write raw YAML editor using SourceView5 (existing widget already used in Pods for logs) |

### Plan/Deploy UX Flow

```
Stack Details Page
  ↓ [Edit Stack] button
Stack Editor Page  (PlanState created in memory)
  ├── Services list (dirty services show ● indicator)
  │   └── [Click service] → Service Editor Page
  │       ├── Image row (→ Registry Browser on click)
  │       ├── Ports expandable group
  │       ├── Volumes expandable group (→ Filesystem Tree on 📁 click)
  │       ├── Environment expandable group
  │       ├── Labels expandable group (→ Label Profile picker)
  │       └── Extra Fields expandable group
  ├── Networks tab → Network Editor / Topology View
  └── [Review Plan] button → Plan Review Page
      ├── Change summary (N services modified, N added, N removed)
      ├── Per-service YAML diff (SourceView5 diff syntax highlighting)
      ├── File change tree
      └── [Deploy] button → writes to disk + optionally runs `docker compose up`
```

---

## Phase 4 — Network Editor with Topology Visualization

**Goal**: A Docker-networks-aware editor that visualizes communication between services and to the outside world.

### Network Topology Canvas

Using `gtk::DrawingArea` or `gtk::GLArea` + Cairo:

```
┌─────────────────────────────────────────────────────┐
│  Network: proxy_net          driver: bridge          │
│  Subnet: 172.20.0.0/24                               │
│                                                      │
│   [traefik :80 :443]──────────[nginx]                │
│        │ :80 :443 (external)       │                  │
│        ▼                           └──[php-fpm]      │
│   ● Internet                                         │
│                                                      │
│  Network: db_net             driver: bridge          │
│   [nginx]──────────[postgres]                        │
│                         │ :5432 (internal only)      │
└─────────────────────────────────────────────────────┘
```

**Visualization elements**:
- **Service nodes**: rounded rectangles, colored by status (green/red/grey)
- **Network edges**: lines connecting services sharing a network
- **Port badges**: shown on service nodes — external ports (visible to host) vs. internal (exposed to other services only)
- **External access indicator**: arrow pointing outward for host-exposed ports
- **Subnet label**: shown per network group

### Network Editor Rows

Alongside the topology canvas, a structured editor:
- Add/remove Docker networks (name, driver, subnet)
- Mark networks as external
- Assign services to networks
- Set per-service static IPs (ipv4_address)

---

## Phase 5 — Image Metadata & Registry Integration

**Goal**: Leverage Pods' existing image infrastructure to enhance service creation.

### Enhance Existing Pods Image Views

Pods already has:
- `ImageRemoteSelectionPage` — search remote registries
- `ImageSearchResponse` model
- `ImageDetails` / `ImageDetailsPage`

**Additions**:

| Enhancement | Description |
|-------------|-------------|
| OCI config surface | In `ComposeServiceEditorPage`: "Image declares these volumes: /data, /config — map them?" |
| Auto-populate | "Auto-populate" button creates bind entries for all declared volumes → mapped to `<service>/appdata/...` |
| Port suggestions | Show declared `ExposedPorts` from image config with "add mapping" buttons |
| Env defaults | Show image-level env vars as suggestions in the env editor |
| Stars | Reuse compose-manager's starred images in registry browser |

### `src/compose/image_meta.rs`

```rust
pub struct ImageMetadata {
    pub declared_volumes: Vec<String>,    // from OCI Config.Volumes
    pub exposed_ports: Vec<String>,       // from OCI Config.ExposedPorts  
    pub env_defaults: HashMap<String, String>, // from OCI Config.Env
    pub working_dir: String,
    pub labels: HashMap<String, String>,  // image-level labels
}

impl ImageMetadata {
    pub async fn from_docker_inspect(client: &bollard::Docker, image: &str) -> Result<Self>;
    pub async fn from_registry_api(image: &str, tag: &str) -> Result<Self>;
}
```

---

## Phase 6 — Live Cross-Referencing

**Goal**: Link running containers back to their compose service definitions.

### Cross-Reference Pass

After both `ContainerList` and `StackList` are loaded, run a reconciliation:

```rust
fn reconcile_stacks_and_containers(
    stack_list: &model::StackList,
    container_list: &model::ContainerList,
) {
    // Match by: container.labels["com.docker.compose.project"] + 
    //           container.labels["com.docker.compose.service"]
    // Docker compose sets these labels automatically
    for container in container_list.iter() {
        let project = container.label("com.docker.compose.project");
        let service = container.label("com.docker.compose.service");
        if let Some(compose_svc) = stack_list.find_service(project, service) {
            compose_svc.set_live_container(Some(&container));
            container.set_compose_service(Some(&compose_svc));
        }
    }
}
```

Docker Compose automatically adds `com.docker.compose.project` and `com.docker.compose.service` labels to all managed containers — this is the reliable cross-reference key.

### Container Details → Compose Link

In `ContainerDetailsPage`, add a new "Compose Source" row:
- Shows stack name + service name if the container is part of a stack
- Click → navigates to that service's editor page

### Unmanaged Containers

Containers with no compose labels → shown in a special "Unmanaged" group at the bottom of the Stacks panel with an "Import to stack" action.

---

## Upstream Sync Strategy

The `TheH3X/pods` fork should track upstream `marhkb/pods` with a clean merge strategy:

```bash
git remote add upstream https://github.com/marhkb/pods.git
git fetch upstream
git merge upstream/main  # or rebase
```

**Expected conflict surface** (small and bounded):
- `src/view/client_view.{rs,blp}` — sidebar changes
- `src/model/client.rs` — stack_manager addition
- `src/application.rs` — app identity
- `Cargo.toml` — name, added deps
- `meson.build` / data files — app identity

**Zero conflict zones** (our additions live here):
- `src/compose/` — entire new module
- `src/model/stack*.rs`, `src/model/compose_*.rs`, `src/model/stack_manager.rs`
- `src/view/stack*.rs`, `src/view/compose_*.rs`

---

## Implementation Order

| Order | Phase | Effort | Key deliverable |
|-------|-------|--------|----------------|
| 0 | Fork setup + App identity | Small | "Stacks" branding, existing Pods functionality intact |
| 1 | Compose module (models + discovery + YAML I/O) | Large | Can scan a docker-stacks dir and read stack data |
| 2 | Stacks panel (list + status view) | Large | See all stacks in sidebar with live status |
| 3 | Stack editor + Plan/Deploy | X-Large | Full edit → plan → deploy workflow |
| 4 | Network editor + topology | Large | Visual network graph |
| 5 | Image metadata integration | Medium | Auto-populate from OCI config |
| 6 | Live cross-referencing | Medium | Container ↔ service bidirectional linking |

---

## Verification Plan

### Automated Tests
```bash
cargo test                           # all tests
cargo test compose::                 # compose module unit tests
cargo clippy -- -D warnings          # lints
cargo test --test integration_stacks # fixture-based integration tests
```

### Test Fixtures
```
tests/fixtures/
├── nested_stack/        # NESTED layout with 3 services
├── flat_stack/          # FLAT layout, single docker-compose.yml
├── mixed_stack/         # Ambiguous layout (both exist)
└── docker_stacks_root/  # Full multi-stack tree
```

### Manual Verification
1. Launch app → Stacks panel shown by default
2. "Open folder" → pick a docker-stacks directory → stacks list populates
3. Click a stack → detail page with live container status badges
4. Edit a service → plan/deploy workflow → files written correctly
5. Network editor → topology graph renders correctly
6. Run `docker compose up` from stack panel → streaming output in terminal widget
7. Orphan containers detected and shown in "Unmanaged" group
8. Container detail page → "Compose Source" link works for managed containers
9. Upstream merge test: `git merge upstream/main` → conflicts only in expected files
