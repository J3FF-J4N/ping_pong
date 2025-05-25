use std::error::Error;

use clock_sync::clock_sync;
use shared::*;
pub mod clock_sync;

// const IP_CLIENT: &'static str = "127.0.0.1:34254"; //"192.168.1.69:34254";

mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // handle_connection(Side::Echo()).unwrap();
    clock_sync().await.unwrap();

    Ok(())
}
