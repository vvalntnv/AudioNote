use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fmt::Write as _;

type HmacSha256 = Hmac<Sha256>;

struct HMACSigner {
    secret: String
}


impl HMACSigner {
    pub fn new(secret: String) -> Self {
        HMACSigner {
            secret,
        }
    }
    ///Encodes the data and returns an HMAC using the 
    ///secret
    pub fn encode_data<S: AsRef<str>>(&self, data: S) -> Result<String, String> {
        let data = data.as_ref().as_bytes();    
        let mut mac = self.create_mac()?; 

        mac.update(data);
        let result = mac.finalize(); 
        let bytes = result.into_bytes();
        
        let mut output = String::new();
 
        for byte in bytes {
            write!(&mut output, "{:02x}", byte)
                .map_err(|err| err.to_string())?;
        }

        Ok(output)
    }

    pub fn verify_data<S: AsRef<str>>(&mut self, data: S, token: &str) -> bool {
        let data = data.as_ref().as_bytes();

        let mut mac = match self.create_mac() {
            Ok(mac) => mac,
            Err(_) => return false
        };

        mac.update(data);

        let data_bytes = token.as_bytes();
        match mac.verify(data_bytes.into()) {
            Ok(_) => true,
            Err(_) => false
        }
    }
    fn create_mac(&self) -> Result<HmacSha256, String> {
        HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|err| err.to_string())
    }
}
