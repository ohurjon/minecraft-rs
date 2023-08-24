use std::net::{TcpStream};
use std::io::{Read, Write};

use std::str::from_utf8;

fn main() {
    let protocol_version = 763;
    let address = "mc.hypixel.net";
    let port = 25565;
    match TcpStream::connect(format!("{}:{}",address,port)) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port {}",port);

            let data= get_server_request_packet(0x00,protocol_version,address,port,1);
            let data2: &[u8] = &[0x01, 0x00];

            let mut buffer= Vec::new();

            stream.write(&*data).unwrap();
            stream.write(data2).unwrap();
            println!("Sent Hello, awaiting reply...");

            let bytes_read = stream.read_to_end(&mut buffer).unwrap();

            let received_data = from_utf8(&buffer[7..bytes_read]).unwrap();
            println!("수신한 데이터: {:?}", received_data);

            Ok(())
        },
        Err(e) => Err({
            println!("Failed to connect: {}", e);
        })
    }.expect("TODO: panic message");
    println!("Terminated.");
}

fn get_server_request_packet(packet_id: i32, protocol_version: i32, address : &str, port: u16, next_state: i32) -> Vec<u8> {
    let mut buffer :Vec<u8>= Vec::new();
    buffer.append(&mut i32_to_varint(packet_id));
    buffer.append(&mut i32_to_varint(protocol_version));
    buffer.push(address.as_bytes().len() as u8);
    buffer.append(&mut address.as_bytes().to_vec());
    buffer.append(&mut port.to_be_bytes().to_vec());
    buffer.append(&mut i32_to_varint(next_state));
    let mut data : Vec<u8> = Vec::new();
    data.push(buffer.len() as u8);
    data.append(&mut buffer);
    return data;
}
fn i32_to_varint(number: i32) -> Vec<u8>{
    let mut tn : u32 = u32::from_be_bytes(number.to_be_bytes());
    let mut buffer: Vec<u8> = Vec::new();
    let bit = 127;
    loop {
        buffer.push((tn & bit) as u8);
        tn >>= 7;

        if tn == 0  {
            break;
        } else {
            if let Some(last) = buffer.last_mut() {
                *last += 128;
            }
        }
    }
    return buffer;
}
