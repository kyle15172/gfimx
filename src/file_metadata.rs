use serde_derive::{Serialize, Deserialize};
use mongodb::bson::{Document, doc};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadata {
    pub path: String,
    pub uid: u32,
    pub gid: u32,
    pub perms: u32,
    pub hash: String
}

impl FileMetadata {
    pub fn to_doc(&self) -> Document {
        doc!{
            "path": self.path.clone(),
            "uid": self.uid,
            "gid": self.gid,
            "perms": self.perms,
            "hash": self.hash.clone()
        }
    }
}