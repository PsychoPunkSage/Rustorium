// WebServices + Networking
use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

extern crate rusqlite;
use rusqlite::{params, Connection};
//  Random Generator.
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
// To work with db
use std::sync::Mutex; // allows only 1 thread (at a time) to access db

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FormData {
    content: String,
}

struct AppState {
    db: Mutex<Connection>,
}

///////////
// INDEX //
///////////
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"))
    /* <<.body()>> This sets the response body to the contents of the file "index.html" using the include_str! macro */
    /* <<include_str!>> reads the entire contents of the specified file and includes it directly into the code as a string literal */

}


////////////
// SUBMIT //
////////////
async fn submit(content: web::Form<FormData>, data: web::Data<AppState>) -> impl Responder {
    let token: String = thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    
    // Open db connection
    let conn = data.db.lock().unwrap();
    conn.execute(
        "INSERT INTO pstes (token, content) VALUES (?, ?)",
        params![&token, &content.content],
    )
    .expect("Failed to insert to DB");

    HttpResponse::SeeOther().append_header(("Location", format!("paste/{}", token))).finish()
}

///////////////
// GET_PASTE //
///////////////
async fn get_paste(token: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let content = conn
    .query_row(
        "SELECT content FROM pastes WHERE token = ?", 
        params![token.to_string()], 
        |row| row.get::<_, String>(0),
    )
        .unwrap_or_else(|_| "Paste not found".to_string());

    HttpResponse::Ok().body(format!("<pre>{}</pre>", content))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish Connection with DB...
    let db = Connection::open("pastebin.db").expect("Failed to open pastebin database");

    // SQL command to create table if it doesn't EXISTS
    db.execute(
        "CREATE TABLE IF NOT EXISTS pastes (token TEXT PRIMARY KEY, content TEXT)",
        params![],
    )
    .expect("Failed to create a table");

    let app_state = web::Data::new(AppState { db: Mutex::new(db) });

    // Creating HttpServer
    HttpServer::new(move || { /* <<HttpServer>> responsible for handling incoming HTTP requests and routing them to the appropriate application code. */
                              /* <<move>> keyword tells the compiler to capture the values of any variables referenced inside the closure */
        /* <<App>> type defines the structure of the application and contains information about the routes, middleware, and application state. */
        App::new() 
            .app_data(app_state.clone())
            .service(web::resource("/style.css").to(|| {
                async { NamedFile::open("src/style.css") }
            }))
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .route("/paste/{token}", web::get().to(get_paste))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

