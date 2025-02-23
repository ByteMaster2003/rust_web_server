use mongodb::Client;
use std::io;

pub async fn db_connect(uri: &str) -> io::Result<Client> {
  match Client::with_uri_str(uri).await {
    Ok(client) => {
      // Ping the server to check if connection is valid
      match client.list_database_names().await {
        Ok(_) => {
          println!("MongoDB connection established successfully");
          Ok(client)
        }
        Err(e) => {
          eprintln!("Failed to ping MongoDB: {}", e);
          Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to ping MongoDB",
          ))
        }
      }
    }
    Err(e) => {
      eprintln!("Failed to connect to MongoDB: {}", e);
      Err(io::Error::new(
        io::ErrorKind::Other,
        "Database connection failed",
      ))
    }
  }
}
