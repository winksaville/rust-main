use std::io::{Write, stdin, stdout};

fn main() -> ! {
    let argc = std::env::args().count();
    println!("argc = {}", argc);
    let mut buf = String::new();
    loop {
        stdin().read_line(&mut buf).unwrap();
        stdout().write_all(buf.as_bytes()).unwrap();
    }
}
