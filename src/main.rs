use std::env;

use iota_streams::{
    app::transport::{
        TransportOptions,
        tangle::{
            client::{SendOptions, Client as StreamsClient}, 
            PAYLOAD_BYTES,
        },
    },
    app_channels::api::tangle::{
//                            Address, 
                            Author},
//    app_channels::api::tangle::Transport,
//    core::{
//        prelude::{ String, Rc, },
//    },
};

fn main() {
    // Initial Configuration
    let node_url = "http://116.203.221.20:14265".to_string();
    let node_mwm: u8 = env::var("MWM").map(|s| s.parse().unwrap_or(14)).unwrap_or(14);
    let mut client = StreamsClient::new_from_url(&node_url);  
        
    //let mut transport = Rc::new(RefCell::new(client));
    let mut send_opt = SendOptions::default();
    send_opt.min_weight_magnitude = node_mwm;
    send_opt.local_pow = false;
    client.set_send_options(send_opt);

    let mut author = Author::new("a", "utf-8", PAYLOAD_BYTES, false, client);

    println!("New Author");
    let channel_address = author.channel_address().unwrap().to_string();
    println!("Channel Address: {}", channel_address);

    let _announcement_message = author.send_announce();


}
