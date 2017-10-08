use iron::{Request, Response, IronResult, status};
use router::Router;

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/", handler, "api_origin");

    router
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, API!")))
}
