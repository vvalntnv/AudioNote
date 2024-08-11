use serde::{de::DeserializeOwned, Serialize};

pub trait Savable: Serialize + DeserializeOwned + Send + Sync + Unpin{
    const DATABASE_NAME: &'static str;
    const COLLECTION_NAME: &'static str;  
    
    fn get_database_name() -> String {
        Self::DATABASE_NAME.to_string()
    }

    fn get_collection_name() -> String {
        Self::COLLECTION_NAME.to_string()
    }
}
