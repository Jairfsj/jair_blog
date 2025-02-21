use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, middleware};
pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
	let srv = HttpServer::new(move || {
	    App::new()
		   .wrap(middleware::Logger::default()) // enable logger
		   .route("/health", web::get().to(HttpResponse::Ok))
	})
	.listen(listener)?
	.run();
	
	Ok(srv)
}
