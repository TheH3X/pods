FROM mcr.microsoft.com/devcontainers/rust:1-trixie

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        build-essential \
        pkg-config \
        gir1.2-glib-2.0 \
        libgtk-4-dev \
        libadwaita-1-dev \
        libgtksourceview-5-dev \
        libvte-2.91-gtk4-dev \
        gettext \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
