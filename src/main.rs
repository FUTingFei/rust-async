#![feature(async_await)]

use hyper::{Body, Client, Request, Response, Server, Uri, service::{make_service_fn, service_fn}};

use std::net::SocketAddr;



async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(
        make_service_fn(|_| {
            async {
                {
                    Ok::<_, hyper::Error>(service_fn(serve_req))
                }
            }
        }));

    if let Err(e) = serve_future.await {
        eprintln!("server err: {}", e);
    }
}
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    run_server(addr).await;
}
