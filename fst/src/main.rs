

extern crate hyper;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn;

extern crate futures;

use futures::future::{self, Future};
use std::net::{IpAddr, Ipv4Addr};

extern crate lazy_static;
extern crate unicase;
mod proxy;

use lazy_static::lazy_static;

extern crate regex;
use regex::Regex;

type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn debug_request(req: Request<Body>) -> BoxFut {
    let body_str = format!("{:?}", req);
    let response = Response::new(Body::from(body_str));
    Box::new(future::ok(response))
}

fn hello_world(_req: Request<Body>) -> BoxFut {
    let response = Response::new(Body::from("Hello World!!!"));
    Box::new(future::ok(response))
}

fn router(req: Request<Body>) -> BoxFut {

    lazy_static! {
        static ref RGX_RISK_BACKEND: Regex = Regex::new("^/risk-backend/.*$").unwrap();
    }

    // wait on https://github.com/hyperium/hyper/issues/1650
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    if RGX_RISK_BACKEND.is_match(req.uri().path()) {
        return proxy::call(client_ip, "http://127.0.0.1:3001", req)
    }

    match req.uri().path() {
        "/hello" => hello_world(req),
        _ => debug_request(req),
    }
}

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let new_svc = || {
        service_fn(router)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running server at {:?}", addr);
    hyper::rt::run(server);
}
