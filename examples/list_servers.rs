//! Lists the cloud servers on the account.
//!
//! Run with a Timeweb Cloud JWT token in the environment:
//!
//! ```sh
//! TIMEWEB_CLOUD_TOKEN=your-token cargo run --example list_servers
//! ```

use timeweb_rs::apis::servers_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TIMEWEB_CLOUD_TOKEN")
        .expect("set the TIMEWEB_CLOUD_TOKEN environment variable");

    let config = timeweb_rs::authenticated(token);
    let response = servers_api::get_servers(&config, None, None).await?;

    println!("{response:#?}");

    Ok(())
}
