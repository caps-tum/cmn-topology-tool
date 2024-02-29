use std::fs;
use std::process::{Command, Stdio};
use log::{info, debug};

use crate::args::LaunchArgs;
use crate::utils::{events_to_perf_events, perf_to_event_vec};
use crate::writer::Writer;

/// Launch application while observing CMN with given perf events
pub fn launch(args: &LaunchArgs, mesh_size: (u16,u16),  nodeid_length: u8, events: Option<Vec<String>>, writer: &Writer) {

    let num_procs = fs::read_to_string("/proc/cpuinfo").unwrap()
        .split("\n")
        .filter(|l| l.starts_with("processor"))
        .map(|l| l.split(":").skip(1).next().unwrap().trim())
        .last().unwrap()
        .parse::<u16>().unwrap();

    let mut perf_events = Vec::new();
    if let Some(_events) = events {
        for i in 0..mesh_size.0 {
            for j in 0..mesh_size.1 {
                perf_events.append(&mut events_to_perf_events(_events.clone(), i, j, nodeid_length));
            }
        }
    }

    let mut cmd = Command::new("taskset");
    cmd.arg("--cpu-list")
        .arg(args.core_map.clone().unwrap_or(String::from(format!("0-{num_procs}"))));

    if let Some(shell) = args.shell.clone() { cmd.arg(shell).arg("-c"); }

    if perf_events.len() > 0 {
        cmd.arg("perf").arg("stat").arg("--field-separator").arg(";").args(perf_events);
    }

    cmd.arg(args.binary.clone());
    if let Some(args) = args.args.clone() { cmd.args(args); }

    if let Some(env) = args.env.clone() {
        for entry in env {
            let mut parts = entry.split("=");
            cmd.env(parts.next().unwrap(), parts.next().unwrap());
        }
    }

    if let Some(pwd) = args.pwd.clone() {
        cmd.current_dir(pwd);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    info!("Launching application {}", args.binary);
    debug!("Command: `{:?}`", cmd);
    let cmd_output = cmd.spawn().unwrap().wait_with_output().unwrap();
    let output_stderr =  String::from_utf8(cmd_output.stderr).unwrap();
    let parsed_output = perf_to_event_vec(output_stderr.as_str(), nodeid_length);
    writer.write_events(&parsed_output, "measurements", None);
    writer.write_lines(vec![String::from_utf8(cmd_output.stdout).unwrap()], "stdout.txt");
    writer.write_lines(vec![output_stderr], "stderr.txt");
}