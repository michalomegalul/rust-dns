# rusty-dns

A DNS server written from scratch in Rust, as a learning project. No DNS crates: every byte is parsed by hand.

## Running

```sh
cargo run
# in another terminal
dig @127.0.0.1 -p 2053 google.com
```

## Roadmap

- [x] **Stage 0: Listen.** Bind a UDP socket and hex-dump incoming queries.
- [ ] **Stage 1: Header.** Parse the 12-byte header (ID, flags, section counts).
- [ ] **Stage 2: Question.** Parse domain names (length-prefixed labels), QTYPE and QCLASS.
- [ ] **Stage 3: First answer.** Build a response by hand and make `dig` accept it (answer every A query with `1.2.3.4`).
- [ ] **Stage 4: Forwarder.** Pass queries to an upstream resolver (e.g. `1.1.1.1`) and relay the answer back.
  - Handle **name compression** (pointers, `0xC0`) in responses.
- [ ] **Stage 5: Local zone.** Answer `*.home` names from a config file (`jellyfin.home -> 192.168.4.21`).
- [ ] **Stage 6: Cache.** Cache answers and respect each record's TTL.
- [ ] **Stage 7: Blocklist.** Block ad and tracker domains from a list, like Pi-hole does.
- [ ] **Stage 8: Recursive resolver.** Start at the root servers and follow referrals yourself, with no upstream.
- [ ] **Stage 9: Concurrency.** Move to `tokio` and handle many clients at once.
- [ ] **Stage 10: Deploy.** Run it in a Proxmox LXC as the homelab's DNS server, with metrics.

## References

- [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035), the original DNS spec. Section 4 covers the message format.
- [Wireshark](https://www.wireshark.org/): capture `dig` traffic to compare against your own output.
