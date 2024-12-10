use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};


// A fallback route for if the route is not matching
async fn dynamic_index() -> impl Responder {
    HttpResponse::Ok().body("<h1>404 - Page Not Found </h1>")}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(

            //Web scope allow you to decalare a parent route in this case we decalred a a parent route name app and within this route we have a child route named passed to as services argument, here we are passing a static html file that will be served by the server on the route /app/index.html or on the root route /app and also we can have a dynamic fallback route such as /* which will server the dynamic_index function if there is no matching routes
            web::scope("/app")
                .service(
                    fs::Files::new("/", "./static").index_file("index.html"),
                ).default_service(web::get().to(dynamic_index))
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
