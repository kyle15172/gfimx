use std::{fs::File, io::{BufReader, self}};

pub struct FileProgress {
    pub file: Box<BufReader<File>>,
    pub offset: i64,
    pub chunks: u64,
    pub chunk_no: u64,
}

impl FileProgress {
    pub fn new(path: String) -> io::Result<Self> {
        let file = File::open(path)?;
        let size = file.metadata()?.len();
        let (mut quot, rem) = (size / 4096, size % 4096);
        if rem > 0 {
            quot += 1;
        }
        let reader = BufReader::with_capacity(4096, file);
        Ok(FileProgress {
            file: Box::new(reader),
            offset: 0,
            chunks: quot,
            chunk_no: 0,
        })
    }
}

pub struct FileChunkInfo {
    chunk_id: String,
    is_done: u64
}