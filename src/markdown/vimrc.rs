use super::ToMarkDown;
use crate::parser::vim::*;

impl ToMarkDown for Ast {
    fn to_markdown(&self) -> String {
        let mut out = String::from("## Key Binding");

        out.push_str(
            self.key_bindings
                .iter()
                .fold("".into(), |acc: String, binding| {
                    format!("{} \r\n {}", acc, binding.to_markdown())
                })
                .as_str(),
        );

        out
    }
}

impl ToMarkDown for KeyBinding {
    fn to_markdown(&self) -> String {
        let keys = &self.keys.iter().fold("".into(), |acc: String, key| {
            format!("{} + {}", acc, key.to_string())
        });

        format!(" - {} : {}", self.binding_type.to_string(), keys)
    }
}
