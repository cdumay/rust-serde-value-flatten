use serde::Serialize;
use serde_value_flatten::to_flatten_maptree;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug)]
struct SubFoo {
    a: String,
    b: u64,
}

#[derive(Serialize)]
struct Foo {
    a: String,
    b: f64,
    c: Vec<i8>,
    d: SubFoo,
    e: Option<HashMap<String, String>>,
}

#[test]
fn test_sub_hashmap() {
    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("hey"), "one".into());
    let foo = Foo {
        a: "test".into(),
        b: 0.5,
        c: vec![5, 9],
        d: SubFoo {
            a: "subtest".into(),
            b: 695217,
        },
        e: Some(hashmap),
    };
    let ser = to_flatten_maptree("_", Some("npm_"), &foo).unwrap();
    let res_str = serde_json::to_string_pretty(&ser).unwrap();
    #[cfg(not(feature = "ovh-ldp"))]
    let expected_str = r#"{
  "npm_a": "test",
  "npm_b": 0.5,
  "npm_c_0": 5,
  "npm_c_1": 9,
  "npm_d_a": "subtest",
  "npm_d_b": 695217,
  "npm_e_hey": "one"
}"#;
    #[cfg(feature = "ovh-ldp")]
    let expected_str = r#"{
  "npm_a": "test",
  "npm_b_float": 0.5,
  "npm_c_0_long": 5,
  "npm_c_1_long": 9,
  "npm_d_a": "subtest",
  "npm_d_b_double": 695217,
  "npm_e_hey": "one"
}"#;
    assert_eq!(res_str, expected_str);
}
