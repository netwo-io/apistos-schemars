extern crate apistos_schemars as schemars;
use schemars::schema_for;

fn main() {
    let _schema = schema_for!(123);
}
