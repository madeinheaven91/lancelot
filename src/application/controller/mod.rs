pub mod logging;
mod router;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tower::ServiceBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::Logger;
use crate::application::controller::router::*;


pub async fn init_server() -> Result<(), Box<dyn std::error::Error>> {
    // let port = dotenv!("PORT").parse::<u32>().unwrap_or(3000);
    
    let port = match std::env::var("PORT") {
        Ok(v) => v.parse::<u16>().unwrap_or(3000),
        Err(_) => 3000
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening at {}", addr.to_string());
    // TODO: log webdriver port
    
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move {
            let svc = hyper::service::service_fn(router);
            let svc = ServiceBuilder::new().layer_fn(Logger::new).service(svc);
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                error!("Server error: {}", err);
            }
        });
    }
}
