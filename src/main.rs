use tungstenite::{connect, Message};
use url::Url;
use serde_json::json;
use serde_json::{Result, Value};
use std::collections::BTreeMap;

struct MarketDepth {

    bids: BTreeMap<i32,f64>,
    asks: BTreeMap<i32,f64>,
}

impl MarketDepth {
    pub fn new() -> MarketDepth {
        MarketDepth {
            bids : BTreeMap::new(),
            asks : BTreeMap::new() 
        }
    }

    fn change_bid(&mut self, price: i32, qty: f64)  {
        if qty == 0.0{
            self.bids.remove(&price);
            println!("zero");
        }
        else {
            self.bids.insert(price, qty);
        }
    }

    fn dump(&self){
      
        println!("Start");
        for (key, val) in &self.bids {
            print!("Key={key}, Value={val}", key=key, val=val);
        }
        println!("");
    }
}


//https://docs.pro.coinbase.com/#protocol-overview
fn main() {
  //  env_logger::init();

    //let nc = nats::connect("127.0.0.1:4222").unwrap();
    let (mut socket, response) =
        connect(Url::parse("wss://ws-feed.pro.coinbase.com").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let product = "BTC-USD";
    let subscription = json!({
        "type": "subscribe",
        "product_ids" : [product],
        "channels": [ "heartbeat", "level2", { "name":"ticker","product_ids":[product]}]
    });
 
    socket.write_message(Message::Text(subscription.to_string().into())).unwrap();
    let mut depth = MarketDepth::new();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let v: Value = serde_json::from_str(&msg.to_string()).unwrap();
     //   println!("Received: {}", msg, );
        if v["type"] == "ticker" {
            println!("Trade {} {} {}", v["price"],v["last_size"], v["side"] );
      //      nc.publish("trade", &msg.to_string());

        }
        else if v["type"] == "l2update" {
            let changes = v["changes"].as_array().unwrap();
            for change in changes.iter() { 
                let info = change.as_array().unwrap();
                let f = info[1].as_str().unwrap().parse::<f32>().unwrap();
                let q = info[2].as_str().unwrap().parse::<f64>().unwrap();
                let v = (f * 100.0) as i32;
                depth.change_bid(v, q);
            //    println!("Change {}-{}-{}-{}", info[0], info[1], info[2], v);
            }
            depth.dump();
        }
    }
    // socket.close(None);
}