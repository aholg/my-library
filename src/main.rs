extern crate iron;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use std::path::Path;
use mount::Mount;
use staticfile::Static;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/index", Static::new(Path::new("src/html/index.html")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
