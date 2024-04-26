use std::ffi::c_void;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use crate::nx_file::NXNodeType::{AUDIO, BITMAP, DOUBLE, INT64, NONE, STRING, VECTOR};

const MAGIC_BYTES: &str = "PKG4";

enum NXNodeType {
    NONE,
    INT64, // 64 bit signed int
    DOUBLE, // 64 bit double
    STRING, // 32 bit uint string; Length = u16, string u8[]
    VECTOR, //
    BITMAP, //
    AUDIO //
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

struct NXVectorData {
    x: i32,
    y: i32
}

struct NXBitmapData {
    id: u32,
    width: u16,
    height: u16
}

struct NXAudioData {
    id: u32,
    length: u32
}

struct NXStringData {
    id: u32,
    as_string: std::string
}

struct NXNode {
    name: u32, // String ID
    child: u32, // Node ID of first child
    n_child: u16, // amount of child
    ntype: NXNodeType,
    data: [u8; 8],
}

impl NXNode {
    fn get_string(&self) -> &str {
        // TODO find on table
        std::str::from_utf8(&self.data[0..4])?
    }
    fn get_bitmap(&self) -> NXBitmapData {
        NXBitmapData {
            id: LittleEndian::read_u32(&self.data[0..4]),
            width: LittleEndian::read_u16(&self.data[4..6]),
            height: LittleEndian::read_u16(&self.data[6..]),
        }
    }
    fn get_audio(&self) -> NXAudioData {
        NXAudioData {
            id: LittleEndian::read_u32(&self.data[0..4]),
            length: LittleEndian::read_u32(&self.data[4..]),
        }
    }

}


struct NXTables {
    bitmapData: Vec<NXBitmapData>,
    audioData: Vec<NXAudioData>,
    stringData: Vec<NXStringData>,
    header: NXFileHeader
}

struct NXFileData {
    base: c_void,
    node_table: NXTables,
}

struct NXFileHeader {
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


struct NXFile {
    file_path: PathBuf,
    file_reader: BufReader<File>,
    header: NXFileHeader,
    data: NXFileData,
}

impl NXFile {
    pub fn new(file_path: std::string) -> Self {
        let file_path = PathBuf::from(file_path);
        let file = File::open(&Self.file_path);

        let mut file_reader : BufReader<File> = BufReader::new(&file)?;

        // Check magic bytes
        let mut magic_bytes_arr: [u8;4] = [0,0,0,0];
        file_reader.read_exact(&mut magic_bytes_arr)?;
        let magic_bytes_arr = file_reader.read_u32::<LittleEndian>()?.to_le_bytes();

        // Check the magic bytes
        let str_magic = std::str::from_utf8(&magic_bytes_arr)?;
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

        // Create the node array I guess xd
        file_reader.seek(SeekFrom::Start(0))?;
        file_reader.seek(SeekFrom::Current(node_offset as i64))?;
        let mut nodes : Vec<NXNode> = Vec::with_capacity(node_count as usize);
        for (i, node) in nodes.iter_mut().enumerate() {
            // Name
            //let str_length = file_reader.read_u16::<LittleEndian>()?;
            //let mut node_name_bytes: Vec<u8> = Vec::with_capacity(str_length as usize);
            //file_reader.read_exact(&mut node_name_bytes)?;
            //let node_name = std::str::from_utf8(&node_name_bytes)?;
            let node_name = file_reader.read_u32::<LittleEndian>()?;

            // First Child ID
            let child = file_reader.read_u32::<LittleEndian>()?;
            let n_child = file_reader.read_u16::<LittleEndian>()?;
            let ntype: NXNodeType = file_reader.read_u16::<LittleEndian>()?.into();

            // Create depending on the type.
            let node_data = file_reader.read_u64::<LittleEndian>()?.to_le_bytes();
            *node = NXNode{
                name: node_name,
                child,
                n_child,
                ntype,
                data: node_data,
            }
        }


        NXFile{
            file_path,
            file_reader,
            header,
            data: NXFileData {},
        }
    }
}