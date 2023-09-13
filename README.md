# krunvm

```krunvm``` is a CLI-based utility for creating microVMs from OCI images, using [libkrun](https://github.com/containers/libkrun) and [buildah](https://github.com/containers/buildah).

## Features

* Minimal footprint
* Fast boot time
* Zero disk image maintenance
* Zero network configuration
* Support for mapping host volumes into the guest
* Support for exposing guest ports to the host

## Demo

[![asciicast](https://asciinema.org/a/CGtTS93VsdzWwUfkY1kqVnaik.svg)](https://asciinema.org/a/CGtTS93VsdzWwUfkY1kqVnaik)

## Supported platforms

* Linux/KVM on x86_64.
* Linux/KVM on AArch64.
* macOS/Hypervisor.framework on ARM64.

## Installation

### macOS

```sh
brew tap slp/krun
brew install krunvm
```

### Fedora

```sh
dnf copr enable -y slp/libkrunfw
dnf copr enable -y slp/libkrun
dnf copr enable -y slp/krunvm
dnf install -y krunvm
```

### Usage

- Create MicroVM based on Fedora:

`krunvm create {{docker.io/fedora}} --cpus {{number_of_vcpus}} --mem {{memory_in_megabytes}} --name "{{name}}"`

- Start a specific image:

`krunvm start "{{image_name}}"`

By default it will drop into `sh`

- Start a specific image with a specific shell/command:

`krunvm start "{{image_name}}" /bin/bash`

- List images:

`krunvm list`
`krunvm list --json` # Json output
`krunvm list --json | jq '.vmconfig_map[].name'` # List names only

- Info about an image:

`krunvm list "{{image_name}}"`
`krunvm list "{{image_name}}" --json` # Json output

- Change a specific image:

`krunvm changevm --cpus {{number_of_vcpus}} --mem {{memory_in_megabytes}} --name "{{new_vm_name}}" "{{current_vm_name}}"`

- Delete a specific image:

`krunvm delete "{{image_name}}"`

- Copy environment variables from host

NB:- Experimental feature, will _probably_ conflict with things in your guest.

```sh
krunvm start alpine \
    --copy-environment \
    --filter-environment PATH,PWD,CWD,HOME,USER,TERMCAP,HOSTNAME,SHELL,PAGER,GPG_TTY,_,LC_TERMINAL,COLORTERM,TERM_PROGRAM,TERM_SESSION_ID,SSH_AUTH_SOCK 
```

### Building from sources

#### Dependencies

* Rust Toolchain
* [libkrun](https://github.com/containers/libkrun)
* [buildah](https://github.com/containers/buildah)
* asciidoctor

#### Building

```
cargo build --release
```

#### Examples in the wild

##### Byzanteam/jet-deployment

https://github.com/Byzanteam/jet-deployment/tree/f6762f0f956e285a1e2b504b5de51b031a1d0950/external-services

## Database
```bash
# Create the db MicroVM
krunvm create -p 5432:5432 --name airbase-db postgres:13.7-alpine

# Enter the vm
krunvm start airbase-db sh

# In the vm, initialize the db
POSTGRES_PASSWORD=postgres docker-entrypoint.sh postgres

# Exit the vm, and start the vm again
krunvm start airbase-db
```

## Minio
```bash
# Create the minio MicroVM
krunvm create -p 9000:9000 -p 9001:9001 --name airbase-minio minio/minio

# Start the vm
krunvm start airbase-minio "minio server /data --console-address \":9001\""
```

```bash
# Init minio

mc alias set minio http://127.0.0.1:9000 minioadmin minioadmin

mc mb minio/minio-public --ignore-existing
mc mb minio/minio-private --ignore-existing

mc policy set download minio/minio-public
```