#![cfg(not(target_family = "wasm"))]
//! WebTransport client implementation.
use super::MTU;
use crate::transport::{PacketReceiver, PacketSender, Transport};
use bevy::tasks::{IoTaskPool, TaskPool};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tracing::{debug, error, info, trace};

use wtransport;
use wtransport::datagram::Datagram;
use wtransport::ClientConfig;

/// WebTransport client socket
pub struct WebTransportClientSocket {
    client_addr: SocketAddr,
    server_addr: SocketAddr,
}

impl WebTransportClientSocket {
    pub fn new(client_addr: SocketAddr, server_addr: SocketAddr) -> Self {
        Self {
            client_addr,
            server_addr,
        }
    }
}

impl Transport for WebTransportClientSocket {
    fn local_addr(&self) -> SocketAddr {
        self.client_addr
    }

    fn listen(self) -> (Box<dyn PacketSender>, Box<dyn PacketReceiver>) {
        let client_addr = self.client_addr;
        let server_addr = self.server_addr;
        let (to_server_sender, mut to_server_receiver) = mpsc::unbounded_channel();
        let (from_server_sender, from_server_receiver) = mpsc::unbounded_channel();

        let config = ClientConfig::builder()
            .with_bind_address(client_addr)
            .with_no_cert_validation()
            .build();
        let server_url = format!("https://{}", server_addr);
        debug!(
            "Starting client webtransport task with server url: {}",
            &server_url
        );
        let endpoint = wtransport::Endpoint::client(config).unwrap();

        let (send, recv) = tokio::sync::oneshot::channel();
        let (send2, recv2) = tokio::sync::oneshot::channel();
        // native wtransport must run in a tokio runtime
        tokio::spawn(async move {
            let connection = endpoint
                .connect(&server_url)
                .await
                .map_err(|e| {
                    error!("failed to connect to server: {:?}", e);
                })
                .unwrap();
            let connection = Arc::new(connection);
            send.send(connection.clone()).unwrap();
            send2.send(connection.clone()).unwrap();
        });
        // NOTE (IMPORTANT!):
        // - we spawn two different futures for receive and send datagrams
        // - if we spawned only one future and used tokio::select!(), the branch that is not selected would be cancelled
        // - this means that we might recreate a new future in `connection.receive_datagram()` instead of just continuing
        //   to poll the existing one. This is FAULTY behaviour
        // - if you want to use tokio::Select, you have to first pin the Future, and then select on &mut Future. Only the reference gets
        //   cancelled
        tokio::spawn(async move {
            let connection = recv.await.expect("could not get connection");
            loop {
                match connection.receive_datagram().await {
                    Ok(data) => {
                        trace!("receive datagram from server: {:?}", &data);
                        from_server_sender.send(data).unwrap();
                    }
                    Err(e) => {
                        error!("receive_datagram connection error: {:?}", e);
                    }
                }
            }
        });
        tokio::spawn(async move {
            let connection = recv2.await.expect("could not get connection");
            loop {
                if let Some(msg) = to_server_receiver.recv().await {
                    trace!("send datagram to server: {:?}", &msg);
                    connection.send_datagram(msg).unwrap_or_else(|e| {
                        error!("send_datagram error: {:?}", e);
                    });
                }
            }
        });
        let packet_sender = WebTransportClientPacketSender { to_server_sender };
        let packet_receiver = WebTransportClientPacketReceiver {
            server_addr,
            from_server_receiver,
            buffer: [0; MTU],
        };
        (Box::new(packet_sender), Box::new(packet_receiver))
    }
}

struct WebTransportClientPacketSender {
    to_server_sender: mpsc::UnboundedSender<Box<[u8]>>,
}

impl PacketSender for WebTransportClientPacketSender {
    fn send(&mut self, payload: &[u8], address: &SocketAddr) -> std::io::Result<()> {
        let data = payload.to_vec().into_boxed_slice();
        self.to_server_sender
            .send(data)
            .map_err(|e| std::io::Error::other(format!("send_datagram error: {:?}", e)))
    }
}

struct WebTransportClientPacketReceiver {
    server_addr: SocketAddr,
    from_server_receiver: mpsc::UnboundedReceiver<Datagram>,
    buffer: [u8; MTU],
}

impl PacketReceiver for WebTransportClientPacketReceiver {
    fn recv(&mut self) -> std::io::Result<Option<(&mut [u8], SocketAddr)>> {
        match self.from_server_receiver.try_recv() {
            Ok(data) => {
                // convert from datagram to payload via xwt
                self.buffer[..data.len()].copy_from_slice(data.payload().as_ref());
                Ok(Some((&mut self.buffer[..data.len()], self.server_addr)))
            }
            Err(e) => {
                if e == TryRecvError::Empty {
                    Ok(None)
                } else {
                    Err(std::io::Error::other(format!(
                        "receive_datagram error: {:?}",
                        e
                    )))
                }
            }
        }
    }
}
