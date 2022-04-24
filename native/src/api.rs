use std::ffi::CString;
use std::net::{SocketAddr, ToSocketAddrs};
use std::os::raw::c_char;
use std::time::Duration;

use socket2::{Domain, Socket, Type};

pub fn ping(host: String, port: i64) -> String {
    let addr: SocketAddr = format!("{}:{}", &host, port)
        .to_socket_addrs()
        .unwrap()
        .filter(|a| a.is_ipv4())
        .next()
        .unwrap();
    let timeout: Duration = Duration::from_secs(4);
    let start = std::time::Instant::now();
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let res = socket.connect_timeout(&addr.into(), timeout);
    let elapsed = std::time::Instant::now().duration_since(start);
    match res {
        Ok(_) => {
            println!("Connected to {} in {} ms", &addr, elapsed.as_millis());
            return elapsed.as_millis().to_string();
        }
        Err(e) => {
            println!("Connect to {} failed: {}", &addr, e);
            return "time out".to_string();
        }
    }
    // std::thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test() {
    println!("ping result:{}", ping("google.com".to_string(), 443));
}
