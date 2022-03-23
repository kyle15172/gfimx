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
    fn get_file(&self, file_name: &str) -> Result<Option<FileMetadata>, String>;
    fn upsert(&self, file_info: FileMetadata) -> Result<(), String>;
}

pub struct DatabaseProxy {
    _impl: Box<dyn DatabaseImpl + Send>
}

impl DatabaseProxy {
    pub fn new(dbtype: DatabaseType) -> Result<Self, String> {
        let mut db: Box<dyn DatabaseImpl + Send> = match dbtype {
            DatabaseType::MongoDB => Box::new(MongoDbConnector::new())
        };
        db.connect()?;
        Ok(DatabaseProxy { _impl: db })
    }

    pub fn get_file(&self, file_name: &str) -> Result<Option<FileMetadata>, String> {
        self._impl.get_file(file_name)
    }

    pub fn upsert(&mut self, file_info: FileMetadata) -> Result<(), String> {
        self._impl.upsert(file_info)
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

        let client = Client::with_uri_str("mongodb://192.168.1.157:27017");

        if let Err(reason) = &client {
            return Err(format!("{}", reason));
        }

        self._db = Some(
            client.unwrap().database("test")
        );
        Ok(())
    }

    fn get_file(&self, file_name: &str) -> Result<Option<FileMetadata>, String> {
        let collection = self._db.as_ref().unwrap().collection::<FileMetadata>("test_files");

        let result =collection.find_one(doc! { "path": file_name }, None);
        return match result {
            Ok(val) => Ok(val),
            Err(reason) => Err(format!("{}", reason))
        };
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