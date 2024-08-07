use std::num::ParseFloatError;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NXNodeType {
    Empty,
    Long, // 64 bit signed int
    Double, // 64 bit double
    Text, // 32 bit uint string; Length = u16, string u8[]
    Vector, //
    Bitmap, //
    Audio //
}

#[derive(Clone)]
pub enum NXNodeData {
    String(String),
    Bitmap(NXBitmapData),
    Audio(NXAudioData),
    Int64(i64),
    Double(f64),
    Vector(NXVectorData),
    None
}

impl Into<[i32;2]> for NXNodeData {
    fn into(self) -> [i32; 2] {
        match self {
            NXNodeData::Vector(vector) => [vector.x, vector.y],
            _ => panic!("NXNodeData variant is not a Vector"),
        }
    }
}

impl Into<String> for NXNodeData {
    fn into(self) -> String {
        match self {
            NXNodeData::Double(num) => num.to_string(),
            NXNodeData::Int64(num) => num.to_string(),
            NXNodeData::String(s) => s,
            _ => panic!("NXNodeData variant is not a String"),
        }
    }
}

impl Into<i64> for NXNodeData {
    fn into(self) -> i64 {
        match self {
            NXNodeData::Double(num) => num as i64,
            NXNodeData::String(s) => s.parse::<i64>().expect("is not a valid number!!"),
            NXNodeData::Int64(i) => i,
            _ => panic!("NXNodeData variant is not an Int64"),
        }
    }
}

impl Into<i32> for NXNodeData {
    fn into(self) -> i32 {
        let as_long: i64 = self.into();
        as_long as i32
    }
}

impl Into<i16> for NXNodeData {
    fn into(self) -> i16 {
        let as_long: i64 = self.into();
        as_long as i16
    }
}

impl Into<u8> for NXNodeData {
    fn into(self) -> u8 {
        let as_long: i64 = self.into();
        as_long as u8
    }
}

impl Into<bool> for NXNodeData {
    fn into(self) -> bool {
        let as_string: String = self.into();
        match as_string.parse::<f64>() {
            Ok(num) => {
                num != 0.0
            }
            Err(_) => {
                as_string.eq("true")
            }
        }
    }
}

impl Into<f64> for NXNodeData {
    fn into(self) -> f64 {
        match self {
            NXNodeData::Double(d) => d,
            NXNodeData::Int64(i) => i as f64,
            NXNodeData::String(s) => s.parse::<f64>().expect("is not a valid number!!"),
            _ => panic!("NXNodeData variant is not a Double"),
        }
    }
}

impl From<u16> for NXNodeType {
    fn from(item: u16) -> Self {
        match item {
            0 => NXNodeType::Empty,
            1 => NXNodeType::Long,
            2 => NXNodeType::Double,
            3 => NXNodeType::Text,
            4 => NXNodeType::Vector,
            5 => NXNodeType::Bitmap,
            6 => NXNodeType::Audio,
            _ => NXNodeType::Empty
        }
    }
}

#[derive(Clone)]
pub struct NXVectorData {
    pub x: i32,
    pub y: i32
}

impl NXVectorData {
    pub fn new(data: &[u8; 8]) -> Self {
        let data_x = &data[0..4];
        let data_y = &data[4..];

        NXVectorData {
            x: LittleEndian::read_i32(data_x),
            y: LittleEndian::read_i32(data_y)
        }
    }
}

#[derive(Clone)]
pub struct NXBitmapData {
    pub id: u32,
    pub width: u16,
    pub height: u16
}


impl NXBitmapData {
    pub fn new(data: &[u8; 8]) -> Self {
        NXBitmapData {
            id: LittleEndian::read_u32(&data[0..4]),
            width: LittleEndian::read_u16(&data[4..6]),
            height: LittleEndian::read_u16(&data[6..]),
        }
    }
}


#[derive(Clone)]
pub struct NXAudioData {
    pub id: u32,
    pub length: u32
}

impl NXAudioData {
    pub fn new(data: &[u8; 8]) -> Self {
        NXAudioData {
            id: LittleEndian::read_u32(&data[0..4]),
            length: LittleEndian::read_u32(&data[4..]),
        }
    }
}

#[derive(Clone)]
pub struct NXNode {
    pub name_id: u32, // String ID
    pub name: String,
    pub child: u32, // Node ID of first child
    pub n_child: u16, // amount of child
    pub ntype: NXNodeType,
    pub data: NXNodeData,
}


impl NXNode {
    pub fn has_children(&self) -> bool {
        self.n_child > 0
    }
}
