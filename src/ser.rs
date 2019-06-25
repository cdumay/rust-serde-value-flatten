// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use std::collections::BTreeMap;

use serde_value::Value;

pub struct FlatSerializer {
    key_separator: String,
    prefix: String,
}

impl FlatSerializer {
    pub fn new(key_separator: String, prefix: String) -> FlatSerializer {
        FlatSerializer { key_separator, prefix }
    }
    #[cfg(not(feature = "ovh-ldp"))]
    fn format_key(&self, xpath: &str, key: &str, _value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("{}{}", self.prefix, k),
            (x, k) => format!("{}{}{}", x, self.key_separator, k)
        }
    }
    #[cfg(feature = "ovh-ldp")]
    fn _schema_suffix(&self, value: &Value) -> String {
        match *value {
            Value::Bool(_) => format!("{}bool", self.key_separator),
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) => format!("{}double", self.key_separator),
            Value::I8(_) | Value::I16(_) | Value::I32(_) | Value::I64(_) => format!("{}long", self.key_separator),
            Value::F32(_) | Value::F64(_) => format!("{}float", self.key_separator),
            _ => "".into()
        }
    }
    #[cfg(feature = "ovh-ldp")]
    fn format_key(&self, xpath: &str, key: &str, value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("{}{}{}", self.prefix, k, self._schema_suffix(value)),
            (x, k) => format!("{}{}{}{}", x, self.key_separator, k, self._schema_suffix(value)),
        }
    }

    pub fn disassemble(&self, xpath: &str, key: &str, value: &Value) -> BTreeMap<Value, Value> {
        let mut parts = BTreeMap::new();
        match value {
            Value::Map(ref tree) => {
                for (k, v) in tree.iter() {
                    let subkey = match k {
                        Value::String(data) => format!("{}", data),
                        Value::Char(data) => format!("{}", data),
                        _ => panic!("Map keys MUST be strings or char")
                    };
                    parts.append(&mut self.disassemble(&self.format_key(xpath, &key, value), &subkey, v));
                };
            }
            Value::Seq(ref values) => {
                for (i, val) in values.iter().enumerate() {
                    parts.append(&mut self.disassemble(&mut self.format_key(xpath, key, value), &format!("{}", i), val));
                }
            }
            _ => {
                parts.insert(Value::String(self.format_key(xpath, key, value)), value.clone());
            }
        };
        parts
    }
}
