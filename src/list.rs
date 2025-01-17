// Copyright 2021 Red Hat, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{ArgMatches, KrunvmConfig, VmConfig};
use serde_json;
use std::io::stdout;
pub fn printvm(vm: &VmConfig) {
    println!("{}", vm.name);
    println!(" CPUs: {}", vm.cpus);
    println!(" RAM (MiB): {}", vm.mem);
    println!(" DNS server: {}", vm.dns);
    println!(" Buildah container: {}", vm.container);
    println!(" Workdir: {}", vm.workdir);
    println!(" Mapped volumes: {:?}", vm.mapped_volumes);
    println!(" Mapped ports: {:?}", vm.mapped_ports);
}

pub fn list(cfg: &KrunvmConfig, _matches: &ArgMatches) {
    let _json = _matches.is_present("json");
    if cfg.vmconfig_map.is_empty() {
        println!("No microVMs found");
    } else {
        if _json {
            serde_json::to_writer(stdout(), &cfg).unwrap();
        } else {
            for (_name, vm) in cfg.vmconfig_map.iter() {
                println!();
                printvm(vm);
            }
            println!();
        }
    }
}
