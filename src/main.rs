use serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
struct Basic {
    doit: bool,
}

#[derive(Serialize)]
struct BasicWithNoJsonSchema {
    #[serde(with = "serde_with::rust::display_fromstr")]
    doit: bool,
}

#[derive(Serialize, JsonSchema)]
struct BasicWithJsonSchema {
    #[serde(with = "serde_with::rust::display_fromstr")]
    doit: bool,
}

fn main() {
    println!("Hello, world!");
}
