use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeID {
    pub x: u16,
    pub y: u16,
    pub port: u16,
    pub nodeid_length: u8
}

impl NodeID {
    pub(crate) fn to_nodeid(&self, nodeid_length: Option<u8>) -> u16 {
        let _nodeid_length = nodeid_length.unwrap_or(self.nodeid_length);
        let offset: u16 = if _nodeid_length == 9 { 3 } else { 2 };
        assert!(self.x < 2_u16.pow(_nodeid_length as u32));
        assert!(self.y < 2_u16.pow(_nodeid_length as u32));
        assert!(self.port <= 1);

        (self.x << offset+3) | (self.y << 3) | (self.port << 2)
    }

    pub fn from_nodeid(nodeid: u16, nodeid_length: Option<u8>) -> NodeID {
        let _nodeid_length = nodeid_length.unwrap_or(9);
        let offset: u16 = if _nodeid_length == 9 { 3 } else { 2 };
        let mask: u16 = !(!0b0 << offset);
        let port = (nodeid & 0b000000100) >> 2;
        let y = (nodeid >> 3) & mask;
        let x = (nodeid >> (3 + offset)) & mask;
        NodeID {x, y, port, nodeid_length: _nodeid_length }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub cmn_idx: u8,
    pub event_type: u8,
    pub event_id: u16,
    pub node_id: NodeID,
    pub counts: Option<u64>
}

impl Event {
    pub fn from_captures(c: regex::Captures, nodeid_length: Option<u8>) -> Event {
        let counts = c.get(1).unwrap().as_str();
        let counts_i = match counts.trim() {
            "<not counted>" => Some(0),
            "<not supported>" => None,
            _ => Some(counts.parse::<u64>().unwrap())
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
