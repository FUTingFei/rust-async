use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};

use std::net::SocketAddr;

use cita_tool::{
    client::basic::{Client, ClientExt},
    crypto::Encryption,
    PrivateKey, TransactionOptions,
    JsonRpcParams, ParamsValue,
};
use cita_types::U256;
use cita_cli::printer::Printer;



pub const RPC_URL: &str = "http://xxx:1337";


async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encryption = Encryption::Secp256k1;
    let priv_key: PrivateKey = PrivateKey::from_str("xxxxx", encryption)
        .unwrap()
        .into();
    let tx_options = TransactionOptions::new()
        .set_code("")
        .set_address("0xf26e01badf4c282edd8c8c14df84dae4a5855632")
        .set_value(Some(U256::from_dec_str("1000000000000000000000").unwrap()));

    let client = Client::new();
    let mut client = client.set_uri(RPC_URL);
    let client = client.set_private_key(&priv_key);

    let number = 1000;
    let mut txs = Vec::with_capacity(number as usize);
    for _ in 0..number {
        let tx = client.generate_transaction(tx_options).map_err(|err| format!("{}", err))?;
        let byte_code = client.generate_sign_transaction(&tx).map_err(|err| format!("{}", err))?;
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
    let result = client.send_request(txs.into_iter()).map_err(|err| format!("{}", err))?;
    printer.println(&json!(result), true);

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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    run_server(addr).await;
}
