use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};
use std::path::PathBuf;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use crate::nx_file::NXNodeType::{AUDIO, BITMAP, DOUBLE, INT64, NONE, STRING, VECTOR};

const MAGIC_BYTES: &str = "PKG4";

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

pub enum NXNodeData {
    String(String),
    Bitmap(NXBitmapData),
    Audio(NXAudioData),
    Int64(i64),
    Double(f64),
    Vector(NXVectorData),
    None
}

impl NXNodeData {

}

impl From<u16> for NXNodeType {
    fn from(item: u16) -> Self {
        match item {
            0 => NONE,
            1 => INT64,
            2 => DOUBLE,
            3 => STRING,
            4 => VECTOR,
            5 => BITMAP,
            6 => AUDIO,
            _ => NONE
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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


#[derive(Debug)]
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

pub struct NXNode {
    name_id: u32, // String ID
    pub name: String,
    pub child: u32, // Node ID of first child
    pub n_child: u16, // amount of child
    ntype: NXNodeType,
    pub data: NXNodeData,
}


impl NXNode {
    pub fn has_children(&self) -> bool {
        self.n_child > 0
    }
}


pub struct NXFileHeader {
    magic : u32,
    node_count: u32,
    node_offset: u64,
    string_count: u32,
    string_offset: u64,
    bitmap_count: u32,
    bitmap_offset: u64,
    audio_count: u32,
    audio_offset: u64,
}

pub struct NXFileData {
    header: NXFileHeader,
    strings : Vec<String>,
    audios: Vec<NXAudioData>,
    bitmaps: Vec<NXBitmapData>,
}

pub struct NXFile {
    file_path: PathBuf,
    freader: BufReader<File>,
    file_data: NXFileData,
    nodes : Vec<NXNode>,
}

impl NXFile {
    pub fn new(file_path: &str) -> Result<Self, Error> {
        let file_path_buf = PathBuf::from(file_path);
        let file = File::open(file_path_buf)?;

        let mut file_reader : BufReader<File> = BufReader::new(file);

        // Check magic bytes
        let mut magic_bytes_arr: [u8;4] = [0,0,0,0];
        file_reader.read_exact(&mut magic_bytes_arr)?;
        //let magic_bytes_arr = file_reader.read_u32::<LittleEndian>()?.to_le_bytes();

        // Check the magic bytes
        let str_magic = std::str::from_utf8(&magic_bytes_arr).map_err(
            |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        )?;
        if !str_magic.eq(MAGIC_BYTES) {
            panic!("Can't open specified file. Not a valid nx file lol");
        }

        // Create the header
        let node_count = file_reader.read_u32::<LittleEndian>()?;
        let node_offset = file_reader.read_u64::<LittleEndian>()?;
        let string_count = file_reader.read_u32::<LittleEndian>()?;
        let string_offset = file_reader.read_u64::<LittleEndian>()?;
        let bitmap_count = file_reader.read_u32::<LittleEndian>()?;
        let bitmap_offset = file_reader.read_u64::<LittleEndian>()?;
        let audio_count = file_reader.read_u32::<LittleEndian>()?;
        let audio_offset = file_reader.read_u64::<LittleEndian>()?;

        let header = NXFileHeader{
            magic: LittleEndian::read_u32(&magic_bytes_arr),
            node_count,
            node_offset,
            string_count,
            string_offset,
            bitmap_count,
            bitmap_offset,
            audio_count,
            audio_offset,
        };

        let nx_file_data = NXFileData{
            header,
            strings: vec![],
            audios: vec![],
            bitmaps: vec![],
        };

        let mut nx_file = NXFile{
            file_path: PathBuf::from(file_path),
            freader: file_reader,
            file_data: nx_file_data,
            nodes: vec![],
        };

        nx_file.load_tables()?;
        nx_file.load_nodes()?;

        Ok(nx_file)
    }

    fn create_node_data(&self, ntype: NXNodeType, data: [u8; 8]) -> NXNodeData {
        match ntype {
            INT64 => {NXNodeData::Int64(LittleEndian::read_i64(&data))},
            DOUBLE => {NXNodeData::Double(LittleEndian::read_f64(&data))},
            STRING => {NXNodeData::String(self.get_string(LittleEndian::read_u32(&data)))},
            VECTOR => {NXNodeData::Vector(NXVectorData::new(&data))},
            BITMAP => {NXNodeData::Bitmap(NXBitmapData::new(&data))},
            AUDIO => {NXNodeData::Audio(NXAudioData::new(&data))},
            _ => { NXNodeData::None}
        }
    }

    fn get_string(&self, string_id: u32) -> String {
        let string_text = self.file_data.strings.get(string_id as usize);
        match string_text {
            Some(text) => {text.to_string()},
            None => { "".to_string() }
        }
    }

    fn load_tables(&mut self) -> Result<(), Error>{
        // CREATE THE TABLES
        // String table
        self.freader.seek(SeekFrom::Start(self.file_data.header.string_offset))?;
        let mut string_table : Vec<String> = vec!["".to_string(); self.file_data.header.string_count as usize];
        for curr_str in string_table.iter_mut() {
            let offset = self.freader.read_u64::<LittleEndian>()?;
            // MARK HERE
            let current_offset = self.freader.seek(SeekFrom::Current(0))?;
            //
            self.freader.seek(SeekFrom::Start(offset))?;
            let string_length = self.freader.read_u16::<LittleEndian>()?;
            let mut string_data: Vec<u8> = vec![0; string_length as usize];
            self.freader.read_exact(&mut string_data)?;

            let string_utf8 = String::from_utf8(string_data).map_err(
                |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            )?;

            *curr_str = string_utf8.to_string();

            // Reset to the beginning of the offset
            self.freader.seek(SeekFrom::Start(current_offset))?;
        }
        self.file_data.strings = string_table;

        // TODO:
        // Bitmap & Audio.
        // I dont need them atm, but ill probably add it... later... maybe
        // Bitmap Table
        /*
        self.freader.seek(SeekFrom::Start(bitmap_offset))?;
        let mut bitmap_table : Vec<NXBitmapImg> = Vec::with_capacity(bitmap_count as usize);
        for i in 0..bitmap_count {
            let offset = self.freader.read_u64::<LittleEndian>()?;
            let current_offset = self.freader.seek(SeekFrom::Current(0))?;
            self.freader.seek(SeekFrom::Start(offset))?;

            let bitmap_length = self.freader.read_u32::<LittleEndian>()?;
            let mut bitmap_data : Vec<u8> = vec![0; bitmap_length as usize];
            self.freader.read_exact(&mut bitmap_data)?;

            bitmap_table.push(NXBitmapImg{
                length: bitmap_length,
                data: bitmap_data,
            });
            self.freader.seek(SeekFrom::Start(current_offset))?;
        }
        self.file_data.bitmaps = bitmap_table;
        */
        Ok(())
    }

    // Create the nodes
    fn load_nodes(&mut self) -> Result<(), Error>{
        // Create the node array I guess xd
        self.freader.seek(SeekFrom::Start(self.file_data.header.node_offset))?;
        let mut nodes : Vec<NXNode> = Vec::with_capacity(self.file_data.header.node_count as usize);
        for _  in 0..self.file_data.header.node_count {
            // Name
            let node_name_id = self.freader.read_u32::<LittleEndian>()?;
            let node_name = self.file_data.strings.get(node_name_id as usize).unwrap_or(&String::new()).clone();

            // First Child ID
            let child = self.freader.read_u32::<LittleEndian>()?;
            let n_child = self.freader.read_u16::<LittleEndian>()?;
            let ntype: NXNodeType = self.freader.read_u16::<LittleEndian>()?.into();

            // Create depending on the type.
            let node_data: [u8; 8] = self.freader.read_u64::<LittleEndian>()?.to_le_bytes();

            nodes.push(NXNode{
                name_id: node_name_id,
                name: node_name,
                child,
                n_child,
                ntype,
                data: self.create_node_data(ntype, node_data),
            });
        }

        self.nodes = nodes;
        Ok(())
    }

    pub fn get_node_children(&self, node: &NXNode) -> Option<Vec<&NXNode>> {
        let start_i = node.child as usize;
        let end_i = start_i + node.n_child as usize;

        if end_i <= self.nodes.len() {
            Some(self.nodes.get(start_i..end_i));
        }
        None
    }

    pub fn resolve(&self, full_path: &str) -> Option<&NXNode> {
        let node_path: Vec<&str> = full_path.split("/").collect();
        if node_path.len() <= 0 {
            return None
        }

        let mut current_node : &NXNode = &self.nodes[0];
        // Search for the first one
        for (_, path) in node_path.iter().enumerate() {
            let node_cursor = self.get_node_children(current_node)?;
            let node_result = node_cursor.iter().find(
                |&x| x.name.eq(path)
            );
            match node_result {
                Some(x) => {
                    current_node = x
                },
                None => {
                    return None
                }
            }
        }
        Some(current_node)
    }
}