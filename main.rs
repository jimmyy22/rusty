use actix_web::{get, App, HttpResponse, HttpServer, Responder};

/// Handler for the index route
#[get("/")]
async fn index() -> impl Responder {
    // Return an HTML page with inline CSS styling
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>My Rust Web App</title>
  <style>
    body {
      background-color: #f0f0f0;
      font-family: Arial, sans-serif;
      margin: 0;
      padding: 20px;
    }
    .container {
      max-width: 800px;
      margin: 40px auto;
      background-color: #fff;
      padding: 30px;
      border-radius: 8px;
      box-shadow: 0 0 10px rgba(0,0,0,0.1);
    }
    h1 {
      color: #333;
    }
    p {
      font-size: 18px;
      line-height: 1.6;
      color: #555;
    }
    a {
      text-decoration: none;
      color: #007BFF;
    }
    a:hover {
      text-decoration: underline;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Welcome to My Rust Web App</h1>
    <p>This is a simple website built using Rust and Actix-web. Enjoy exploring Rust's power in web development!</p>
    <p>Learn more about <a href="https://www.rust-lang.org/" target="_blank">Rust</a> and <a href="https://actix.rs/" target="_blank">Actix-web</a>.</p>
  </div>
</body>
</html>
    "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define the server address
    let address = "127.0.0.1:8080";
    println!("Starting server at http://{address}");

    // Create and run the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(index) // Register the index route
    })
    .bind(address)?
    .run()
    .await
}
