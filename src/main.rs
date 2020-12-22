use tungstenite::{connect, Message};
use url::Url;
use serde_json::json;
use serde_json::{Result, Value};

//https://docs.pro.coinbase.com/#protocol-overview
fn main() {
  //  env_logger::init();

    let nc = nats::connect("127.0.0.1:4222").unwrap();
    let (mut socket, response) =
        connect(Url::parse("wss://ws-feed.pro.coinbase.com").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let subscription = json!({
        "type": "subscribe",
        "product_ids" : ["BTC-USD"],
        "channels": [ "heartbeat", { "name":"ticker","product_ids":["BTC-USD"]}]
    });
 
    socket.write_message(Message::Text(subscription.to_string().into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let v: Value = serde_json::from_str(&msg.to_string()).unwrap();
   //     println!("Received: {}", msg, );
        if v["type"] == "ticker" {
            println!("Trade {} {} {}", v["price"],v["last_size"], v["side"] );
            nc.publish("trade", &msg.to_string());

        }
    }
    // socket.close(None);
}