use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};
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
    as_string: str
}

struct NXBitmapImg {
    length: u32,
    data: Vec<u8>
}

pub struct NXNode<'a> {
    file_data: &'a NXFileData,
    name_id: u32, // String ID
    name: String,
    child: u32, // Node ID of first child
    n_child: u16, // amount of child
    ntype: NXNodeType,
    data: [u8; 8],
}

impl NXNode<'_> {
    fn get_string(&self) -> &str {
        // TODO find on table
        let string_id = LittleEndian::read_u32(&self.data);


        return "";
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

    fn get_vector(&self) -> NXVectorData {
        NXVectorData {
            x: LittleEndian::read_i32(&self.data[0..4]),
            y: LittleEndian::read_i32(&self.data[4..]),
        }
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

pub struct NXFile<'a> {
    file_path: PathBuf,
    freader: BufReader<File>,
    file_data: NXFileData,
    nodes : Vec<NXNode<'a>>,
}

impl<'a> NXFile<'a> {
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


        // CREATE THE TABLES
        // String table
        file_reader.seek(SeekFrom::Start(string_offset))?;
        let mut string_table : Vec<String> = vec!["".to_string(); string_count as usize];//Vec::with_capacity(string_count as usize);
        for curr_str in string_table.iter_mut() {
            let offset = file_reader.read_u64::<LittleEndian>()?;
            // MARK HERE
            let current_offset = file_reader.seek(SeekFrom::Current(0))?;
            //
            file_reader.seek(SeekFrom::Start(offset))?;
            let string_length = file_reader.read_u16::<LittleEndian>()?;
            let mut string_data: Vec<u8> = vec![0; string_length as usize];
            file_reader.read_exact(&mut string_data)?;

            let string_utf8 = String::from_utf8(string_data).map_err(
                |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            )?;

            *curr_str = string_utf8.to_string();

            // Reset to the beginning of the offset
            file_reader.seek(SeekFrom::Start(current_offset))?;
        }

        // TODO:
        // Bitmap & Audio.
        // I dont need them atm, but ill probably add it... later... maybe
        let nx_file_data = NXFileData{
            header,
            strings: string_table,
            audios: vec![],
            bitmaps: vec![],
        };


        Ok(NXFile{
            file_path: PathBuf::from(file_path),
            freader: file_reader,
            file_data: nx_file_data,
            nodes: vec![],
        })
    }

    // Create the nodes
    pub fn parse(&mut self) -> Result<Vec<NXNode>, Error>{
        // Create the node array I guess xd
        self.freader.seek(SeekFrom::Start(self.file_data.header.node_offset))?;
        let mut nodes : Vec<NXNode> = Vec::with_capacity(self.file_data.header.node_count as usize);
        for i in 0..self.file_data.header.node_count {
            // Name
            //let str_length = file_reader.read_u16::<LittleEndian>()?;
            //let mut node_name_bytes: Vec<u8> = Vec::with_capacity(str_length as usize);
            //file_reader.read_exact(&mut node_name_bytes)?;
            //let node_name = std::str::from_utf8(&node_name_bytes)?;
            let node_name_id = self.freader.read_u32::<LittleEndian>()?;
            let node_name = self.file_data.strings.get(node_name_id as usize).unwrap_or(&String::new()).clone();

            // First Child ID
            let child = self.freader.read_u32::<LittleEndian>()?;
            let n_child = self.freader.read_u16::<LittleEndian>()?;
            let ntype: NXNodeType = self.freader.read_u16::<LittleEndian>()?.into();

            // Create depending on the type.
            let node_data = self.freader.read_u64::<LittleEndian>()?.to_le_bytes();
             nodes.push(NXNode{
                file_data: &self.file_data,
                name_id: node_name_id,
                name: node_name,
                child,
                n_child,
                ntype,
                data: node_data,
            });
        }

        Ok(nodes)
    }
}