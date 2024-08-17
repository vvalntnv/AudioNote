use mongodb::{Client, Collection};
use std::env;

use super::savable::Savable;

struct ConnectionArgs {
    username: String,
    password: String,
    host: String,
    port: isize,
}

pub struct DatabaseConnection {
    client: Client
}

impl ConnectionArgs {
    fn new(
        username: &str,
        password: &str,
        host: &str,
        port: isize
    ) -> Self {
        let username = username.to_string();
        let password = password.to_string();
        let host = host.to_string();

        ConnectionArgs {
            username, password, host, port
        } 
    } 

    fn construct_url(&self) -> String{
        let url = format!(
            "mongodb://{user}:{pass}@{host}:{port}",
            user=self.username,
            pass=self.password,
            host=self.host,
            port=self.port 
        );

        url
    }
}

impl DatabaseConnection {
    async fn new(args: ConnectionArgs) -> Self {
        let url = args.construct_url();
        println!("{}", &url);
        let client = Client::with_uri_str(&url)
            .await
            .unwrap();
        
        DatabaseConnection {
            client: client
        }
    } 

    pub async fn from_env() -> Self {
        let username = env::var("MONGO_USERNAME").unwrap();
        let password = env::var("MONGO_PASSWORD").unwrap();
        let host = env::var("MONGO_HOST").unwrap();
        let port = env::var("MONGO_PORT").unwrap()
            .parse::<isize>()
            .expect("Port couldn't be casted to integer");

        let connection_args = ConnectionArgs::new(
            &username, &password, &host, port
        );
        return Self::new(connection_args).await;
    }

    pub fn get_collection<S: Savable>(&self) -> Collection<S> {
        let database_name = S::get_database_name();
        let database = self.client.database(&database_name); 

        let collection_name = S::get_collection_name();
        let collection = database.collection(&collection_name);

        collection
    }
}
