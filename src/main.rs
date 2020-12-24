use serde_json::json;
use serde_json::{Result, Value};
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use tungstenite::{connect, Message};
use url::Url;

struct PriceLevel {
    price: i32,
    qty: f64
}

struct MarketDepthSide {
    levels: BTreeMap<i32, f64>,
    topLevels: Vec<PriceLevel>,
    bestLevel: PriceLevel
}
struct MarketDepth {
    bids: MarketDepthSide,
    asks: MarketDepthSide,
}

// have the top most x levels. say 5
impl MarketDepthSide {
    pub fn new() -> MarketDepthSide {
        MarketDepthSide {
            levels: BTreeMap::new(),
            topLevels: Vec::new(),
            bestLevel: PriceLevel { price: 0, qty: 0.0}
        }
    }

    fn change(&mut self, price: i32, qty: f64) {
        if qty == 0.0 {
            self.levels.remove(&price);
        } else {
            self.levels.insert(price, qty);
        }
    }
    fn add(&mut self, price: i32, qty: f64) {
        self.levels.insert(price, qty);
    }

    fn sort(& mut self){

        if self.levels.len() > 5 {
          //  let mut iter = self.levels.iter().next_back();
            //self.topLevels.insert(
        
        }

    }
    fn dump(&mut self) {
        /*
        for (key, val) in &self.levels {
            print!(" [{key}] ({val}) ", key = key, val = val);
        }*/
        let (max_key, max_value) = self.levels.iter().next_back().unwrap(); 
        let (min_key, min_value) = self.levels.iter().next().unwrap();
        let mut show = false;
        if self.bestLevel.price != *max_key {
            self.bestLevel.price = *max_key;
            show = true;
        }
        if self.bestLevel.qty != *max_value {
            self.bestLevel.qty = *max_value;
            show = true;
        }
        if show {
            println!("Bid {} {}", self.bestLevel.price, self.bestLevel.qty);
        }
    }
}

impl MarketDepth {
    pub fn new() -> MarketDepth {
        MarketDepth {
            bids: MarketDepthSide::new(),
            asks: MarketDepthSide::new(),
        }
    }

    fn change_bid(&mut self, price: i32, qty: f64) {
        self.bids.change(price, qty);
    }
    fn add_bid(&mut self, price: i32, qty: f64) {
        self.bids.add(price, qty);
    }

    fn dump(&mut self) {
        self.bids.dump();
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

    socket
        .write_message(Message::Text(subscription.to_string().into()))
        .unwrap();
    let mut depth = MarketDepth::new();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let v: Value = serde_json::from_str(&msg.to_string()).unwrap();

        match v["type"].as_str().unwrap() {
            "ticker" => {
                //        println!("Trade {} {} {}", v["price"],v["last_size"], v["side"] );
                //      nc.publish("trade", &msg.to_string());
            }
            "l2update" => {
                let changes = v["changes"].as_array().unwrap();
                for change in changes.iter() {
                    let info = change.as_array().unwrap();
                    let f = info[1].as_str().unwrap().parse::<f32>().unwrap();
                    let q = info[2].as_str().unwrap().parse::<f64>().unwrap();
                    let v = (f * 100.0) as i32;
                    if info[0] == "buy" {
                        depth.change_bid(v, q);
                    }
                    //println!("Change {}-{}-{}-{}", info[0], info[1], info[2], v);
                }
                depth.dump();
            }
            "snapshot" => {
                //Received: {"type":"snapshot","product_id":"BTC-USD","asks":[["23324.53","0.85790064"],["23327.72","0.05902916"],["23328.11","0.52000000"],["23328.22","0.26000000"],["23328.23","0.12000000"],["23328.25","0.35000000"],["23328.31","0.85775608"],["
                //println!("Received: {}", msg);
                let bids = v["bids"].as_array().unwrap();
                for bid in bids.iter(){
                    let info = bid.as_array().unwrap();

                    let f = info[0].as_str().unwrap().parse::<f32>().unwrap();
                    let q = info[1].as_str().unwrap().parse::<f64>().unwrap();
                    let v = (f * 100.0) as i32;
                    depth.add_bid(v,q);
                }
                depth.dump();
            }
            _ => {
                println!("unknown {}", v["type"])
            }
        }
    }
    // socket.close(None);
}
