//use std::net::TcpListener;
//use jair_blog::start;

//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//	std::env::set_var("RUST_LOG", "actix_web=info");
//	env_logger::init();
//	
//	let listener = TcpListener::bind("0.0.0.0:8080")?;
//	start_blog(listener)?.await?;
//	Ok(())
//}

//use std::net::TcpListener;
//use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
//use tera::Tera;
//pub mod handlers; // new line
//#[macro_use]
//extern crate lazy_static;
//lazy_static! {
//	pub static ref TEMPLATES: Tera = {
//	    let mut tera = match Tera::new("templates/**/*.html") {
//			Ok(t) => t,
//			Err(e) => {
//				println!("Parsing error(s): {}", e);
//				::std::process::exit(1);
//			}
//		};
//		tera.autoescape_on(vec![".html", ".sql"]);
//		tera
//	};
//}
//pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
//	let server = HttpServer::new(move || {
//	    App::new()
//		   .app_data(web::Data::new(TEMPLATES.clone()))
//		   .wrap(middleware::Logger::default()) // enable logger
//		   .route("/health", web::get().to(HttpResponse::Ok))
//            .service(handlers::index) // new line
//	})
//	.listen(listener)?
//	.run();
//	
//	Ok(server)
//}

use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder, middleware};
use tera::Tera;

mod handlers;
//pub mod handlers; // new line

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
            .service(handlers::home_handler::index) // new line
    })
    .listen(listener)?
    .run();
    
    Ok(server)
}

#[actix_web::main] // Transforma a main em assíncrona
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("🚀 Server running at http://127.0.0.1:8080/");
    start_blog(listener)?.await
}
