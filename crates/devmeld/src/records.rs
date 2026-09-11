//! Serialization of DevMeld-owned records, separate from authored JSON resources.
use crate::{Result, error};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub(crate) fn encode(value: &impl Serialize) -> Result<Vec<u8>> {
    Ok(toml::to_string_pretty(value)?.into_bytes())
}

pub(crate) fn decode<T: DeserializeOwned>(bytes: &[u8], path: &Path) -> Result<T> {
    toml::from_slice(bytes).map_err(|e| error(format!("{}: {e}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Evidence {
        path: String,
        text: String,
    }

    #[test]
    fn exact_text_and_path_keys_survive_toml_without_json_string_nesting() {
        let path = r"\\?\C:\资料\shop\.devmeld\context.toml";
        for text in [
            "# 团队\n\nssh / http\n",
            "\nfirst\r\nsecond\r\n",
            "\"\"\" and ''' and \\ and trailing slash\\\n",
            "\0\u{0001}\t\r\u{007f}",
            "",
        ] {
            let value = BTreeMap::from([(
                path.to_owned(),
                Evidence {
                    path: path.into(),
                    text: text.into(),
                },
            )]);
            let bytes = encode(&value).unwrap();
            let restored: BTreeMap<String, Evidence> =
                decode(&bytes, Path::new("receipt.toml")).unwrap();
            assert_eq!(restored, value);
            // A literal TOML path, not a JSON-escaped path string.
            assert!(String::from_utf8(bytes).unwrap().contains(path));
        }
    }
}
