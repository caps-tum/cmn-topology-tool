use std::process::Command;
use log::{info};
use std::fs;
use std::io::Write;
use std::path::{Path};

use crate::utils::{get_event_string, get_event_type_id, perf_to_event_vec};
use crate::writer::Writer;

pub fn determine(nodeid_length: Option<u8>, mesh_size: (u16, u16), cores_per_dsu: u16,
                 benchmark_binary_path: &Path, benchmark_binary_args: &str,
                 writer: &Writer){
    info!("Determining Edges");

    let _nodeid_length = nodeid_length.unwrap_or(9);

    let num_procs = fs::read_to_string("/proc/cpuinfo").unwrap()
        .split("\n")
        .filter(|l| l.starts_with("processor"))
        .map(|l| l.split(":").skip(1).next().unwrap().trim())
        .map(|x| x.parse::<u16>().unwrap())
        .last()
        .unwrap() + 1;
    let num_dsus = num_procs / cores_per_dsu as u16;

    for n in 1..num_dsus {
        print!("\r[{n}/{num_dsus}]");
        std::io::stdout().flush().expect("Could not flush stdout");
        
        let mut events = Vec::new();
        for i in 0..mesh_size.0 {
            for j in 0..mesh_size.1 {
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 0, _nodeid_length, &get_event_type_id("mxp_p0_dat_txflit_valid")));
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 0, _nodeid_length, &get_event_type_id("mxp_p1_dat_txflit_valid")));
            }
        }

        let mut cmd = Command::new("perf");

        cmd.arg("stat")
            .arg("--field-separator")
            .arg(";")
            .args(events)
            .arg(benchmark_binary_path);
        if benchmark_binary_args.len() > 0 {
            cmd.args(benchmark_binary_args.split(" ").collect::<Vec<_>>());
        }
        cmd.arg("--cores")
           .arg(format!("0,{}", cores_per_dsu*n));

        let output =  String::from_utf8(cmd.output().unwrap().stderr).unwrap();
        let parsed_output = perf_to_event_vec(output.as_str(), Some(_nodeid_length));
        writer.write_events(&parsed_output, format!("cores_0_{}", n*cores_per_dsu).as_str(), Some("cores"));
    }

}