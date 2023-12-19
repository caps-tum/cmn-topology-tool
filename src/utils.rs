use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::event::{Event, NodeID};

pub fn get_event_string(cmn_idx: u8, x: u16, y: u16, port: u16, nodeid_length: u8, event: &str) -> String {
    format!("arm_cmn_{cmn_idx}/{event},bynodeid=0x1,nodeid={:#0x}/", NodeID {x,y,port,nodeid_length}.to_nodeid(None))
}

pub fn get_event_type_id(event_name: &str) -> String {
    let fcontent = fs::read_to_string(format!("/sys/bus/event_source/devices/arm_cmn_0/events/{event_name}")).unwrap();
    return String::from(fcontent.trim());
}

pub fn perf_to_event_vec(s: &str, nodeid_length: Option<u8>) -> Vec<Event> {
    let pattern = Regex::new(
        r"^(\d+|<not supported>|<not counted>);;arm_cmn_(\d)\/type=(.*?),eventid=(.*?),bynodeid=0x1,nodeid=(.*?)\/.*?$")
        .unwrap();

    s.split("\n")
        .filter_map(|p| pattern.captures(p))
        .map(|c| Event::from_captures(c, nodeid_length))
        .filter(|e| e.counts.is_some())
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