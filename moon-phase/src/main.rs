/* An actix microservice that has three routes
A. / that returns a hello page with moon image
B. /moon that returns the current moon phase
C. /moon?date=YYYY-MM-DD that returns the moon phase for the given date
*/

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

use moon_phase::get_moon_phase;

// create a function that returns a moon image
#[get("/")]
async fn moon_image() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(include_bytes!("../moon.jpg").to_vec())
}

// create a function that returns the current moon phase
#[get("/moon")]
async fn moon_phase_cur() -> impl Responder {
    let moon_phase = get_moon_phase(chrono::Utc::now().date_naive())
        .await
        .unwrap();
    // print moon phase
    println!("Moon Phase of today: {moon_phase}");
    HttpResponse::Ok().body(moon_phase)
}

// create datetime object from string
fn parse_date(date: String) -> Result<chrono::NaiveDate, chrono::ParseError> {
    // parse date string
    let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
    Ok(date)
}

// create a function that returns the moon phase for the given date
#[get("/moon/{date}")]
async fn moon_phase_for_date(date: web::Path<String>) -> impl Responder {
    // handle parse error
    let date = match parse_date(date.into_inner()) {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().body("Invalid date"),
    };
    let moon_phase = get_moon_phase(date).await.unwrap();
    // print moon phase
    println!("Moon Phase: {moon_phase}" );
    HttpResponse::Ok().body(moon_phase)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add a print message to the console to indicate that the server is running
    println!("Server running at http://localhost:8080");
    // start http server
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(moon_image)
            .service(moon_phase_cur)
            .service(moon_phase_for_date)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
