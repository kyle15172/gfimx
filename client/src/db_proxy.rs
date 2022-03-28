use mongodb::{
    bson::doc,
    sync::{Client, Database}, options::UpdateOptions,
};

use crate::file_metadata::FileMetadata;

const HOST: Option<&str> = option_env!("MONGO_HOST");
const PORT: Option<&str> = option_env!("MONGO_PORT");

/// Enum for choosing which database to use
pub enum DatabaseType {
    MongoDB,    
}

/// Trait that all database implementations will implement.
pub trait DatabaseImpl {
    /// Connects to a database
    /// 
    /// # Returns
    /// * `Ok(())` - A Unit object indication connection was successful
    /// * `Err(String)` - Reason as to why the connection failed
    fn connect(&mut self) -> Result<(), String>;

    /// Retrieve information for the given file
    /// 
    /// # Arguments
    /// * `file_name` - The full path of the file to get information about
    /// 
    /// # Returns
    /// * `Ok(Option(FileMetadata))` - Information about the file if present in the database or a `None`
    /// * `Err(String)` - Reason as to why retrieving file information failed
    fn get_file(&self, file_name: &str) -> Result<Option<FileMetadata>, String>;

    /// Upserts file information
    /// 
    /// # Arguments
    /// * `file_info` - Dataclass containing the file information
    /// 
    /// # Returns
    /// * `Ok(())` - A Unit object if upsertion was successful
    /// * `Err(String)` - Reason as to why upserting failed 
    fn upsert(&self, file_info: FileMetadata) -> Result<(), String>;
}

/// Proxy object for the rest of the system to interface with databases.
/// 
/// Holds an implementation class that it uses to perform the desired
/// operation
pub struct DatabaseProxy {
    _impl: Box<dyn DatabaseImpl + Send>
}

impl DatabaseProxy {

    /// Factory method that creates an implementation based on the database
    /// type provided. Connects to the database before returning a new instance.
    /// 
    /// # Arguments
    /// * `dbtype` - The type of database required
    /// 
    /// # Returns
    /// * `Ok(Self)` - The object if initialization was successful
    /// * `Err(String)` - Reason as to why initialization failed
    pub fn new(dbtype: DatabaseType) -> Result<Self, String> {
        let mut db: Box<dyn DatabaseImpl + Send> = match dbtype {
            DatabaseType::MongoDB => Box::new(MongoDbConnector::new())
        };
        db.connect()?;
        Ok(DatabaseProxy { _impl: db })
    }

    /// Retrieve information for the given file
    /// 
    /// # Arguments
    /// * `file_name` - The full path of the file to get information about
    /// 
    /// # Returns
    /// * `Ok(Option(FileMetadata))` - Information about the file if present in the database or a `None`
    /// * `Err(String)` - Reason as to why retrieving file information failed
    pub fn get_file(&self, file_name: &str) -> Result<Option<FileMetadata>, String> {
        self._impl.get_file(file_name)
    }

    /// Upserts file information
    /// 
    /// # Arguments
    /// * `file_info` - Dataclass containing the file information
    /// 
    /// # Returns
    /// * `Ok(())` - A Unit object if upsertion was successful
    /// * `Err(String)` - Reason as to why upserting failed
    pub fn upsert(&mut self, file_info: FileMetadata) -> Result<(), String> {
        self._impl.upsert(file_info)
    }
}

/// MongoDB implementation for `DatabaseImpl`
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

        let host = HOST.ok_or("MONGO_HOST variable not set!")?;
        let port = PORT.unwrap_or("27017");

        let client = Client::with_uri_str(format!("mongodb://{}:{}", host, port));

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