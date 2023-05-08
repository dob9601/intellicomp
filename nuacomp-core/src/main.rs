use nuacomp_core::Command;
use schemars::schema_for;

pub fn main() {
    let schema = schema_for!(Command);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
