# Stacks

A GNOME desktop application for managing Docker Compose stacks — built on [Pods](https://github.com/marhkb/pods) by Marcus Behrendt.

Stacks focuses on the self-hosted / homelab workflow: discover your docker-stacks root directory, browse all your compose stacks in a sidebar, view service topology, and deploy changes — all from a native GTK4 interface.

## Features

- 📂 **Stack Discovery** — scan any directory following the flat (`docker-compose.yml`) or nested (per-service folders) layout convention
- 📋 **Stacks Panel** — sidebar view with stack list, service counts, and running/stopped status badges
- 🔍 **Stack Details** — services list, networks, Start/Stop/Pull actions via `docker compose`
- ✏️ **Stack Editor** — edit service definitions and review a plan before deploying
- 🔁 **Live Status** — cross-reference compose label metadata with running containers
- 🏷️ **Label Profiles** — one-click Traefik and Homepage dashboard label templates
- 🔒 **Automatic Backups** — snapshots before every deploy, pruned to keep the 10 most recent

## Fork

Stacks is a fork of [marhkb/pods](https://github.com/marhkb/pods). Container/image/volume/pod management from upstream Pods is fully preserved. Stacks adds the compose workflow on top.

Upstream tracking: `git remote add upstream https://github.com/marhkb/pods.git`

## Building

### Dev Container (Recommended)

The repository includes a devcontainer configuration. Open in VS Code with the Dev Containers extension to get a fully-configured Rust + GTK4 environment.

```sh
cargo build
cargo test
```

### Meson / Flatpak

```sh
meson setup build --prefix=/usr -Dprofile=development
meson compile -C build
```

## Architecture

Stacks follows the `marhkb/pods` GObject/Gtk4-rs architecture:

- **`src/compose/`** — pure-Rust compose backend (discovery, YAML I/O, plan/deploy, backup, CLI bridge)
- **`src/model/stack*.rs`** — GObject models wrapping compose DTOs (`Stack`, `StackList`, `StackManager`)
- **`src/view/stack*.rs`** — GTK4 view components (panel, row, details, editor, plan review)

## License

GPL-3.0-or-later — see [COPYING](COPYING)

Upstream Pods contributors: Marcus Behrendt, Wojciech Kępka, and others (see About dialog)
