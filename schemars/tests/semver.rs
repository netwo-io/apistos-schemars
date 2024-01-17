mod util;
extern crate apistos_schemars as schemars;
use schemars::JsonSchema;
use semver::Version;
use util::*;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct SemverTypes {
    version: Version,
}

#[test]
fn semver_types() -> TestResult {
    test_default_generated_schema::<SemverTypes>("semver")
}
