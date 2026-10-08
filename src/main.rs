use std::net::UdpSocket;

// Port 53 needs root, so we use 2053 while developing.
// Test with:  dig @127.0.0.1 -p 2053 google.com
const ADDR: &str = "127.0.0.1:2053";

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(ADDR)?;
    println!("listening on {ADDR}");

    // A classic DNS message over UDP is at most 512 bytes.
    let mut buf = [0u8; 512];

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("\n{len} bytes from {src}:");
        hexdump(&buf[..len]);

        // TODO (stage 1): parse the 12-byte header from `buf` and print its fields.
    }
}

/// Prints bytes 16 per line: offset, hex, then printable ASCII.
fn hexdump(bytes: &[u8]) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02x}")).collect();
        let ascii: String = chunk
            .iter()
            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
            .collect();
        println!("{:04x}  {:<48} {}", i * 16, hex.join(" "), ascii);
    }
}
