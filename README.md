lwip - A Rust wrapper for lwIP
==============================

[![Version](https://img.shields.io/crates/v/lwip.svg?style=flat)](https://crates.io/crates/lwip)
[![Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat)](https://docs.rs/lwip)
[![Download](https://img.shields.io/crates/d/lwip.svg)](https://crates.io/crates/lwip)

A netstack for the special purpose of turning packets from/to a TUN interface into TCP streams and UDP packets.
It uses `lwIP` as the backend netstack.

```rust, ignore
use futures::{SinkExt, StreamExt};

let (stack, mut tcp_listener, udp_socket) = ::lwip::NetStack::new().unwrap();
let (mut stack_sink, mut stack_stream) = stack.split();

// tun device is assumed implementing `Stream` and `Sink`
let (mut tun_sink, mut tun_stream) = tun.split();

// Reads packet from stack and sends to TUN.
tokio::spawn(async move {
    while let Some(pkt) = stack_stream.next().await {
        if let Ok(pkt) = pkt {
            tun_sink.send(pkt).await.unwrap();
        }
    }
});

// Reads packet from TUN and sends to stack.
tokio::spawn(async move {
    while let Some(pkt) = tun_stream.next().await {
        if let Ok(pkt) = pkt {
            stack_sink.send(pkt).await.unwrap();
        }
    }
});

// Extracts TCP connections from stack and sends them to the dispatcher.
tokio::spawn(async move {
    while let Some((stream, local_addr, remote_addr)) = tcp_listener.next().await {
        tokio::spawn(handle_inbound_stream(stream, local_addr, remote_addr));
    }
});

// Receive and send UDP packets between netstack and NAT manager. The NAT
// manager would maintain UDP sessions and send them to the dispatcher.
tokio::spawn(async move {
    handle_inbound_datagram(udp_socket).await;
});
```
