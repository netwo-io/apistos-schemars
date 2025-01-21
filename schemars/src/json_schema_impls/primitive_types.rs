use primitive_types::H128;
use primitive_types::H160;
use primitive_types::H256;
use primitive_types::H384;
use primitive_types::H512;
use primitive_types::H768;
use primitive_types::U128;
use primitive_types::U256;
use primitive_types::U512;

use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use std::borrow::Cow;

macro_rules! schema_uint {
    ($t:ty) => {
        impl JsonSchema for $t {
            no_ref_schema!();
        
            fn schema_name() -> String {
                stringify!($t).to_string()
            }
        
            fn schema_id() -> Cow<'static, str> {
                Cow::Borrowed(std::any::type_name::<$t>())
            }
        
            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

macro_rules! schema_fixed_hash {
    ($t:ty,$len_in_chars:literal) => {
        impl JsonSchema for $t {
            no_ref_schema!();

            fn schema_name() -> String {
                stringify!($t).to_string()
            }

            fn schema_id() -> Cow<'static, str> {
                Cow::Borrowed(std::any::type_name::<$t>())
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    string: Some(Box::new(StringValidation {
                        pattern: Some(concat!("0x\\w{", stringify!($len_in_chars),"}").to_string()),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

schema_uint!(U128);
schema_uint!(U256);
schema_uint!(U512);

schema_fixed_hash!(H128, 32);
schema_fixed_hash!(H160, 40);
schema_fixed_hash!(H256, 64);
schema_fixed_hash!(H384, 96);
schema_fixed_hash!(H512, 128);
schema_fixed_hash!(H768, 192);



