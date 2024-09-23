use std::path::Path;

use actix_ws::{AggregatedMessage, AggregatedMessageStream, CloseCode, CloseReason, Session};
use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncReadExt};
use fs2::FileExt;

use super::data::WebSocketData;

type ConnClosed = bool;

pub struct ProgressSocket {
    book_id: String
}

impl ProgressSocket {
    pub fn new(book_id: String) -> Self {
        ProgressSocket { book_id }
    }

    /// Runs the socket handler in a separate green thread
    /// It consumes the websocket_data
    pub async fn initialize(&self, websocket_data: WebSocketData) { 
        let mut stream = websocket_data.stream
            .aggregate_continuations();
        let mut session = websocket_data.session;
        let task = self.run(stream, session);

        // TODO: Tokio tutorial
        tokio::spawn(async move {
            task.await
        }).await;
    }

    pub async fn run(
        &self, 
        mut stream: AggregatedMessageStream,
        mut session: Session
    ) {
        
        loop {
            let res = self.handle_incoming(&mut stream, &mut session).await;
            match res {
                Ok(false) => 
                    if let Some(close_reason) = self.send_data_to(&mut session)
                        .await 
                    {
                        session.close(Some(close_reason)).await.unwrap();
                        break;
                    }
                Ok(true) => {
                    session.close(None).await.unwrap();
                    break;
                },
                Err(err) => {
                    let close_reason = CloseReason {
                        code: CloseCode::Error,
                        description: Some(err)
                    };
                    session.close(Some(close_reason)).await.unwrap();
                    break;
                }
            };
        } 
    }

    async fn send_data_to(&self, session: &mut Session) -> Option<CloseReason> {
        let mut file = if let Some(path) = self.get_log_file_path() {
            File::options()
                .read(true)
                .open(path.as_ref())
                .await
                .unwrap() 
        } else {
            let close_reason = CloseReason {
                code: CloseCode::Normal,
                description: Some("There is no progress made".to_owned())
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
        stream: &mut AggregatedMessageStream, 
        session: &mut Session
    ) -> Result<ConnClosed, String> {
        while let Some(message) = stream.next().await {
            match message {
                Ok(AggregatedMessage::Ping(msg)) => {
                    session.pong(&msg).await.unwrap(); 
                },
                Ok(AggregatedMessage::Close(_)) => {
                    return Ok(true)
                },
                Err(err) => {
                    return Err(err.to_string())
                },
                _ => {}
            }
        }
        Ok(false)
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
