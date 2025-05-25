use std::{error::Error, net::Ipv4Addr, time::Duration};

use rppal::gpio::Gpio;
use tokio::{
    io,
    net::UdpSocket,
    time::interval,
};

const IP_ECHO: (Ipv4Addr, u16) = (Ipv4Addr::new(127, 0, 0, 1), 34254);

pub async fn clock_sync() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(IP_ECHO).await.unwrap();

    let mut microtick = interval(Duration::from_millis(1));

    let mut current_microtick = 0u8;
    let mut current_macrotick = 0u64;

    let mut last_macrotick_message = 0u64;

    let mut macro_signal = Gpio::new()?.get(23).unwrap().into_output();

    macro_signal.set_low();

    let mut in_sync_signal = Gpio::new()?.get(24)?.into_output();

    in_sync_signal.set_low();

    let mut in_sync = false;

    let mut data = [0; 256];
    loop {
        match socket.try_recv(&mut data[..]) {
            Ok(_) => {

                let message = ((data[0] as u64) << 56)
                    | ((data[1] as u64) << 48)
                    | ((data[2] as u64) << 40)
                    | ((data[3] as u64) << 32)
                    | ((data[4] as u64) << 24)
                    | ((data[5] as u64) << 16)
                    | ((data[6] as u64) << 8)
                    | data[7] as u64;

                last_macrotick_message = message;

                let difference = current_macrotick as i128 - message as i128;

                if difference.abs() > 1 {
                    current_macrotick = message;
                    dbg!(&current_macrotick);
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

        microtick.tick().await;
        current_microtick += 1;

        if current_microtick == 20 {
            current_macrotick += 1;
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

            let difference = dbg!(current_macrotick - last_macrotick_message);

            if difference > 50 && difference < 100 {
                // todo!("log something");
            } else if difference >= 100 {
                in_sync = false;
            }
        }

        if in_sync == true {
            in_sync_signal.set_high();
        } else {
            in_sync_signal.set_low();
        }
    }
}
