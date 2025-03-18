use crate::JsonSchema;
use crate::generator::SchemaGenerator;
use crate::schema::*;
use bytes::{Bytes, BytesMut};

forward_impl!((JsonSchema for Bytes) => Vec<u8>);
forward_impl!((JsonSchema for BytesMut) => Vec<u8>);
