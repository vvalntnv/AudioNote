use actix_web::web;

use super::{errors::StreamError, external_models::StreamGeneratedResponse};

pub fn stream_scope(cfg: &mut web::ServiceConfig) {

}

type StreamResult<T> = Result<T, StreamError>;

pub async fn generate_stream_link() 
-> StreamResult<web::Json<StreamGeneratedResponse>> 
{
    todo!()
}
