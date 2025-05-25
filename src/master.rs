use std::error::Error;

use shared::*;

mod clock_sync_test;

// const IP_SERVER: &'static str = "127.0.0.1:34254"; //"192.168.1.70:34254";

mod shared;
fn main()  -> Result<(), Box<dyn Error>> {

    // let socket = UdpSocket::bind(IP_JEFF).expect("Unable to bind to socket ...");
    
    // handle_connection(Side::Master()).unwrap();

    clock_sync_test::clock_sync_test().unwrap();

    Ok(())
}