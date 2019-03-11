
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{service_fn, make_service_fn};
use futures::future::{self, Future};

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
    let addr = ([0, 0, 0, 0], 13001).into();

    let make_svc = make_service_fn(|socket: &AddrStream| {

        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| { // returns BoxFut

            if req.uri().path().starts_with("/risk-backend/") {
                return hyper_reverse_proxy::call(remote_addr.ip(), "http://127.0.0.1:13655", req)
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
