use core::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use piping_server::piping_server::PipingServer;
use piping_server::req_res_handler::req_res_handler;
use piping_server::util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let http_port: u16 = 8080;
    let piping_server = &PipingServer::new();

    // Set default log level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let http_svc = make_service_fn(|_| {
        let piping_server = piping_server.clone();
        let handler =
            req_res_handler(move |req, res_sender| piping_server.handler(req, res_sender));
        async { Ok::<_, Infallible>(service_fn(handler)) }
    });
    let http_server = Server::bind(&([0, 0, 0, 0], http_port).into()).serve(http_svc);

    log::info!("HTTP server is running on {}...", http_port);
    match http_server.await {
        Err(e) => return Err(util::make_io_error(format!("HTTP server error: {}", e))),
        _ => (),
    }
    Ok(())
}
