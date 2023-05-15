use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct KeywordArgument {
    pub name: String,
    pub description: String,
    pub shorthand: Option<char>,

    pub repeatable: bool,

    pub style: KeywordArgumentStyle,
    pub value_type: ValueType,

    #[serde(default)]
    pub incompatible_with: Vec<String>,
}

impl ToString for KeywordArgument {
    fn to_string(&self) -> String {
        let prefix = match self.style {
            KeywordArgumentStyle::Standard => "--",
            KeywordArgumentStyle::Old => "-",
        };

        format!("{prefix}{}", self.name)
    }
}

#[derive(Debug, PartialEq, Eq, JsonSchema, Serialize, Deserialize)]
pub enum KeywordArgumentStyle {
    /// The usual double-dash prefix for a keyword argument, present on most commands
    Standard,
    /// The older single-dash prefix for a keyword argument, present on commands such as `find`
    Old,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PositionalArgument {
    pub name: String,
    pub description: String,

    pub value_type: ValueType,

    #[serde(default)]
    pub incompatible_with: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", content = "content")]
#[non_exhaustive]
pub enum ValueType {
    /// The argument is a flag and thus does not have an associated value.
    Flag,

    /// The value of the argument should be treated as a free-text string and no completion can be
    /// done for it.
    String,

    Path,

    /// The value of the argument must be one of a given set of strings.
    Enumeration(Vec<String>),
    // Subcommand {
    //     keyword_arguments: Vec<Argument>,
    //     positional_arguments: Vec<Argument>,
    // },
}
