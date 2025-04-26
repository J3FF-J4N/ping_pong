use std::error::Error;

use shared::*;

// const IP_CLIENT: &'static str = "127.0.0.1:34254"; //"192.168.1.69:34254";


mod shared;

fn main()  -> Result<(), Box<dyn Error>> {

    handle_connection(Side::Echo()).unwrap();

    Ok(())
}