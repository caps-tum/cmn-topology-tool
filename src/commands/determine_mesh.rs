use std::process::Command;
use log::{info,debug};

use crate::writer::Writer;
use crate::utils::{get_event_string, get_event_type_id, perf_to_event_vec};

/// Determine mesh size of CMN
///  Use one dummy MXP perf event (mxp_n_dat_txflit_valid) and try to measure it for all possible mesh nodes given the nodeid_length
///  Those nodes actually addressable will return <not counted> or some value, non-addressable (i.e. non-existing) nodes will return <not supported>
pub fn determine(nodeid_length: u8, writer: &Writer) -> (u16,u16) {
    info!("Determining Mesh Size");

    debug!("Getting MXP type and one event ID");
    let mxp_event_type = get_event_type_id("mxp_n_dat_txflit_valid");

    let mesh_size: u16 = match nodeid_length {
        7  => 4,
        9  => 8,
        11 => 16,
        _  => 0
    };
    let mut events = Vec::new();

    for i in 0..mesh_size {
        for j in 0..mesh_size {
            events.push(String::from("-e"));
            events.push(get_event_string(0, i, j, 1, nodeid_length, mxp_event_type.as_str()));
            events.push(String::from("-e"));
            events.push(get_event_string(0, i, j, 0, nodeid_length, mxp_event_type.as_str()));
        }
    }

    let output = String::from_utf8(Command::new("perf")
        .arg("stat")
        .arg("--field-separator")
        .arg(";")
        .args(events)
        .arg("sleep")
        .arg(".01")
        .output().unwrap().stderr).unwrap();

    let parsed_output = perf_to_event_vec(output.as_str(), nodeid_length);
    writer.write_events(&parsed_output, "mxp", None);

    (parsed_output.iter().max_by_key(|event| event.node_id.x).unwrap().node_id.x + 1,
     parsed_output.iter().max_by_key(|event| event.node_id.y).unwrap().node_id.y + 1)
}