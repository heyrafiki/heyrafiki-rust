//! Conformance tests against the pinned public `OpenAPI` contract.

use std::{collections::BTreeSet, fs};

use heyrafiki::SUPPORTED_OPERATIONS;
use serde_yaml_ng::{Mapping, Value};
use sha2::{Digest, Sha256};

const CONTRACT_SHA256: &str = "d1c7349246e766aaf961e11c591a32de0afcc5900649be462e3656059722b211";

#[test]
fn pinned_contract_has_expected_digest() {
    let bytes = fs::read("openapi/openapi.yaml").expect("pinned OpenAPI contract");
    let digest = format!("{:x}", Sha256::digest(bytes));
    assert_eq!(digest, CONTRACT_SHA256);
}

#[test]
fn sdk_operation_matrix_matches_every_published_operation() {
    let source = fs::read_to_string("openapi/openapi.yaml").expect("pinned OpenAPI contract");
    let document: Value = serde_yaml_ng::from_str(&source).expect("valid OpenAPI YAML");
    let paths = mapping_field(document.as_mapping().expect("OpenAPI object"), "paths");

    let mut published = BTreeSet::new();
    for (path, item) in paths {
        let path = path.as_str().expect("path string");
        let item = item.as_mapping().expect("path item");
        for method in ["get", "post", "delete"] {
            let Some(operation) = item.get(Value::String(method.into())) else {
                continue;
            };
            let operation = operation.as_mapping().expect("operation object");
            let operation_id = operation
                .get(Value::String("operationId".into()))
                .and_then(Value::as_str)
                .expect("operationId");
            published.insert((
                operation_id.to_owned(),
                method.to_uppercase(),
                path.to_owned(),
            ));
        }
    }

    let implemented = SUPPORTED_OPERATIONS
        .iter()
        .map(|operation| {
            (
                operation.operation_id.to_owned(),
                operation.method.to_owned(),
                operation.path.to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(implemented, published);
    assert_eq!(implemented.len(), 30);
}

fn mapping_field<'a>(mapping: &'a Mapping, field: &str) -> &'a Mapping {
    mapping
        .get(Value::String(field.into()))
        .and_then(Value::as_mapping)
        .unwrap_or_else(|| panic!("missing {field}"))
}
