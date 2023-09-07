use std::io::Cursor;

use protobuf::big_data;
use prost::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = reqwest::get("http://localhost:3000/proto")
        .await?
        .bytes()
        .await?;

    println!("Bytes: {:?}", bytes.len());

    let big_data = deserialize_bigdata(&bytes)?;
    // println!("{:#?}", big_data.data[0]);
    Ok(())    
}

fn deserialize_bigdata(buf: &[u8]) -> Result<big_data::BigDataObjectArray, prost::DecodeError> {
    big_data::BigDataObjectArray::decode(&mut Cursor::new(buf))
}