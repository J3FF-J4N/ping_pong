use std::{
    error::Error,
    net::{Ipv4Addr, UdpSocket},
};

const IP_ECHO: (Ipv4Addr, u16) = (Ipv4Addr::new(127, 0, 0, 1), 34254);

pub fn clock_sync_test() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0)).unwrap();

    // UdpSocket
    socket
        .send_to(&[0, 0, 0, 0, 0, 0, 0b000100111, 0b00010000], IP_ECHO)
        .unwrap();
    // loop {

    // }
    // 100111_00010000

    Ok(())
}
