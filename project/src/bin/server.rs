#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate logger;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate mijnadres_server as adr;

use dotenv::dotenv;
use std::path::Path;

use iron::{Iron, Chain};
use staticfile::Static;
use mount::Mount;
use simplelog::{Config, LogLevelFilter, TermLogger, CombinedLogger};
use logger::Logger;

fn main() {
    dotenv().ok();
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();

    info!("Logger configured");

    Iron::new(chain()).http("localhost:3000").unwrap();
}

fn chain() -> Chain {
    let mut chain = Chain::new(mount());
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link(adr::database::connection_pool_middleware());
    chain.link_after(logger_after);

    chain
}

fn mount() -> Mount {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("client/")));
    mount.mount("/api", adr::api::routes::router());

    mount
}
