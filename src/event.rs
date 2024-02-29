/* CMN Perf event wrapping and handling */
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeID {
    pub x: u16,
    pub y: u16,
    pub port: u16,
    pub nodeid_length: u8
}

impl NodeID {
    pub(crate) fn to_nodeid(&self) -> u16 {
        let offset: u16 = match self.nodeid_length {
            7  => 2,
            9  => 3,
            11 => 4,
            _  => 0
        };
        assert!(self.x < 2_u16.pow(self.nodeid_length as u32));
        assert!(self.y < 2_u16.pow(self.nodeid_length as u32));
        assert!(self.port <= 1);

        (self.x << offset+3) | (self.y << 3) | (self.port << 2)
    }

    pub fn from_nodeid(nodeid: u16, nodeid_length: u8) -> NodeID {
        let offset: u16 = match nodeid_length {
            7  => 2,
            9  => 3,
            11 => 4,
            _  => 0
        };
        let mask: u16 = !(!0b0 << offset);
        let port = (nodeid & 0b000000100) >> 2;
        let y = (nodeid >> 3) & mask;
        let x = (nodeid >> (3 + offset)) & mask;
        NodeID {x, y, port, nodeid_length }
    }
}


#[derive(Debug)]
pub struct Event {
    pub cmn_idx: u8,
    pub event_type: u8,
    pub event_id: u16,
    pub node_id: NodeID,
    pub counts: i128 // typical perf counters are unsigned 64-bit, but we need -1 for <not supported>, so go one step higher
}

impl Event {
    pub fn from_captures(c: regex::Captures, nodeid_length: u8) -> Event {
        let counts = c.get(1).unwrap().as_str();
        let counts_i = match counts.trim() {
            "<not counted>" => 0,
            "<not supported>" => -1,
            _ => counts.parse::<i128>().unwrap()
        };
        Event {
            cmn_idx: c.get(2).unwrap().as_str().parse().unwrap(),
            event_type: u8::from_str_radix(c.get(3).unwrap().as_str().trim_start_matches("0x"), 16).unwrap(),
            event_id: u16::from_str_radix(c.get(4).unwrap().as_str().trim_start_matches("0x"), 16).unwrap(),
            node_id: NodeID::from_nodeid(u16::from_str_radix(c.get(5).unwrap().as_str().trim_start_matches("0x"), 16)
                                                            .unwrap(),
                                         nodeid_length),
            counts: counts_i
        }
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Event", 5)?;
        state.serialize_field("cmn_idx", &self.cmn_idx)?;
        state.serialize_field("event_type", &format!("{:#x}", self.event_type))?;
        state.serialize_field("event_id", &format!("{:#x}", self.event_id))?;
        state.serialize_field("node_id", &format!("{:#x}", self.node_id.to_nodeid()))?;
        state.serialize_field("counts", &self.counts)?;

        state.end()
    }
}