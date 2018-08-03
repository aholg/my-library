extern crate handlebars_iron as hbs;
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::HandlebarsEngine;

fn main() {
    let template_engine = setup_templates();
    setup_routes(template_engine);
}

fn setup_templates() -> HandlebarsEngine {
    let mut templateEngine = hbs::HandlebarsEngine::new();
    templateEngine.add(Box::new(hbs::DirectorySource::new("./src/html/", ".hbs")));
    if let Err(r) = templateEngine.reload() {
        panic!("{}", r);
    }
    templateEngine
}

fn setup_routes(mut templateEngine: HandlebarsEngine) {
    let mut router = Router::new();
    router.get("/index", index_handler, "index");

    let mut chain = Chain::new(router);
    chain.link_after(templateEngine);
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(hbs::Template::new("index", "")).set_mut(status::Ok);
    Ok(resp)
}
