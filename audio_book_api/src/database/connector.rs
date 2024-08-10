use mongodb::Client;
use std::env;

struct ConnectionArgs {
    username: String,
    password: String,
    host: String,
    port: isize,
}

struct DatabaseConnection {
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
        format!(
            "mongodb://{user}:{pass}@{host}:{port} ",
            user=self.username,
            pass=self.password,
            host=self.host,
            port=self.port 
        ) 
    }
}

impl DatabaseConnection {
    async fn new(args: ConnectionArgs) -> Self {
        let url = args.construct_url();
        let client = Client::with_uri_str(url)
            .await
            .unwrap();
        
        DatabaseConnection {
            client: client
        }
    } 

    async fn from_env() -> Self {
        let username = env::var("MONGO_USERNAME").unwrap();
        todo!()
    }
}
