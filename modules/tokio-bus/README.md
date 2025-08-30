# tokio-bus

Tokio-based implementation of the ASN event bus.

This crate provides an implementation of the `AsnBus` trait using Tokio's broadcast channels. It allows for asynchronous message passing between different parts of an application.

## Overview

The `tokio-bus` module implements the event bus pattern using Tokio's broadcast channels. It allows multiple receivers to receive the same messages, making it suitable for scenarios where you need to distribute events to multiple consumers.

## Features

- Asynchronous message passing
- Multiple receiver support
- Integration with the ASN bus traits
- Non-blocking send and receive operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-bus = { path = "./modules/tokio-bus" }
```

## Usage

Basic usage:

```rust
use tokio_bus::new_tokio_bus;
use asn_core_bus::{AsnBus, AsnTransmitter, AsnReceiver};

#[derive(Clone, Debug)]
enum Message {
    Update(String),
    Shutdown,
}

let bus = new_tokio_bus::<Message>(16);
let sender = bus.get_sender();
let mut receiver = bus.get_receiver();

sender.send_message(Message::Update("Hello".to_string())).unwrap();
let msg = receiver.get_message().unwrap();
```

## API Documentation

For detailed API documentation, see the [tokio_event_bus](src/tokio_event_bus.rs) module.

## Examples

You can find more examples in the [examples](../../examples) directory, specifically the [ex_bus.rs](../../examples/ex_bus.rs) file.

## Error Handling

The module handles various error conditions:

- `AsnBusSendError::Closed`: When trying to send a message to a closed channel
- `AsnBusRecvError::Empty`: When trying to receive a message from an empty channel
- `AsnBusRecvError::Closed`: When trying to receive a message from a closed channel
- `AsnBusRecvError::Lagged(n)`: When the receiver lagged too far behind and missed `n` messages

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.