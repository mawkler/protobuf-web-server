use big_data;
use protobuf::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = reqwest::get("http://localhost:3000/proto")
        .await?
        .bytes()
        .await?;

    println!("Bytes: {:?}", bytes.len());

    let big_data = deserialize_bigdata(&bytes)?;
    Ok(())
}

fn deserialize_bigdata(
    buf: &[u8],
) -> Result<big_data::big_data::BigDataObjectArray, protobuf::Error> {
    big_data::big_data::BigDataObjectArray::parse_from_bytes(buf)
}
