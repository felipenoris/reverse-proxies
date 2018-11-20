
extern crate hyper;
extern crate futures;
extern crate lazy_static;
extern crate unicase;
extern crate regex;

use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{service_fn, make_service_fn};
use futures::future::{self, Future};
use lazy_static::lazy_static;
use regex::Regex;

mod proxy;

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

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| { // returns BoxFut

            lazy_static! {
                static ref RGX_RISK_BACKEND: Regex = Regex::new("^/risk-backend/.*$").unwrap();
            }

            if RGX_RISK_BACKEND.is_match(req.uri().path()) {
                return proxy::call(remote_addr.ip(), "http://127.0.0.1:3001", req)
            }

            match req.uri().path() {
                "/hello" => hello_world(req),
                _ => debug_request(req),
            }
        })
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running server at {:?}", addr);
    hyper::rt::run(server);
}
