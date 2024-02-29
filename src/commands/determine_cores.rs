use std::process::Command;
use log::{debug, info};
use std::fs;
use std::io::Write;
use std::path::{Path};

use crate::utils::{get_event_string, get_event_type_id, perf_to_event_vec};
use crate::writer::Writer;

/// Determine position of cores / DSUs throughout CMN
///  Observe MXP p0/p1 data flits while a custom benchmark (src/benchmark/benchmark.rs) is running on two cores which causes
///   cache line transmissions between both cores.
///  This causes p0/p1 to "light up" on an otherwise quiet system
pub fn determine(nodeid_length: u8, mesh_size: (u16, u16), cores_per_dsu: u16,
                 benchmark_binary_path: &Path, benchmark_binary_args: Option<Vec<String>>,
                 writer: &Writer) {
    info!("Determining Cores");

    let num_procs = fs::read_to_string("/proc/cpuinfo").unwrap()
        .split("\n")
        .filter(|l| l.starts_with("processor"))
        .map(|l| l.split(":").skip(1).next().unwrap().trim())
        .map(|x| x.parse::<u16>().unwrap())
        .last()
        .unwrap() + 1;
    let num_dsus = num_procs / cores_per_dsu;
    debug!("Getting placements of DSUs");
    for n in 1..num_dsus {
        print!("\r[{n}/{num_dsus}]");
        std::io::stdout().flush().expect("Could not flush stdout");
        
        let mut events = Vec::new();
        for i in 0..mesh_size.0 {
            for j in 0..mesh_size.1 {
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 0, nodeid_length,
                                             &get_event_type_id("mxp_p0_dat_txflit_valid")));
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 0, nodeid_length,
                                             &get_event_type_id("mxp_p1_dat_txflit_valid")));
            }
        }

        let mut cmd = Command::new("perf");

        cmd.arg("stat")
            .arg("--field-separator")
            .arg(";")
            .args(events)
            .arg(benchmark_binary_path);
        if let Some(benchmark_binary_args) = benchmark_binary_args.clone() {
            cmd.args(benchmark_binary_args);
        }
        cmd.arg("--cores")
           .arg(format!("0,{}", cores_per_dsu*n));

        let output =  String::from_utf8(cmd.output().unwrap().stderr).unwrap();
        let parsed_output = perf_to_event_vec(output.as_str(), nodeid_length);
        writer.write_events(&parsed_output, format!("cores_0_{}", n*cores_per_dsu).as_str(), Some("cores"));
    }
    println!(); // newline to end \r shenanigans at start of loop
}