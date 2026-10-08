use std::net::UdpSocket;

const ADDR: &str = "127.0.0.1:2053";

#[derive(Debug)]
struct DnsHeader {
    id: u16,
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

fn parse_header(buf: &[u8]) -> DnsHeader {
    assert!(buf.len() >= 12);

    let id = u16::from_be_bytes([buf[0], buf[1]]);
    let flags_byte1 = buf[2];
    let flags_byte2 = buf[3];

    let qr = (flags_byte1 >> 7) & 0x01;
    let opcode = (flags_byte1 >> 3) & 0x0F;
    let aa = (flags_byte1 >> 2) & 0x01;
    let tc = (flags_byte1 >> 1) & 0x01;
    let rd = flags_byte1 & 0x01;

    let ra = (flags_byte2 >> 7) & 0x01;
    let rcode = flags_byte2 & 0x0F;

    let qdcount = u16::from_be_bytes([buf[4], buf[5]]);
    let ancount = u16::from_be_bytes([buf[6], buf[7]]);
    let nscount = u16::from_be_bytes([buf[8], buf[9]]);
    let arcount = u16::from_be_bytes([buf[10], buf[11]]);

    DnsHeader {
        id,
        qr,
        opcode,
        aa,
        tc,
        rd,
        ra,
        rcode,
        qdcount,
        ancount,
        nscount,
        arcount,
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(ADDR)?;
    println!("listening on {ADDR}");

    let mut buf = [0u8; 512];

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("\n{len} bytes from {src}:");
        hexdump(&buf[..len]);

        let header = parse_header(&buf[..len]);
        println!("{:#?}", header);
    }
}

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
