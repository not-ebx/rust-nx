use byteorder::{ByteOrder, LittleEndian};

#[derive(Clone, Copy)]
pub enum NXNodeType {
    NONE,
    INT64, // 64 bit signed int
    DOUBLE, // 64 bit double
    STRING, // 32 bit uint string; Length = u16, string u8[]
    VECTOR, //
    BITMAP, //
    AUDIO //
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

impl From<u16> for NXNodeType {
    fn from(item: u16) -> Self {
        match item {
            0 => NXNodeType::NONE,
            1 => NXNodeType::INT64,
            2 => NXNodeType::DOUBLE,
            3 => NXNodeType::STRING,
            4 => NXNodeType::VECTOR,
            5 => NXNodeType::BITMAP,
            6 => NXNodeType::AUDIO,
            _ => NXNodeType::NONE
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