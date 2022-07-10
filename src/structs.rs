pub struct FileChunk {
    pub file_name: String,
    pub chunk: Option<Vec<u8>>
}

pub struct FileHash {
    pub name: String,
    pub hash: String,
}
