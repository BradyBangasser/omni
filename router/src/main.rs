mod executer;
mod handle;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use hyper_util::server::conn::auto::Builder as AutoBuilder;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::TcpListener;

use hyper_util::rt::TokioIo;

use crate::executer::LocalExecuter;
use crate::handle::handle_request;

fn main() {
    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let local = tokio::task::LocalSet::new();

    local.block_on(&rt, async {
        let tcp_sock = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();

        tcp_sock.set_reuse_port(true).unwrap();
        tcp_sock.bind(&addr.into()).unwrap();
        tcp_sock.listen(1024).unwrap();
        tcp_sock.set_nonblocking(true).unwrap();

        let tcp_listener = TcpListener::from_std(tcp_sock.into()).unwrap();

        loop {
            tokio::select! {
                tcp_result = tcp_listener.accept() => {
                    if let Ok((tcp_stream, _client_addr)) = tcp_result {
                        tokio::task::spawn_local(async move {
                            let io = TokioIo::new(tcp_stream);

                            if let Err(err) = AutoBuilder::new(LocalExecuter).serve_connection(io, hyper::service::service_fn(handle_request)).await {
                                eprintln!("Connection error: {:?}", err);
                            }
                        });
                    }
                }
            }
        }
    })
}
