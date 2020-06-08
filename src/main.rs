#![feature(async_await)]

use hyper::{Body, Client, Request, Response, Server, Uri, service::{make_service_fn, service_fn}};

use std::net::SocketAddr;

use cita_tool::{
    client::basic::{Client, ClientExt},
    crypto::Encryption,
    PrivateKey, TransactionOptions, UnverifiedTransaction,
};
use cita_types::U256;


pub const RPC_URL: &str = "http://101.132.38.100:1337";


async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let tx_options = TransactionOptions::new()
                    .set_code("")
                    .set_address("0xf26e01badf4c282edd8c8c14df84dae4a5855632")
                    .set_value(Some(U256::from_str("1000000000000000000000").unwrap()));
    let client = Client::new();
    let mut client = client.set_uri(RPC_URL);
    let client = client.set_private_key("xxx");

    let rpc_response = client.send_raw_transaction(tx_options).unwrap();
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
