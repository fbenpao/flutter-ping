use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use std::os::raw::c_char;
use std::ffi::CString;

use socket2::{Domain, Socket, Type};

pub fn ping(port:i64)->Result<i64,*mut c_char> {
    let addr: SocketAddr = format!("{}:{}", "google.com", port)
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
               return Ok(elapsed.as_millis())
            }
            Err(e) => {
                println!("Connect to {} failed: {}", &addr, e);
          return  Err(CString::new("timeout").unwrap().into_raw())
            }
        }
        // std::thread::sleep(std::time::Duration::from_secs(1));
}
