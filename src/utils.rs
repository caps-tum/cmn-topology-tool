use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::event::{Event, NodeID};

pub fn get_event_string(cmn_idx: u8, x: u16, y: u16, port: u16, nodeid_length: u8, event: &str) -> String {
    format!("arm_cmn_{cmn_idx}/{event},bynodeid=0x1,nodeid={:#0x}/", NodeID {x,y,port,nodeid_length}.to_nodeid())
}

pub fn get_event_type_id(event_name: &str) -> String {
    let fcontent = fs::read_to_string(format!("/sys/bus/event_source/devices/arm_cmn_0/events/{event_name}")).unwrap();
    return String::from(fcontent.trim());
}

pub fn perf_to_event_vec(s: &str, nodeid_length: u8) -> Vec<Event> {
    let pattern = Regex::new(
        r"^(\d+|<not supported>|<not counted>);;arm_cmn_(\d)\/type=(.*?),eventid=(.*?),bynodeid=0x1,nodeid=(.*?)\/.*?$")
        .unwrap();

    s.split("\n")
        .filter_map(|p| pattern.captures(p))
        .map(|c| Event::from_captures(c, nodeid_length))
        .collect::<Vec<Event>>()
}

pub fn events_map_to_vec() -> Vec<String> {
    let basepath = PathBuf::from("/sys/bus/event_source/devices/arm_cmn_0/events");

    let mut vec = Vec::new();
    for file in fs::read_dir(basepath.as_path()).unwrap() {
        let _file = file.unwrap();
        vec.push(format!("{:?};{:?}",
                         _file.file_name(),
                         fs::read_to_string(_file.path().as_path()).unwrap().trim()));
    }

    vec
}

pub fn events_to_perf_events(events: Vec<String>, node_x: u16, node_y: u16, nodeid_length: u8) -> Vec<String> {
    let mut perf_events = Vec::new();
    for event in events {
        let mut ports = Vec::new();
        let mut parsed_event = event.clone();
        let parts: Vec<_> = event.split(":").collect();
        if parts.len() > 1 {
            match parts[0] {
                "0" => ports.push(0),
                "1" => ports.push(1),
                "01"|"10" => ports.append(&mut vec![0, 1]),
                _ => {}
            }
            parsed_event = String::from(parts[1]);
        } else {
            ports.push(0);
        }

        for port in ports {
            perf_events.push(String::from("-e"));
            perf_events.push(get_event_string(0, node_x, node_y, port, nodeid_length,
                                              &get_event_type_id(parsed_event.as_str())));
        }

    }
    perf_events

}