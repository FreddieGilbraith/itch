extern crate itch;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Foo {
    b: bool,
    s: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TestEnum {
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Bar {
    f: f64,
    i: i64,
    foos: Vec<Foo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TestStruct {
    bar: Bar,
    e: HashMap<String, TestEnum>,
}

impl TestStruct {
    pub fn new() -> Self {
        Self {
            bar: Bar {
                foos: vec![
                    Foo {
                        b: true,
                        s: "a".to_owned(),
                    },
                    Foo {
                        b: false,
                        s: "ab".to_owned(),
                    },
                    Foo {
                        b: true,
                        s: "abc".to_owned(),
                    },
                ],
                f: 123.456,
                i: 123456,
            },
            e: {
                let mut hm = HashMap::new();
                hm.insert("ein".to_owned(), TestEnum::One);
                hm.insert("zwei".to_owned(), TestEnum::Two);
                hm.insert("drei".to_owned(), TestEnum::Three);

                hm
            },
        }
    }
}
mod from_json {
    use super::*;
    #[test]
    fn to_json() {
        let input_json_string = serde_json::to_string(&TestStruct::new()).unwrap();
        let input_json_bytes: &[u8] = input_json_string.as_bytes();

        let mut output_json_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Json,
            &itch::ToType::Json,
            input_json_bytes,
            &mut output_json_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_json::from_str(std::str::from_utf8(&output_json_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_toml() {
        let input_json_string = serde_json::to_string(&TestStruct::new()).unwrap();
        let input_json_bytes: &[u8] = input_json_string.as_bytes();

        let mut output_toml_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Json,
            &itch::ToType::Toml,
            input_json_bytes,
            &mut output_toml_bytes,
        )
        .unwrap();

        let output: TestStruct =
            toml::from_str(std::str::from_utf8(&output_toml_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_url() {
        let input_json_string = serde_json::to_string(&TestStruct::new()).unwrap();
        let input_json_bytes: &[u8] = input_json_string.as_bytes();

        let mut output_url_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Json,
            &itch::ToType::Url,
            input_json_bytes,
            &mut output_url_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_qs::from_str(std::str::from_utf8(&output_url_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_yaml() {
        let input_json_string = serde_json::to_string(&TestStruct::new()).unwrap();
        let input_json_bytes: &[u8] = input_json_string.as_bytes();

        let mut output_yaml_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Json,
            &itch::ToType::Yaml,
            input_json_bytes,
            &mut output_yaml_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_yaml::from_str(std::str::from_utf8(&output_yaml_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
}
mod from_toml {
    use super::*;
    #[test]
    fn to_json() {
        let input_toml_string = toml::to_string(&TestStruct::new()).unwrap();
        let input_toml_bytes: &[u8] = input_toml_string.as_bytes();

        let mut output_json_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Toml,
            &itch::ToType::Json,
            input_toml_bytes,
            &mut output_json_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_json::from_str(std::str::from_utf8(&output_json_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_toml() {
        let input_toml_string = toml::to_string(&TestStruct::new()).unwrap();
        let input_toml_bytes: &[u8] = input_toml_string.as_bytes();

        let mut output_toml_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Toml,
            &itch::ToType::Toml,
            input_toml_bytes,
            &mut output_toml_bytes,
        )
        .unwrap();

        let output: TestStruct =
            toml::from_str(std::str::from_utf8(&output_toml_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_url() {
        let input_toml_string = toml::to_string(&TestStruct::new()).unwrap();
        let input_toml_bytes: &[u8] = input_toml_string.as_bytes();

        let mut output_url_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Toml,
            &itch::ToType::Url,
            input_toml_bytes,
            &mut output_url_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_qs::from_str(std::str::from_utf8(&output_url_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_yaml() {
        let input_toml_string = toml::to_string(&TestStruct::new()).unwrap();
        let input_toml_bytes: &[u8] = input_toml_string.as_bytes();

        let mut output_yaml_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Toml,
            &itch::ToType::Yaml,
            input_toml_bytes,
            &mut output_yaml_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_yaml::from_str(std::str::from_utf8(&output_yaml_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
}
mod from_yaml {
    use super::*;
    #[test]
    fn to_json() {
        let input_yaml_string = serde_yaml::to_string(&TestStruct::new()).unwrap();
        let input_yaml_bytes: &[u8] = input_yaml_string.as_bytes();

        let mut output_json_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Yaml,
            &itch::ToType::Json,
            input_yaml_bytes,
            &mut output_json_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_json::from_str(std::str::from_utf8(&output_json_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_toml() {
        let input_yaml_string = serde_yaml::to_string(&TestStruct::new()).unwrap();
        let input_yaml_bytes: &[u8] = input_yaml_string.as_bytes();

        let mut output_toml_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Yaml,
            &itch::ToType::Toml,
            input_yaml_bytes,
            &mut output_toml_bytes,
        )
        .unwrap();

        let output: TestStruct =
            toml::from_str(std::str::from_utf8(&output_toml_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_url() {
        let input_yaml_string = serde_yaml::to_string(&TestStruct::new()).unwrap();
        let input_yaml_bytes: &[u8] = input_yaml_string.as_bytes();

        let mut output_url_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Yaml,
            &itch::ToType::Url,
            input_yaml_bytes,
            &mut output_url_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_qs::from_str(std::str::from_utf8(&output_url_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
    #[test]
    fn to_yaml() {
        let input_yaml_string = serde_yaml::to_string(&TestStruct::new()).unwrap();
        let input_yaml_bytes: &[u8] = input_yaml_string.as_bytes();

        let mut output_yaml_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::Yaml,
            &itch::ToType::Yaml,
            input_yaml_bytes,
            &mut output_yaml_bytes,
        )
        .unwrap();

        let output: TestStruct =
            serde_yaml::from_str(std::str::from_utf8(&output_yaml_bytes).unwrap()).unwrap();
        assert_eq!(output, TestStruct::new());
    }
}
