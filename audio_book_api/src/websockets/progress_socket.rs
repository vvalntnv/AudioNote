use std::path::Path;
use std::time::{Duration, Instant};
use std::pin::pin;

use actix_ws::{self, AggregatedMessage, AggregatedMessageStream, CloseCode, CloseReason, Session};
use futures_util::{future::{self, Either}, StreamExt};
use actix_web::rt;
use tokio::time::interval;
use tokio::{fs::File, io::AsyncReadExt};

use super::data::WebSocketData;

type IncommingMessage = Result<AggregatedMessage, actix_ws::ProtocolError>;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ProgressSocket {
    book_id: String
}

impl ProgressSocket {
    pub fn new(book_id: String) -> Result<Self, String> {  
        let socket = ProgressSocket { book_id };

        if let None = socket.get_log_file_path() {
            Err("The book is not in progress!".to_owned())
        } else {
            Ok(socket)
        }
    }

    /// Runs the socket handler in a separate green thread
    /// It consumes the websocket_data
    /// It also consumes self
    pub fn initialize(self, websocket_data: WebSocketData) { 
        let stream = websocket_data.stream
            .aggregate_continuations();
        let session = websocket_data.session;

        // TODO: Tokio tutorial
        rt::spawn(async move {
            self.run(stream, session).await;
        });
    }

    async fn run(
        &self,
        mut stream: AggregatedMessageStream,
        mut session: Session,
    ) {
        let mut last_heartbeat = Instant::now();
        let mut interval = interval(HEARTBEAT_INTERVAL);
        
        let close_reason = loop {
            let tick = pin!(interval.tick());

            match future::select(stream.next(), tick).await {
                Either::Left((message, _)) => {
                    if let Some(m) = message {
                        match self.handle_incoming(m, &mut session).await {
                            Some(reason) => break reason,
                            _ => last_heartbeat = Instant::now()
                        }
                    } else {
                        ()
                    }
                },
                Either::Right((_instant, _)) => {
                    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                        break CloseReason {
                            code: CloseCode::Away,
                            description: Some("Client took too long".to_owned())
                        }
                    }
                    let _ = session.ping(b"").await;
                    if let Some(reason) = self.send_data_to(&mut session).await {
                        break reason
                    }
                }
            };
        };

        let _ = session.close(Some(close_reason)).await;
    }

    async fn send_data_to(&self, session: &mut Session) -> Option<CloseReason> {
        let mut file = if let Some(path) = self.get_log_file_path() {
            File::options()
                .read(true)
                .create(false)
                .open(path.as_ref())
                .await
                .unwrap() 
        } else {
            let close_reason = CloseReason {
                code: CloseCode::Normal,
                description: Some("There is no progress being made".to_owned())
            };
            return Some(close_reason);
        };

        let mut buffer = String::new();
        let read_result = file.read_to_string(&mut buffer).await;

        match read_result {
            Ok(_) => {
                if let Err(_) = session.text(buffer.as_ref()).await {
                    Some(CloseReason {
                        code: CloseCode::Normal,
                        description: Some("you did it :(".to_string())
                    }) 
                } else {
                    None
                } 
            },
            Err(err) => Some(CloseReason {
                code: CloseCode::Error,
                description: Some(err.to_string())
            })
        }
    }

    async fn handle_incoming(
        &self, 
        message: IncommingMessage,
        session: &mut Session
    ) -> Option<CloseReason> {
        match message {
            Ok(AggregatedMessage::Ping(msg)) => {
                session.pong(&msg).await.unwrap(); 
                None
            },
            Ok(AggregatedMessage::Close(_)) => {
                Some(CloseReason { 
                    code: CloseCode::Normal, 
                    description: None 
                }) 
            },
            Ok(AggregatedMessage::Text(msg)) => {
                session.text(msg).await.unwrap();
                None
            },
            Err(err) => {
                Some(CloseReason { 
                    code: CloseCode::Error, 
                    description: Some(err.to_string()) 
                })
            },
            _ => None
        }
    }

    fn get_log_file_path(&self) -> Option<Box<Path>> {
        let path = format!("logs/{}.log", &self.book_id);
        let path = Path::new(&path);

        if !path.exists() {
            return None 
        }

        if !path.is_file() {
            return None
        }

        Some(Box::from(path))
    }
}
