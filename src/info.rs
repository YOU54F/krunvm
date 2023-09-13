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

pub fn info(cfg: &KrunvmConfig, _matches: &ArgMatches) {
    let _json = _matches.is_present("json");
    let name = _matches.value_of("NAME").unwrap();
    if cfg.vmconfig_map.is_empty() {
        println!("No microVMs found");
    } else {
        let vmcfg = match cfg.vmconfig_map.get(name) {
            None => {
                println!("No VM found with name {}", name);
                std::process::exit(-1);
            }
            Some(vmcfg) => vmcfg,
        };
        if _json {
            serde_json::to_writer(stdout(), &vmcfg).unwrap();
        } else {
            printvm(&vmcfg);
            println!();
        }
    }
}
