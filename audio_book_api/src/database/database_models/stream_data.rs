use mongodb::bson;
use sha2::{Sha256, Digest};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng, RngCore};
use serde::{Serialize, Deserialize};

use chrono::{DateTime, Utc, Duration};
use base64::{alphabet::URL_SAFE, engine::{GeneralPurpose, GeneralPurposeConfig}, Engine as _};

use crate::database::savable::Savable;

pub struct StreamFactory {
    book_id: String
}

impl StreamFactory {
    fn new(book_id: String) -> Self {
        StreamFactory {
            book_id
        }
    }

    fn create_stream_data(&self) -> StreamData {
        let stream_id = self.generate_stream_id();
        let valid_until = Utc::now() + Duration::minutes(20);
        let refresh_token = self.generate_token();
        let refresh_token_valid_until = valid_until + Duration::minutes(5);

        StreamData {
            stream_id, 
            valid_until: valid_until.into(), 
            refresh_token, 
            refresh_token_valid_until: refresh_token_valid_until.into(), 
            book_id: self.book_id.to_string()
        }
    }

    pub fn generate_token(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let engine = GeneralPurpose::new(&URL_SAFE, GeneralPurposeConfig::default()); 
        
        engine.encode(bytes)
    }

    pub fn generate_stream_id(&self) -> String {
        let rand_str_len = 16;

        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand_str_len)
            .map(char::from)
            .collect();

        println!("{}", &rand_string);

        let mut hasher = Sha256::new();
        hasher.update(rand_string);
        let result = hasher.finalize();

        let hex: String = result.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

        println!("{}", hex);

        hex
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamData {
    stream_id: String,
    valid_until: bson::DateTime,
    refresh_token: String,
    refresh_token_valid_until: bson::DateTime, 
    book_id: String
}

impl Savable for StreamData {
    const DATABASE_NAME: &'static str = "audio_note";
    const COLLECTION_NAME: &'static str = "streams";
}
