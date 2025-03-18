use crate::JsonSchema;
use crate::generator::SchemaGenerator;
use crate::schema::{InstanceType, Schema, SchemaObject, SubschemaValidation};
use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

impl JsonSchema for IpNetwork {
    no_ref_schema!();

    fn schema_name() -> String {
        "Ip".to_string()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    Ipv4Network::json_schema(generator),
                    Ipv6Network::json_schema(generator),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl JsonSchema for Ipv4Network {
    no_ref_schema!();

    fn schema_name() -> String {
        "IpV4".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("ipv4".to_string()),
            ..Default::default()
        }
        .into()
    }
}

impl JsonSchema for Ipv6Network {
    no_ref_schema!();

    fn schema_name() -> String {
        "IpV6".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("ipv6".to_string()),
            ..Default::default()
        }
        .into()
    }
}
