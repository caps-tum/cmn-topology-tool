use std::process::Command;
use log::{info,debug};

use crate::utils::{get_event_string, get_event_type_id, perf_to_event_vec};
use crate::writer::Writer;

pub fn determine(nodeid_length: Option<u8>, mesh_size: (u16, u16), writer: &Writer){
    info!("Determining Node Placement");

    let _nodeid_length = nodeid_length.unwrap_or(9);

    let mut out_events = Vec::new();
    debug!("Getting event types and IDs for HNF, HNI, and RNID");
    for event_type in [ get_event_type_id("hnf_seq_full"),
                              get_event_type_id("hni_arready_no_arvalid"),
                              get_event_type_id("rnid_rdb_hybrid") ] {
        let mut events = Vec::new();
        for i in 0..mesh_size.0 {
            for j in 0..mesh_size.1 {
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 1, _nodeid_length, event_type.as_str()));
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 0, _nodeid_length, event_type.as_str()));
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

        let mut parsed_output = perf_to_event_vec(output.as_str(), Some(_nodeid_length));
        out_events.append(&mut parsed_output);
    }
    writer.write_events(&out_events, "nodes", None);
}