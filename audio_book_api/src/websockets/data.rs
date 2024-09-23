use actix_ws::{self, MessageStream, Session};
use actix_web::{web, HttpRequest, HttpResponse};

pub struct WebSocketData {
    /// Use this guy for sending
    pub session: Session,

    /// Use this guy for receiving
    pub stream: MessageStream 
}

impl WebSocketData {
    pub fn new(req: HttpRequest, body: web::Payload) -> Result<(Self, HttpResponse), String> {
        let (res, session, stream) = match actix_ws::handle(&req, body) {
            Ok(data) => data,
            Err(err) => return Err(err.to_string())
        };

        let ws_data = WebSocketData { session, stream}; 

        Ok((ws_data, res))
    }
}
