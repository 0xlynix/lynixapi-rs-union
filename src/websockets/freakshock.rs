use std::collections::HashMap;

use actix_web_actors::ws;
use actix_web::{http::header, web, HttpRequest, HttpResponse};
use actix::{Actor, Addr, StreamHandler};
use actix::AsyncContext;


struct FreakshockWs;

impl Actor for FreakshockWs {
    type Context = ws::WebsocketContext<Self>;
    //type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[Freakshock WS] Connection established");
        println!("Address: {:?}", ctx.address());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("[Freakshock WS] Connection stopped");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FreakshockWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                // Print message that the connection is closed with IP address of client
                println!("[Freakshock WS] Connection closed");
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

pub async fn freakshock_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    // Extract the Authorization header
    let auth_header = match req.headers().get(header::AUTHORIZATION) {
        Some(value) => value,
        None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };

    println!("[Freakshock WS] Authentication in progress...");

    // Check if it's a bearer token and if the token value is "FREAKHOUND"
    if let Ok(auth_str) = auth_header.to_str() {
        if auth_str != "Bearer FREAKHOUND" {
            // Print authentication error
            println!("[Freakshock WS] Authentication error");
            return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        }
    } else {
        return Err(actix_web::error::ErrorBadRequest("Bad Request"));
    }

    let resp = ws::start(FreakshockWs, &req, stream);
    resp.map_err(actix_web::Error::from)
}