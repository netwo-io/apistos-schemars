mod util;
use bytes1::{Bytes, BytesMut};
use util::*;

#[test]
fn bytes() -> TestResult {
    test_default_generated_schema::<(Bytes, BytesMut)>("bytes")
}
