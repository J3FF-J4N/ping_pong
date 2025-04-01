use std::{error::Error, net::UdpSocket};



pub enum Side<'a> {
    Server(&'a str),
    Client(&'a str),
}

pub fn handle_connection(side: Side) -> Result<(), Box<dyn Error>>{

    if let Side::Server(ip) = side {

        let socket = UdpSocket::bind(ip).expect("Unable to bind to socket ...");

        let now = chrono::Utc::now();

        return Ok(());

    } else if let Side::Client(ip) = side {

        let socket = UdpSocket::bind(ip).expect("Unable to bind to socket ...");
        
        return Ok(());
    } else {
        unreachable!("This statement should never be reached");
    }

}