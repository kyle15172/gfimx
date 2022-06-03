use std::{fs::File, io::{BufReader, self}};

pub struct FileProgress {
    pub name: String,
    pub file: Box<BufReader<File>>,
    pub chunks: u64,
    pub chunk_no: u64,
}

const CAPACITY:u64 = 1;

impl FileProgress {
    pub fn new(path: String) -> io::Result<Self> {
        let file = File::open(&path)?;
        let size = file.metadata()?.len();
        let (mut quot, rem) = (size / CAPACITY, size % CAPACITY);
        if rem > 0 {
            quot += 1;
        }
        let reader = BufReader::with_capacity(CAPACITY.try_into().unwrap(), file);
        Ok(FileProgress {
            name: path.clone(),
            file: Box::new(reader),
            chunks: quot,
            chunk_no: 0,
        })
    }
}

pub struct FileChunk {
    pub file_name: String,
    pub chunk: Option<Vec<u8>>
}

pub struct FileHash {
    pub name: String,
    pub hash: String,
}
