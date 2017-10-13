use iron::{Request, Response, IronResult, IronError, Plugin, status};
use iron::headers::ContentType;
use router::Router;
use persistent::Read;
use serde_json;
use diesel::prelude::*;
use super::super::database::ConnectionPool;

use super::Event;

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/", handler, "api_origin");

    router
}

fn handler(request: &mut Request) -> IronResult<Response> {
    use super::super::schema::events::dsl::*;

    let arc = request.get::<Read<ConnectionPool>>().unwrap();
    let connection = arc.get().unwrap();

    let past_events: Vec<Event> = events
        .limit(5)
        .load::<Event>(&*connection)
        .expect("Error loading past events");

    let default_event = Event::new("default", "default");
    let event = past_events.first().unwrap_or(&default_event);

    match serde_json::to_string(&event) {
        Ok(m) => {
            let mut response = Response::with((status::Ok, m));
            response.headers.set(ContentType::json());
            Ok(response)
        },

        Err(e) => Err(IronError::new(e, status::InternalServerError)),
    }
}
