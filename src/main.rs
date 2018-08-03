extern crate handlebars_iron as hbs;
extern crate iron;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::HandlebarsEngine;
use hbs::Template;
use hbs::handlebars::to_json;

fn main() {
    let template_engine = setup_templates();
    setup_routes(template_engine);
}

fn setup_templates() -> HandlebarsEngine {
    let mut template_engine = hbs::HandlebarsEngine::new();
    template_engine.add(Box::new(hbs::DirectorySource::new("./src/html/", ".hbs")));
    if let Err(r) = template_engine.reload() {
        panic!("{}", r);
    }
    template_engine
}

fn setup_routes(template_engine: HandlebarsEngine) {
    let mut router = Router::new();
    router.get("/index", index_handler, "index");

    let mut chain = Chain::new(router);
    chain.link_after(template_engine);
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    let link = Link { link: "https://i.pinimg.com/originals/fe/77/26/fe77263d6fa1ec9a262a186789d5a4e6.gif".to_string() };
    let data = to_json(&link);

    let mut resp = Response::new();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
pub struct Link {
    link: String,
}
