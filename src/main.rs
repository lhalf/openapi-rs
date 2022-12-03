use serde_json;
use openapiv3::OpenAPI;

fn main() {
    let openapi_json = include_str!("../examples/xkcd.json");
    let openapi: OpenAPI = serde_json::from_str(openapi_json)
        .expect("Could not deserialise input");
    for path in openapi.paths {
        println!("{:?}", path.0);
        println!("---------------")
    }
}