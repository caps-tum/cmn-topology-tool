use std::process::Command;
use log::{info};
use std::fs;
use std::io::Write;
use std::path::{Path};
use crate::args::NUMAConfig;

use crate::utils::{get_event_string, get_event_type_id, perf_to_event_vec};
use crate::writer::Writer;

pub fn determine(nodeid_length: Option<u8>, mesh_size: (u16, u16), cores_per_dsu: u16, numa_config: NUMAConfig,
                 benchmark_binary_path: &Path, benchmark_binary_args: &str,
                 writer: &Writer){
    info!("Determining Node Placement");

    let _nodeid_length = nodeid_length.unwrap_or(9);

    let num_procs = fs::read_to_string("/proc/cpuinfo").unwrap()
        .split("\n")
        .filter(|l| l.starts_with("processor"))
        .map(|l| l.split(":").skip(1).next().unwrap().trim())
        .map(|x| x.parse::<u16>().unwrap())
        .last()
        .unwrap() + 1;
    let num_dsus = num_procs / cores_per_dsu as u16;

    for x in 0..num_dsus {
        for y in x+1..num_dsus {
            // CMN-600 on the Ampere Altra Max maps two DSUs per MXP, which are 64,32, and 16 core IDs apart on
            //  mono, hemi, and quad NUMA respectively - skip those
            // TODO: derive logic on other systems
            //  assumption: 64 => num_procs/2 (since two ports),
            //              32 => num_procs/2/2 (since hemi),
            //              16 => num_procs/2/4 (since quad)
            let cont;
            match numa_config {
                NUMAConfig::Monolithic => { cont = y>=(64/cores_per_dsu) && x == y-(64/cores_per_dsu) },
                NUMAConfig::Hemisphere => { cont = y>=(32/cores_per_dsu) && x == y-(32/cores_per_dsu) },
                NUMAConfig::Quadrant   => { cont = y>=(16/cores_per_dsu) && x == y-(16/cores_per_dsu) },
            }
            if cont { continue }

            print!("\r[{x} {y} | {num_dsus} -- {}/{}]", x*num_dsus + y, num_dsus*num_dsus/2);
            std::io::stdout().flush().expect("Could not flush stdout");

            let mut events = Vec::new();
            for i in 0..mesh_size.0 {
                for j in 0..mesh_size.1 {
                    events.push(String::from("-e"));
                    events.push(get_event_string(0, i, j, 0, _nodeid_length, &get_event_type_id("mxp_n_dat_txflit_valid")));
                    events.push(String::from("-e"));
                    events.push(get_event_string(0, i, j, 0, _nodeid_length, &get_event_type_id("mxp_s_dat_txflit_valid")));
                    events.push(String::from("-e"));
                    events.push(get_event_string(0, i, j, 0, _nodeid_length, &get_event_type_id("mxp_e_dat_txflit_valid")));
                    events.push(String::from("-e"));
                    events.push(get_event_string(0, i, j, 0, _nodeid_length, &get_event_type_id("mxp_w_dat_txflit_valid")));
                }
            }

            let mut cmd = Command::new("perf");

            cmd.arg("stat")
                .arg("--field-separator")
                .arg(";")
                .args(events)
                .arg(benchmark_binary_path)
                .arg("--num-samples")
                .arg("2500")
                .arg("--num-iterations")
                .arg("25000")
                .arg("--cores")
                .arg(format!("{},{}", cores_per_dsu*x, cores_per_dsu*y));

            let output =  String::from_utf8(cmd.output().unwrap().stderr).unwrap();
            let parsed_output = perf_to_event_vec(output.as_str(), Some(_nodeid_length));
            writer.write_events(&parsed_output, format!("edges_{}_{}", x*cores_per_dsu, y*cores_per_dsu).as_str(), Some("edges"));
        }
        }


}