extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::{Iron, Chain};
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("client/")));

    let chain = Chain::new(mount);

    Iron::new(chain).http("localhost:3000").unwrap();
}
