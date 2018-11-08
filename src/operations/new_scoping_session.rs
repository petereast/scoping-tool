use actix_web::{AsyncResponder, Error, HttpResponse, Json, State};
use futures::future::{ok as FutOk, Future};
use std::env;

use events::*;
use http_interface::*;
use state::*;
use utils::generate_id;

pub fn new_scoping_session(
    (payload, state): (Json<NewScopingSessionCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let session_id = generate_id();

    state
        .outgoing_events
        .send(SystemEvents::StartNewSessionEvent(StartNewSessionEvent {
            session_id: session_id.clone().into(),
            session_title: payload.title.clone().into(),
            session_description: payload.description.clone().into(),
        })).unwrap();

    state.redis.emit(
        StartNewSessionEvent {
            session_id: session_id.clone().into(),
            session_title: payload.title.clone().into(),
            session_description: payload.description.clone().into(),
        },
        "scoping.StartNewSession".into(),
    );

    let app_url = match env::var("URL") {
        Ok(url) => url,
        Err(_) => "http://localhost:8008/app/scope/".into(),
    };

    let submission_url = format!("{}{}", app_url, session_id);

    state.logger.log(format!(
        "[Request] new_scoping_session: {:?}\n[       ] Url: {}",
        payload, submission_url
    ));
    let response = NewScopingSessionOkResponse::new(session_id, submission_url.into());

    println!("[Response] Ok: {:?}", response);
    FutOk(HttpResponse::Ok().json(response)).responder()
}
