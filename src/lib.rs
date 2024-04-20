#![allow(non_snake_case)]
use std::ptr;
use std::net::{SocketAddr, UdpSocket};
use std::sync::OnceLock;
use std::convert::TryFrom;

struct RGB {
    red: u8,
    green: u8,
    blue: u8
}

impl IntoIterator for RGB {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.red, self.green, self.blue])
    }
}
static SOCKET: OnceLock<UdpSocket> = OnceLock::new();

fn flatten<T, const N: usize>(v: Vec<Vec<[T; N]>>) -> Vec<T> {
    v.into_iter().flatten().flatten().collect()
}

#[no_mangle]
pub extern fn USBIntLED_getVersion() -> i64 {
    println!("Get Version");
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    257
}

#[no_mangle]
pub extern fn USBIntLED_Init() -> bool {
    println!("Init");
    let _ = SOCKET.set(UdpSocket::bind("0.0.0.0:0").unwrap());
    return true;
}

#[no_mangle]
pub extern fn USBIntLED_Terminate() -> bool {
    println!("Terminate");
    return true
}

#[no_mangle]
pub extern fn USBIntLED_set(_a1: i64, a2: usize) {
    let mut header: Vec<u8> = vec![2, 2];
    let mut leds: Vec<RGB> = Vec::new();
    for i in (3..1920).step_by(4) {
        let n: u32 = unsafe { ptr::read((a2+i) as *const u32) };
        let a: [u8; 4] =n.to_le_bytes();
        leds.push(RGB {
            red: a[1],
            green: a[2],
            blue: a[3],
        });
    }
    
    let addr = SocketAddr::from(([100, 64, 0, 79], 21324));
    let mut flattened: Vec<u8> = leds.into_iter().flatten().collect();
    header.append(&mut flattened);
    match SOCKET.get() {
        Some(socket) => {
            let sock: UdpSocket = socket.try_clone().unwrap();
            let _ = sock.send_to(&header, addr);
        },
        None => {
            println!("Socket hasn't been initialized yet")
        },
    }
}
