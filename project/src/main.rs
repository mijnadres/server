#[macro_use] extern crate log;
extern crate simplelog;
extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::{Iron, Chain};
use staticfile::Static;
use mount::Mount;
use simplelog::{Config, LogLevelFilter, TermLogger, CombinedLogger};

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();

    info!("Logger configured");

    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("client/")));

    let chain = Chain::new(mount);

    Iron::new(chain).http("localhost:3000").unwrap();
}
