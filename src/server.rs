use std::{error::Error, net::UdpSocket, time::Instant};

use shared::*;


const IP_ALEX: &'static str = "192.168.1.70:10520";
const ROUNDTRIP_COUNT: usize = 50_000;

mod shared;
fn main()  -> Result<(), Box<dyn Error>> {

    // let socket = UdpSocket::bind(IP_JEFF).expect("Unable to bind to socket ...");

    

    handle_connection(Side::Server(IP_ALEX)).unwrap();

    println!("Hello, world!");

    Ok(())
}