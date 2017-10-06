extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("client/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
