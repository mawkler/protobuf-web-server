use axum::{routing::get, Router};
use big_data::big_data::BigDataObjectArray;
use protobuf_json_mapping::ParseOptions;
use serde_json::Value;
use std::{fs::read_to_string, fs::File, io::BufReader, net::SocketAddr};

async fn open_and_parse_json() -> &'static str {
    let path = "large-file.json";
    // let file = File::open(path).expect("File not found");
    // let reader = BufReader::new(file);
    let file_string = read_to_string(path).expect("File not found");

    let foo: BigDataObjectArray = protobuf_json_mapping::parse_from_str_with_options(
        &file_string,
        &ParseOptions {
            ignore_unknown_fields: true,
            ..Default::default()
        },
    )
    .expect("Failed to parse JSON");

    // Read the JSON contents of the file as an instance of `User`.
    // let u: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    println!("u = {:#?}", foo);
    "foo"
}

#[tokio::main]
async fn main() {
    println!("{}", std::env::current_dir().unwrap().display());

    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    // let app = Router::new().route("/", get(handler));
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                println!("Beep boop");
                "Hello, world!"
            }),
        )
        .route("/proto", get(open_and_parse_json));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}
