#!/bin/bash

## update and install 1st level of packages
apt-get update
apt-get install -y \
    curl \
    git \
    gnupg2 \
    jq \
    sudo \
    zsh \
    build-essential \
    cmake \
    libssl-dev \
    openssl \
    unzip \
    g++-12 \

update-alternatives --install /usr/bin/c++ c++ /usr/bin/g++-12 50   

## Add package directories to PATH
export PATH="/usr/bin:/usr/local/bin:$PATH"

## update and install 2nd level of packages
apt-get install -y pkg-config

## install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y

source $HOME/.cargo/env
