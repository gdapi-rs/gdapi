use gdapi::client::Client;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut client = Client::new();

	let username = env::var("GD_USERNAME")?;
	let password = env::var("GD_PASSWORD")?;

	let user = client.login(&username, &password).await?;
	println!("Logged in to account: {:?}", user);

	let comment_id = client.upload_account_comment("Uploaded with gdapi-rs!").await?;
	println!("Created comment with id: {}", comment_id);

	Ok(())
}