use std::{error::Error, net::UdpSocket, time::Instant};

use shared::*;

const IP_JEFF: &'static str = "192.168.1.69:10520";
const ROUNDTRIP_COUNT: usize = 50_000;

mod shared;

fn main()  -> Result<(), Box<dyn Error>> {

    handle_connection(Side::Client(IP_JEFF)).unwrap();

    println!("Hello, world!");

    Ok(())
}