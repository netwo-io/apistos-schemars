use crate::gen::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use std::borrow::Cow;

macro_rules! tuple_impls {
    ($($len:expr => ($($name:ident)+))+) => {
        $(
            impl<$($name: JsonSchema),+> JsonSchema for ($($name,)+) {
                no_ref_schema!();

                fn schema_name() -> String {
                    let mut name = "Tuple_of_".to_owned();
                    name.push_str(&[$($name::schema_name()),+].join("_and_"));
                    name
                }

                fn schema_id() -> Cow<'static, str> {
                    let mut id = "(".to_owned();
                    id.push_str(&[$($name::schema_id()),+].join(","));
                    id.push(')');

                    Cow::Owned(id)
                }

                fn json_schema(gen: &mut SchemaGenerator) -> Schema {
                    json_schema!({
                        "type": "array",
                        "prefixItems": [
                            $(gen.subschema_for::<$name>()),+
                        ],
                        "minItems": $len,
                        "maxItems": $len,
                    })
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (T0)
    2 => (T0 T1)
    3 => (T0 T1 T2)
    4 => (T0 T1 T2 T3)
    5 => (T0 T1 T2 T3 T4)
    6 => (T0 T1 T2 T3 T4 T5)
    7 => (T0 T1 T2 T3 T4 T5 T6)
    8 => (T0 T1 T2 T3 T4 T5 T6 T7)
    9 => (T0 T1 T2 T3 T4 T5 T6 T7 T8)
    10 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9)
    11 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10)
    12 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11)
    13 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12)
    14 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13)
    15 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14)
    16 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15)
}
