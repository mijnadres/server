use iron::{Request, Response, IronResult, IronError, status};
use iron::headers::ContentType;
use router::Router;
use serde_json;

use super::Event;

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/", handler, "api_origin");

    router
}

fn handler(_: &mut Request) -> IronResult<Response> {
    let event = Event::new("mijnadres", "Hello, World");

    match serde_json::to_string(&event) {
        Ok(message) => {
            let mut response = Response::with((status::Ok, message));
            response.headers.set(ContentType::json());
            Ok(response)
        },

        Err(e) => Err(IronError::new(e, status::InternalServerError)),
    }
}
