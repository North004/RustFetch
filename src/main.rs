#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::HashMap;
use std::io::{self, stdout};
use std::path;
use std::path::Path;
use sysinfo::{Components, Disks, Networks, System};
use whoami;

const BYTEGB: u64 = 1024 * 1024 * 1024;
const RESET: &'static str = "\x1b[0m";
const BLACK: &'static str = "\x1b[30m";
const RED: &'static str = "\x1b[31m";
const GREEN: &'static str = "\x1b[32m";
const YELLOW: &'static str = "\x1b[33m";
const BLUE: &'static str = "\x1b[34m";
const MAGENTA: &'static str = "\x1b[35m";
const CYAN: &'static str = "\x1b[36m";
const WHITE: &'static str = "\x1b[37m";
const BRIGHT_BLACK: &'static str = "\x1b[90m";
const BRIGHT_RED: &'static str = "\x1b[91m";
const BRIGHT_GREEN: &'static str = "\x1b[92m";
const BRIGHT_YELLOW: &'static str = "\x1b[93m";
const BRIGHT_BLUE: &'static str = "\x1b[94m";
const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
const BRIGHT_CYAN: &'static str = "\x1b[96m";
const BRIGHT_WHITE: &'static str = "\x1b[97m";

fn btogb(x: u64) -> f64 {
    x as f64 / (1024 * 1024 * 1024) as f64
}

fn main() {
    let (order, info) = system_info();

    for key in order {
        match info.get(&key) {
            Some(Some(val)) => println!("{RED}{}{RESET}: {}", key, val),
            Some(None) => println!("{}: None", key),
            None => println!("{}: Key not found", key),
        }
    }

    let (order, info) = mem_info();

    for key in order {
        match info.get(&key) {
            Some(Some(val)) => println!("{MAGENTA}{}{RESET}: {}", key, val),
            Some(None) => println!("{}: None", key),
            None => println!("{}: Key not found", key),
        }
    }

    let (order, info) = cpu_info();
    for key in order {
        match info.get(&key) {
            Some(Some(val)) => println!("{CYAN}{}{RESET}: {}", key, val),
            Some(None) => println!("{}: None", key),
            None => println!("{}: Key not found", key),
        }
    }
}

fn system_info() -> (Vec<String>, HashMap<String, Option<String>>) {
    let mut order = Vec::new();
    let mut sysinfo = HashMap::new();

    let hostname: Option<String> = match whoami::fallible::hostname() {
        Ok(result) => Some(result),
        Err(_) => None,
    };

    order.push("Username".to_string());
    sysinfo.insert("Username".to_string(), Some(whoami::username()));

    order.push("Hostname".to_string());
    sysinfo.insert("Hostname".to_string(), hostname);

    order.push("Distro".to_string());
    sysinfo.insert("Distro".to_string(), Some(whoami::distro()));

    order.push("Kernel".to_string());
    sysinfo.insert("Kernel".to_string(), System::kernel_version());
    (order, sysinfo)
}
fn mem_info() -> (Vec<String>, HashMap<String, Option<String>>) {
    let mut order = Vec::new();
    let mut meminfo = HashMap::new();
    let sys = System::new_all();

    order.push("Memory".to_string());
    meminfo.insert(
        "Memory".to_string(),
        Some(format!(
            "{:.2} GB / {:.2} GB",
            btogb(sys.used_memory()),
            btogb(sys.total_memory())
        )),
    );

    order.push("Swap".to_string());
    meminfo.insert(
        "Swap".to_string(),
        Some(format!(
            "{:.2} GB / {:.2} GB",
            btogb(sys.used_swap()),
            btogb(sys.total_swap())
        )),
    );

    (order, meminfo)
}

fn cpu_info() -> (Vec<String>, HashMap<String, Option<String>>) {
    let mut order = Vec::new();
    let mut cpuinfo = HashMap::new();
    let sys = System::new_all();

    order.push("CPU Brand".to_string());
    cpuinfo.insert(
        "CPU Brand".to_string(),
        Some(sys.cpus()[0].brand().to_string()),
    );

    order.push("Num Cores".to_string());
    cpuinfo.insert("Num Cores".to_string(), Some(sys.cpus().len().to_string()));

    order.push("CPU Frequency".to_string());
    cpuinfo.insert(
        "CPU Frequency".to_string(),
        Some(format!("{}Mhz", sys.cpus()[0].frequency())),
    );

    order.push("Architecture".to_string());
    cpuinfo.insert("Architecture".to_string(), Some(whoami::arch().to_string()));

    (order, cpuinfo)
}
