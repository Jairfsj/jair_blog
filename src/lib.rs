//use std::net::TcpListener;
//use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
//use tera::Tera;
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
//	let srv = HttpServer::new(move || {
//	    App::new()
//		   .app_data(web::Data::new(TEMPLATES.clone()))
//		   .wrap(middleware::Logger::default()) // enable logger
//		   .route("/health", web::get().to(HttpResponse::Ok))
//	})
//	.listen(listener)?
//	.run();
//	
//	Ok(srv)

//use std::net::TcpListener;
//use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder, middleware};
//use tera::Tera;

//pub mod handlers; // new line

//#[macro_use]
//extern crate lazy_static;

//lazy_static! {
//    pub static ref TEMPLATES: Tera = {
//        let mut tera = match Tera::new("templates/**/*.html") {
//            Ok(t) => t,
//            Err(e) => {
//                println!("Parsing error(s): {}", e);
//                ::std::process::exit(1);
//            }
//        };
//        tera.autoescape_on(vec![".html", ".sql"]);
//        tera
//    };
//}

//pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
//    let server = HttpServer::new(move || {
//        App::new()
//            .app_data(web::Data::new(TEMPLATES.clone()))
//            .wrap(middleware::Logger::default()) // enable logger
//            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
//            .service(handlers::index) // new line
//    })
//    .listen(listener)?
//    .run();
    
//    Ok(server)
//}


use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, middleware};
use tera::Tera;

pub mod handlers; // Importa o módulo handlers (que é o mod.rs)
pub use handlers::home_handler::index; // Reexporta o módulo home_handler

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
            .service(handlers::home_handler::index) // Usa a função index do módulo home_handler
    })
    .listen(listener)?
    .run();
    
    Ok(server)
}
