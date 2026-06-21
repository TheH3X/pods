<h1 align="center">
  <img src="data/icons/com.github.TheH3X.Stacks.svg" alt="Stacks" width="192" height="192"/>
  <br>
  Stacks
</h1>

<p align="center"><strong>Manage your Docker Compose stacks</strong></p>

<br>

Stacks is a GTK4/libadwaita desktop application for managing Docker Compose stacks with a **stack-first, everything-is-a-stack** philosophy.

Forked from [Pods](https://github.com/marhkb/pods) and extended with Docker Compose stack management as a first-class feature.

## Philosophy

- **Everything is a stack** — even a single container is modelled as a single-service stack. No free-floating containers.
- **Per-service directory layout** — each service gets its own `<name>/compose.yml` + `<name>/appdata/` directory, keeping bind mounts self-contained and easy to migrate by copying.
- **Plan/Deploy workflow** — all edits are in-memory until you explicitly deploy. No accidental writes.
- **Raw-merge preservation** — unmodelled compose keys (`depends_on`, `healthcheck`, `build`, …) are never lost on write.
- **Live cross-referencing** — Docker Compose labels (`com.docker.compose.service`) are used to link running containers back to their stack definition.

## Features

- Discover and browse all your Docker Compose stacks from a `docker-stacks` directory tree
- Edit services with full plan/deploy workflow — image, ports, volumes, env vars, labels, and extra fields
- View live container status for each service directly in the stack editor
- YAML diff view — see exactly what will change on disk before deploying
- Visualize Docker network topology between services and to the outside world (ports exposed to host vs. internal)
- Browse images from Docker Hub and GHCR; auto-populate service configuration from OCI image metadata
- Per-service filesystem tree with bind mount annotations
- Run `docker compose up / down / pull / logs` with streamed terminal output
- Homepage and Traefik label profiles per image type
- Browse and manage images and volumes (powered by the Bollard Docker API)
- Backup/snapshot on deploy

## Stack Layout

```
~/opt/docker-stacks/
  monitoring/
    compose.stack.yml       # shared networks/volumes
    grafana/
      compose.yml           # one service per folder
      appdata/              # bind mount data (stays with the service)
    prometheus/
      compose.yml
    docker-compose.yml      # include manifest (entry point for docker compose up)
    .env                    # DOMAIN=, TZ=, UID=, GID=
```

Monolithic stacks (single `docker-compose.yml` with all services) are also supported — Stacks detects and handles both layouts automatically.

## Requirements

- GTK 4 >= 4.18
- libadwaita >= 1.7
- gtksourceview-5 >= 4.90
- vte-2.91-gtk4 >= 0.70
- Docker with the Compose plugin (`docker compose`)

## Building

```shell
git clone https://github.com/TheH3X/pods.git stacks
cd stacks
meson _build --prefix=/usr/local
ninja -C _build install
```

## Development

This project uses a [devcontainer](.devcontainer/) for a consistent development environment.

See [Pods' development documentation](https://github.com/marhkb/pods#-developing) for Meson/GNOME Builder setup — the same workflow applies here.

## Upstream Sync

Stacks tracks upstream [marhkb/pods](https://github.com/marhkb/pods) for engine, model, and UI improvements:

```shell
git remote add upstream https://github.com/marhkb/pods.git
git fetch upstream
git merge upstream/main
```

Stack-specific code lives entirely in `src/compose/`, `src/model/stack*.rs`, and `src/view/stack*.rs` to minimize merge conflicts.

## License

GPL-3.0-or-later (same as upstream Pods)
