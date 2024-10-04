use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Itch {
    Obj(IndexMap<String, Itch>),
    Array(Vec<Itch>),
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

#[derive(Clone, Debug)]
pub enum FromType {
    Json,
    Toml,
    Url,
    Yaml,
    Xml,
    Cbor,
}

impl std::str::FromStr for FromType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cbor" => Ok(Self::Cbor),
            "json" => Ok(Self::Json),
            "qs" => Ok(Self::Url),
            "toml" => Ok(Self::Toml),
            "url" => Ok(Self::Url),
            "xml" => Ok(Self::Xml),
            "yaml" => Ok(Self::Yaml),
            "yml" => Ok(Self::Yaml),
            _ => Err(format!("could not parse `{}` as an input type", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ToType {
    Json,
    Toml,
    Url,
    Yaml,
    Xml,
    Cbor,
}

impl std::str::FromStr for ToType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cbor" => Ok(Self::Cbor),
            "json" => Ok(Self::Json),
            "qs" => Ok(Self::Url),
            "toml" => Ok(Self::Toml),
            "url" => Ok(Self::Url),
            "xml" => Ok(Self::Xml),
            "yaml" => Ok(Self::Yaml),
            _ => Err(format!("could not parse `{}` as an output type", s)),
        }
    }
}

pub fn convert<From: std::io::Read, To: std::io::Write>(
    from_type: &FromType,
    to_type: &ToType,
    mut input: From,
    mut output: To,
) -> Result<(), String> {
    let itch: Itch =
        match from_type {
            FromType::Json => serde_json::from_reader(input)
                .map_err(|e| format!("error parsing json: `{}`", e))?,

            FromType::Xml => serde_xml_rs::from_reader(input)
                .map_err(|e| format!("error parsing xml: `{}`", e))?,

            FromType::Yaml => serde_yaml::from_reader(input)
                .map_err(|e| format!("error parsing yaml: `{}`", e))?,

            FromType::Toml => {
                let mut s = String::new();
                input
                    .read_to_string(&mut s)
                    .map_err(|e| format!("error parsing toml: `{}`", e))?;
                toml::from_str(&s).map_err(|e| format!("error parsing toml: `{}`", e))?
            }

            FromType::Url => {
                let mut s = String::new();
                input
                    .read_to_string(&mut s)
                    .map_err(|e| format!("error parsing url query string: `{}`", e))?;
                serde_qs::from_str(&s)
                    .map_err(|e| format!("error parsing url query string: `{}`", e))?
            }

            FromType::Cbor => {
                serde_cbor::from_reader(input).map_err(|e| format!("error parsing cbor `{}`", e))?
            }
        };

    match to_type {
        ToType::Json => serde_json::to_writer(output, &itch).map_err(|e| {
            dbg!(&e);
            format!("error outputting json: `{}`", e)
        })?,

        ToType::Url => serde_qs::to_writer(&itch, &mut output)
            .map_err(|e| format!("error outputting url query string: `{}`", e))?,

        ToType::Xml => serde_xml_rs::to_writer(output, &itch)
            .map_err(|e| format!("error outputting xml: `{}`", e))?,

        ToType::Yaml => serde_yaml::to_writer(output, &itch)
            .map_err(|e| format!("error outputting yaml: `{}`", e))?,

        ToType::Toml => {
            let s = toml::to_string_pretty(&itch)
                .map_err(|e| format!("error outputting toml: `{}`", e))?;
            output
                .write(s.as_bytes())
                .map(|_| ())
                .map_err(|e| format!("error outputting toml: `{}`", e))?
        }

        ToType::Cbor => serde_cbor::to_writer(output, &itch)
            .map_err(|e| format!("error outputing cbor `{}`", e))?,
    };

    Ok(())
}
