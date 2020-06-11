use std::net::SocketAddr;
use std::convert::From;
use std::thread;

use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};

use cita_tool::{
    client::basic::{Client as CitaClient, ClientExt},
    crypto::Encryption,
    PrivateKey, TransactionOptions,
    JsonRpcParams, ParamsValue,
};
use cita_types::U256;



pub const RPC_URL: &str = "http://10.10.20.164:1337";


async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encryption = Encryption::Secp256k1;
    let priv_key: PrivateKey = PrivateKey::from_str("b235fb8b5d4765c7bfa18d4844cc3eb39412f100e84073bf7cc2c97ecf2d446b", encryption)
        .unwrap()
        .into();
    let tx_options = TransactionOptions::new()
        .set_code("")
        .set_address("0xf26e01badf4c282edd8c8c14df84dae4a5855632")
        .set_value(Some(U256::from_dec_str("1000000000000000000000").unwrap()));

    let client = CitaClient::new();
    let mut client = client.set_uri(RPC_URL);
    let client = client.set_private_key(&priv_key);

    let number_subthread = 8;
    let number_total = 8000;
    let number_part = 1000;
    let mut txs = Vec::with_capacity(number_total as usize);
    
    println!("{:?}","before loop");

    for i in 0..number_subthread {
        let mut tx_sub = Vec::with_capacity(number_part as usize);

        let handle = thread::spawn(move || {
            for _ in 0..&number_part {
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
                tx_sub.push(params);
            }
        });

        txs.append(&mut tx_sub);
    }

    /*
    for _ in 0..number_part {
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
    */
    println!("{:?}","after loop");
    let _result = client.send_request(txs.into_iter());
    println!("{:?}","after send");

    let _rpc_response = client.send_raw_transaction(tx_options).unwrap();
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    run_server(addr).await;
}
