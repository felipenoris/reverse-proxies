

extern crate hyper;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn;

extern crate futures;

use futures::future::{self, Future};
//use hyper::{Method, StatusCode};
use std::net::{IpAddr, Ipv4Addr};

///////////////////////////////
extern crate lazy_static;
extern crate unicase;
mod proxy;
///////////////////////////////

//use lazy_static::lazy_static;

//extern crate regex;
//use regex::Regex;

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

    // wait on https://github.com/hyperium/hyper/issues/1650
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    match req.uri().path() {
        "/risk-backend/hello" => hello_world(req),
        "/risk-backend/dummy" => proxy::call(client_ip, "http://127.0.0.1:13351", req),
        _ => debug_request(req),
    }
}

fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3001).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn(router)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running server on {:?}", addr);

    // Run this server for... forever!
    hyper::rt::run(server);
}
