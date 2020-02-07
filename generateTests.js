const prelude = `extern crate itch;

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
}`;

const inputs = {
  //url: ` let input_url_string = serde_qs::to_string(&TestStruct::new()).unwrap(); let input_url_bytes: &[u8] = input_url_string.as_bytes();`,
  //xml: ` let input_xml_string = serde_xml_rs::to_string(&TestStruct::new()).unwrap(); let input_xml_bytes: &[u8] = input_xml_string.as_bytes();`,
  json: ` let input_json_string = serde_json::to_string(&TestStruct::new()).unwrap(); let input_json_bytes: &[u8] = input_json_string.as_bytes();`,
  toml: ` let input_toml_string = toml::to_string(&TestStruct::new()).unwrap(); let input_toml_bytes: &[u8] = input_toml_string.as_bytes();`,
  yaml: ` let input_yaml_string = serde_yaml::to_string(&TestStruct::new()).unwrap(); let input_yaml_bytes: &[u8] = input_yaml_string.as_bytes();`
};

const outputs = {
  //xml: `let output: TestStruct = serde_xml_rs::from_str(std::str::from_utf8(&output_xml_bytes).unwrap()).unwrap();`,
  json: `let output: TestStruct = serde_json::from_str(std::str::from_utf8(&output_json_bytes).unwrap()).unwrap();`,
  toml: `let output: TestStruct = toml::from_str(std::str::from_utf8(&output_toml_bytes).unwrap()).unwrap();`,
  url: `let output: TestStruct = serde_qs::from_str(std::str::from_utf8(&output_url_bytes).unwrap()).unwrap();`,
  yaml: `let output: TestStruct = serde_yaml::from_str(std::str::from_utf8(&output_yaml_bytes).unwrap()).unwrap();`
};

console.log(prelude);

function enumify(s) {
  return s
    .split("")
    .map((x, i) => (i === 0 ? x.toUpperCase() : x))
    .join("");
}

for (const input of Object.keys(inputs)) {
  console.log(`mod from_${input} { use super::*;`);

  for (const output of Object.keys(outputs)) {
    console.log(`#[test]
		fn to_${output}() { `);

    console.log(inputs[input]);

    console.log(`
        let mut output_${output}_bytes: Vec<u8> = Vec::new();

        itch::convert(
            &itch::FromType::${enumify(input)},
			&itch::ToType::${enumify(output)},
			input_${input}_bytes,
            &mut output_${output}_bytes,
        )
		.unwrap();
		`);

    console.log(outputs[output]);

    console.log("assert_eq!(output, TestStruct::new());}");
  }
  console.log("}");
}
