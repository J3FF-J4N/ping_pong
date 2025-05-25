use std::{error::Error, net::{Ipv4Addr, UdpSocket}};

const IP_ECHO: (Ipv4Addr, u16) = (Ipv4Addr::new(127, 0, 0, 1), 34254);

pub fn clock_sync_test()  -> Result<(), Box<dyn Error>> {

    let socket = UdpSocket::bind(IP_ECHO)?; 

    

    socket.send_to(&[0,0,0,0,0,0,255,255], IP_ECHO).unwrap();


    Ok(())
}