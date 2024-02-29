use std::process::Command;
use log::{info,debug};

use crate::utils::{get_event_string, get_event_type_id, perf_to_event_vec};
use crate::writer::Writer;

/// Determine placement of HNF, HNI, and RNID nodes
///  The CMN perf integration exposes counters for HNF, HNI, and RNI/RND nodes. Use same approach as with MXP node detection
pub fn determine(nodeid_length: u8, mesh_size: (u16, u16), writer: &Writer){
    info!("Determining Node Placement");

    let mut out_events = Vec::new();
    debug!("Getting placements of HNF, HNI, and RNID nodes");
    for event_type in [ get_event_type_id("hnf_seq_full"),
                              get_event_type_id("hni_arready_no_arvalid"),
                              get_event_type_id("rnid_rdb_hybrid") ] {
        let mut events = Vec::new();
        for i in 0..mesh_size.0 {
            for j in 0..mesh_size.1 {
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 1, nodeid_length, event_type.as_str()));
                events.push(String::from("-e"));
                events.push(get_event_string(0, i, j, 0, nodeid_length, event_type.as_str()));
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

        let mut parsed_output = perf_to_event_vec(output.as_str(), nodeid_length);
        out_events.append(&mut parsed_output);
    }
    writer.write_events(&out_events, "nodes", None);
}