use std::collections::HashMap;
use std::fs;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use log::info;

use crate::args::{LaunchMultiArgs, LaunchMultiConfig};
use crate::utils::{events_to_perf_events, perf_to_event_vec};
use crate::writer::Writer;

/// Launch multiple applications while observing CMN with given perf events
pub fn launch_multi(args: &LaunchMultiArgs, mesh_size: (u16,u16),  nodeid_length: u8, events: Option<Vec<String>>,
                    writer: &mut Writer) {
    let num_procs = fs::read_to_string("/proc/cpuinfo").unwrap()
        .split("\n")
        .filter(|l| l.starts_with("processor"))
        .map(|l| l.split(":").skip(1).next().unwrap().trim())
        .last().unwrap()
        .parse::<u16>().unwrap();

    let config: LaunchMultiConfig = serde_json::from_str(fs::read_to_string(args.config.clone()).unwrap().as_str()).unwrap();
    writer.additional_args = serde_json::to_string(&config).unwrap();

    let mut perf_events = Vec::new();
    if let Some(_events) = events {
        for i in 0..mesh_size.0 {
            for j in 0..mesh_size.1 {
                perf_events.append(&mut events_to_perf_events(_events.clone(), i, j, nodeid_length));
            }
        }
    }

    let mut commands = Vec::new();
    for exec in config.executables {
        let mut cmd = Command::new("taskset");

        cmd.arg("--cpu-list")
            .arg(exec.core_map.clone().unwrap_or(String::from(format!("0-{num_procs}"))));

        if let Some(shell) = exec.shell.clone() { cmd.arg(shell).arg("-c"); }

        cmd.arg(exec.binary.clone());
        if let Some(args) = exec.args.clone() { cmd.arg(args); }

        if let Some(env) = exec.env {
            for (k,v) in env {
                cmd.env(k, v.as_str().unwrap());
            }
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        commands.push((exec.binary, cmd));
    }

    let mut perf_cmd = Command::new("perf");
    perf_cmd.arg("stat").arg("--field-separator").arg(";").args(perf_events);
    perf_cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let perf_proc = perf_cmd.spawn().unwrap();


    let mut id_name_map = HashMap::new();
    info!("Launching {} applications", commands.len());
    let mut inflight: Vec<Child> = commands.iter_mut().map(|(binary,c)| {
        let child = c.spawn().unwrap();
        id_name_map.insert(child.id(), binary.split("/").last().unwrap());
        child
    }).collect();

    while inflight.iter_mut().fold(false, |acc, c| acc || c.try_wait().unwrap().is_none() ) {
        sleep(Duration::from_secs(1));
    }

    for child in inflight {
        let id = child.id().clone();
        let output = child.wait_with_output().unwrap();

        writer.write_lines(vec![String::from_utf8(output.clone().stdout).unwrap()],
                           format!("{}-{}.stdout", id, id_name_map.get(&id).unwrap()).as_str());
        writer.write_lines(vec![String::from_utf8(output.clone().stderr).unwrap()],
                           format!("{}-{}.stderr", id, id_name_map.get(&id).unwrap()).as_str());
    }

    // send ^C / SIGINT to perf_proc
    let _ = Command::new("kill")
        .args(["-s", "INT", &perf_proc.id().to_string()])
        .spawn().unwrap().wait();

    let cmd_output = perf_proc.wait_with_output().unwrap();
    let output_stderr =  String::from_utf8(cmd_output.stderr).unwrap();
    let parsed_output = perf_to_event_vec(output_stderr.as_str(), nodeid_length);
    writer.write_events(&parsed_output, "measurements", None);
    writer.write_lines(vec![String::from_utf8(cmd_output.stdout).unwrap()], "stdout.txt");
    writer.write_lines(vec![output_stderr], "stderr.txt");
}