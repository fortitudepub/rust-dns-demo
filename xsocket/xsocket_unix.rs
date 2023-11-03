use std::{future::Future, net::SocketAddr, pin::Pin, task::{self, Poll}};
use hyper::{service::Service, Uri};
use tokio::net::TcpStream;

#[derive(Clone)]
struct UnixConnector;

// TODO:
// 1. unix domain socket path.
impl Service<Uri> for UnixConnector {
    type Response = TcpStream;
    type Error = std::io::Error;
    // We can't "name" an `async` generated future.
    type Future = Pin<Box<
        dyn Future<Output = Result<Self::Response, Self::Error>> + Send
    >>;

    fn poll_ready(&mut self, _: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        // This connector is always ready, but others might not be.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Uri) -> Self::Future {
        Box::pin(TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], 1337))))
    }
}
