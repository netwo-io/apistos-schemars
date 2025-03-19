use crate::JsonSchema;
use crate::generator::SchemaGenerator;
use crate::schema::*;
use enumset::{EnumSet, EnumSetType};

forward_impl!((<T> JsonSchema for EnumSet<T> where T: EnumSetType + JsonSchema) => std::collections::BTreeSet<T>);
