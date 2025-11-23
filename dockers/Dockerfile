FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    build-essential curl git ca-certificates \
    pkg-config libssl-dev bash \
    && rm -rf /var/lib/apt/lists/*

# Set up user
ARG USERNAME=samvaf
ARG USER_UID=1001
ARG USER_GID=$USER_UID

# Create user and add to sudo group
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && usermod -aG sudo $USERNAME \
    && mkdir -p /etc/sudoers.d \
    && echo "$USERNAME ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/$USERNAME \
    && chsh -s /bin/bash $USERNAME

USER $USERNAME
WORKDIR /workspace
