use std::{
    error::Error,
    fs::File,
    net::{Ipv4Addr, UdpSocket},
};

use chrono::{DateTime, Local};
use std::io::Write;

/*This is used to determine who is running */
pub enum Side {
    Master(),
    Echo(),
}

const ROUNDTRIP_COUNT: usize = 50_000;
const BUFF_SIZE: usize = 256;
const IP_ECHO: (Ipv4Addr, u16) = (Ipv4Addr::new(192, 168, 1, 69), 34254); //(Ipv4Addr::LOCALHOST, 34254);//"127.0.0.1:34254"; //"192.168.1.70:34254";
const IP_MASTER: (Ipv4Addr, u16) = (Ipv4Addr::new(192, 168, 1, 21), 0); //(Ipv4Addr::LOCALHOST, 0); //"192.168.1.69:34254";

// pub fn handle_connection(side: Side) -> Result<(), Box<dyn Error>>{

//     if let Side::Server(ip) = side {

//         let socket = UdpSocket::bind(IP_SERVER).expect("Unable to bind to socket ...");

//         socket.connect(IP_CLIENT).expect(format!("Unable to connect to IP: {:?}", IP_CLIENT).as_str()); //Only client address is accepted
//         socket.send(b"buf").unwrap();

//         let start = chrono::Utc::now(); //Timing is only for server

//         let end = chrono::Utc::now(); //Stop at datapoint that should be gathered

//         let difference = end - start;

//         println!("{}", difference);

//         return Ok(());

//     } else if let Side::Client(ip) = side {

//         let mut buffer = [0u8; BUFF_SIZE];

//         // let mut buffer = String::new();

//         let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to bind to socket ...");

//         // socket.connect(IP_SERVER).expect(format!("Unable to connect to IP: {:?}", IP_SERVER).as_str()); //Only server address is accepted
//         socket.recv(&mut buffer).unwrap();

//         println!("{}", String::from_utf8_lossy(&buffer));

//         return Ok(());
//     } else {
//         unreachable!("This statement should never be reached");
//     }

// }

// pub fn handle_connection(side: Side) -> Result<(), Box<dyn Error>> {
//     if let Side::Server(_) = side {

//         let socket = UdpSocket::bind(IP_SERVER)?;
//         println!("Server listening on {:?}", IP_SERVER);

//         let mut buffer = [0u8; BUFF_SIZE];

//         // Receive data from the client
//         let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer)?;
//         println!("Received '{}' from {}", String::from_utf8_lossy(&buffer[..number_of_bytes]), src_addr);

//         // Optionally send a response back
//         socket.send_to(b"buf", src_addr)?;

//         return Ok(());
//     } else if let Side::Client(_) = side {
//         let socket = UdpSocket::bind(IP_CLIENT)?;
//         println!("Client bound to {}", socket.local_addr()?);

//         // Send data to the server
//         socket.send_to(b"Hello, server!", IP_SERVER)?;

//         let mut buffer = [0u8; BUFF_SIZE];
//         let (number_of_bytes, _) = socket.recv_from(&mut buffer)?;
//         println!("Received response: {}", String::from_utf8_lossy(&buffer[..number_of_bytes]));

//         return Ok(());
//     } else {
//         unreachable!("This statement should never be reached");
//     }
// }

//sudo ip addr add 192.168.1.21 dev enp2s0
//cross build --release --bin echo --target armv7-unknown-linux-gnueabihf

pub fn handle_connection(side: Side) -> Result<(), Box<dyn Error>> {
    if let Side::Master() = side {
        let socket = UdpSocket::bind(IP_MASTER)?;
        println!("Master bound to {:?}", IP_MASTER);

        let mut buffer = [0u8; BUFF_SIZE];

        let mut measurements: [(DateTime<Local>, DateTime<Local>); ROUNDTRIP_COUNT] =
            [(Default::default(), Default::default()); ROUNDTRIP_COUNT]; // = //[(TimeDelta::default(), TimeDelta::default()); ROUNDTRIP_COUNT];

        for i in 0..ROUNDTRIP_COUNT {
            let start = chrono::Local::now();

            socket.send_to(format!("{i}").as_bytes(), IP_ECHO)?;

            socket.recv_from(&mut buffer)?;

            let end = chrono::Local::now();

            measurements[i] = (start, end);

            // println!("Received response: {}", String::from_utf8_lossy(&buffer[..number_of_bytes]));
        }

        let mut measurement_file = File::create_new("50_000_high_CPU_load.csv").unwrap();

        writeln!(
            measurement_file,
            "{},{},{}",
            "Start - HH:MM:SS:NANOSECONDS",
            "End - HH:MM:SS:NANOSECONDS",
            "Difference in NANOSECONDS"
        )
        .unwrap();

        measurements.iter().for_each(|(start, end)| {
            writeln!(
                measurement_file,
                "{},{},{}",
                start.format("%T:%f"),
                end.format("%T:%f"),
                (*end - *start).num_nanoseconds().unwrap()
            )
            .unwrap();
        });

        return Ok(());
    } else if let Side::Echo() = side {
        let socket = UdpSocket::bind(IP_ECHO)?;
        println!("Echo bound to {}", socket.local_addr()?);

        println!("Waiting for Master to send first message ...");

        let mut buffer = [0u8; BUFF_SIZE];

        loop {
            // Receive data from the client
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer)?;
            // println!("Received '{}' from {}", String::from_utf8_lossy(&buffer[..number_of_bytes]), src_addr);

            // Optionally send a response back
            socket.send_to(b"pong", src_addr)?;
        }
    } else {
        unreachable!("This statement should never be reached");
    }
}
// /*Helper function to avoid duplicate code since the connection works the same for client and server */
// fn get_socket() -> UdpSocket{

// }
