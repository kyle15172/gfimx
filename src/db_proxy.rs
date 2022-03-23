use mongodb::{
    bson::doc,
    sync::{Client, Database}, options::UpdateOptions,
};

use crate::file_metadata::FileMetadata;

pub enum DatabaseType {
    MongoDB,    
}

pub trait DatabaseImpl {
    fn connect(&mut self) -> Result<(), String>;
    fn get_file(&self, file_name: &str) -> Option<FileMetadata>;
    fn upsert(&self, file_info: FileMetadata) -> Result<(), String>;
}

pub struct DatabaseProxy {
    _impl: Box<dyn DatabaseImpl + Send>
}

impl DatabaseProxy {
    pub fn new(dbtype: DatabaseType) -> Self {
        let db: Box<dyn DatabaseImpl + Send> = match dbtype {
            DatabaseType::MongoDB => Box::new(MongoDbConnector::new())
        };
        DatabaseProxy { _impl: db }
    }
}

struct MongoDbConnector {
    _db: Option<Database>,
}

impl MongoDbConnector {
    pub fn new() -> Self {
        MongoDbConnector { _db: None }
    }
}

impl DatabaseImpl for MongoDbConnector {
    fn connect(&mut self) -> Result<(), String> {
        self._db = Some(
            Client::with_uri_str("mongodb://localhost:27017").unwrap().database("test")
        );
        Ok(())
    }

    fn get_file(&self, file_name: &str) -> Option<FileMetadata> {
        let collection = self._db.as_ref().unwrap().collection::<FileMetadata>("test_files");
        collection.find_one(doc! { "path": file_name }, None).unwrap()        
    }

    fn upsert(&self, file_info: FileMetadata) -> Result<(), String> {
        let collection = self._db.as_ref().unwrap().collection::<FileMetadata>("test_files");

        let options = UpdateOptions::builder()
        .upsert(Some(true))
        .build();

        let res = collection.update_one(
            doc! { "path": file_info.path.clone() }, 
            doc! {"$set":file_info.to_doc()}, 
            Some(options));

        return if let Err(reason) = res {
            Err(format!("{}", reason))
        } else {
            Ok(())
        }
    }
}