
extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(req: &HttpRequest) -> String {
    let msg = format!("###### Dummy Service got Request #######\n{:?}\n", req);
    println!("{}", msg);
    msg
}

fn main() {
    let addr = "127.0.0.1:13656";
    println!("dummy running at {}/risk-backend/dummy", addr);

    server::new(|| App::new()
            .resource("/risk-backend/dummy", |r| r.f(index))
        )
        .no_http2()
        .bind(addr)
        .unwrap()
        .run();
}
