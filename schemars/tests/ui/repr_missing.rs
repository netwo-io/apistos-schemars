extern crate apistos_schemars as schemars;
use schemars::JsonSchema_repr;

#[derive(JsonSchema_repr)]
pub enum Enum {
    Unit,
}

fn main() {}
