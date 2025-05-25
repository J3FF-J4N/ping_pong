// use std::{error::Error, net::{Ipv4Addr, UdpSocket}, time::Duration};



use std::{error::Error, net::Ipv4Addr, time::Duration};

// use std::Duration;
use tokio::{io::{self, Interest}, net::UdpSocket, time::{interval, Interval}, try_join};
use rppal::gpio::Gpio;

// const IP_ECHO: (Ipv4Addr, u16) = (Ipv4Addr::new(192, 168, 1, 69), 34254);
const IP_ECHO: (Ipv4Addr, u16) = (Ipv4Addr::new(127, 0, 0, 1), 34254);


pub async fn clock_sync() -> Result<(), Box<dyn Error>>{
    
    let socket = UdpSocket::bind(IP_ECHO).await.unwrap(); 

    let mut microtick = interval(Duration::from_millis(1));

    let mut current_microtick = 0u8;
    let mut current_macrotick = 0u64;

    let mut last_macrotick_message = 0u64;

    let mut macro_signal = Gpio::new()?.get(23).unwrap().into_output();

    let mut in_sync_signal = Gpio::new()?.get(24)?.into_output();

    // let mut macro_pin = macro_signal.get(23)?.into_output();

    let mut in_sync = false;

    // let mut buffer = [0u8; 256];

    loop {

        let ready = socket.ready(Interest::READABLE);//.await.unwrap();


        if let Ok(ready) =  try_join!(ready){
            if ready.0.is_readable() {
            // The buffer is **not** included in the async task and will only exist
            // on the stack.
            let mut data = [0; 256];
            match socket.try_recv(&mut data[..]) {
                Ok(n) => {
                    
                    println!("received {:?}", &data[..n]);

                    let message = ((data[0] << 56) | (data[1] << 48) | (data[2] << 40) | (data[3] << 32) | (data[4] << 24) | (data[5] << 16) | (data[6] << 8) | data[7]) as u64;

                    last_macrotick_message = message;

                    let difference = current_macrotick  as i128 - message  as i128;

                    if difference.abs() > 1 {
                        current_macrotick = message;
                        in_sync = true;
                    } else {
                        in_sync = true;
                    }

                }
                // False-positive, continue
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    println!("{e}");
                }
            }
        }
        } 

        
        

        microtick.tick().await;
        current_microtick += 1;

        if current_microtick == 20 {
            current_macrotick +=1;
            current_microtick = 0;

            macro_signal.set_high();
            
            if current_macrotick as f64 % 10.0 != 0.0 {

                if current_microtick == 1 {
                    macro_signal.set_low();
                }

            } else {

                if current_microtick == 2 {
                    macro_signal.set_low();
                }

            }

            let difference = current_macrotick - last_macrotick_message;

            if difference > 50 && difference < 100{
                todo!("log something");
            } else if difference > 100 {
                in_sync = false;
            }

        }

        if in_sync == true {
            in_sync_signal.set_high();
            dbg!(&in_sync);
        } else {
            in_sync_signal.set_low();
        }
    }

}