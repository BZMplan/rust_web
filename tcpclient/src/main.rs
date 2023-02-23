use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("localhost:3111").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!("Response:{:?}", str::from_utf8(&buffer).unwrap());
    //输出ASCII编码的数据
    //Response:[72, 101, 108, 108, 111]
    //println!("Response:{:?}", &buffer);
}
