#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:3000")
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}