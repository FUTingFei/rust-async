use std::net::SocketAddr;
use std::convert::From;
use std::time::{Instant};


use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};

use cita_tool::{
    client::basic::{Client as CitaClient, ClientExt},
    crypto::Encryption,
    PrivateKey, TransactionOptions,
    JsonRpcParams, ParamsValue,
};
use cita_types::U256;



pub const RPC_URL: &str = "http://101.132.38.100:1337";
pub const BLOCK_NUMBER: &str = "blockNumber";

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let start = Instant::now();
    let encryption = Encryption::Secp256k1;
    let priv_key: PrivateKey = PrivateKey::from_str("b235fb8b5d4765c7bfa18d4844cc3eb39412f100e84073bf7cc2c97ecf2d446b", encryption)
        .unwrap().into();
    let chain_id = U256::from_dec_str("1").unwrap();


    let client = CitaClient::new();
    let mut client = client.set_uri(RPC_URL);
    let client = client.set_private_key(&priv_key);
    let client = client.set_chain_id(chain_id);

    let current_block_number = client.get_current_height().unwrap();

    let tx_options = TransactionOptions::new()
        .set_address("0xffffffffffffffffffffffffffffffffff010000")
        .set_value(Some(U256::from_dec_str("0").unwrap()))
        .set_current_height(Some(current_block_number))
        .set_code("0x65766964656e63653d3078666666666666666666666666666666666666666666666666666666666666666666666666666666662674696d657374616d703d31353837373233373432303030")
        .set_version(Some(2u32));

    let number_total = 10000;
    let mut txs = Vec::with_capacity(number_total as usize);

    println!("{:?}","before loop");

    for _ in 0..number_total {
        let tx = client.generate_transaction(tx_options).unwrap();
        let byte_code = client.generate_sign_transaction(&tx).unwrap();
        let params = JsonRpcParams::new()
            .insert(
                "method",
                ParamsValue::String(String::from("sendRawTransaction")),
            )
            .insert(
                "params",
                ParamsValue::List(vec![ParamsValue::String(byte_code)]),
            );
        txs.push(params);
    }

    println!("{:?}","after loop");
    let _result = client.send_request(txs.into_iter());
    println!("{:?}","after send");

    let duration1 = start.elapsed();

    let _rpc_response = client.send_raw_transaction(tx_options).unwrap();

    let duration2 = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}, {:?}", duration1, duration2);

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
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    run_server(addr).await;
}
