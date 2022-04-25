use gdapi::client::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let client = Client::new();

	let user = client.search_user("Vaporox").await?;
	println!("Found user: {:?}", user);

	let user = client.user(71).await?;
	println!("Found user: {:?}", user);

	Ok(())
}