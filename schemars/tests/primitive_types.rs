mod util;
use util::*;

#[test]
fn u256() -> TestResult {
    test_default_generated_schema::<primitive_types::U256>("u256")
}

#[test]
fn h160() -> TestResult {
    test_default_generated_schema::<primitive_types::H160>("h160")
}
