#![doc = include_str!("../README.md")]

mod lwip;
mod mutex;
mod output;
mod stack;
mod stack_impl;
mod tcp_listener;
mod tcp_listener_impl;
mod tcp_stream;
mod tcp_stream_context;
mod tcp_stream_impl;
mod udp;
mod util;

pub(crate) static LWIP_MUTEX: mutex::AtomicMutex = mutex::AtomicMutex::new();
pub(crate) use mutex::AtomicMutexGuard as LWIPMutexGuard;

pub use stack::NetStack;
pub use tcp_listener::TcpListener;
pub use tcp_stream::TcpStream;
pub use udp::UdpSocket;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("LwIP error ({0})")]
    LwIP(i8),

    #[error("AtomicMutexErr {0:?}")]
    AtomicMutexErr(#[from] mutex::AtomicMutexErr),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
