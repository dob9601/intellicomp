use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use self::error::CommandParseError;

mod argument;
pub use argument::{KeywordArgument, PositionalArgument, ValueType};
mod error;

fn quotes_balanced(string: &str) -> bool {
    let mut double_balanced = true;
    let mut single_balanced = true;

    for char in string.chars() {
        if char == '\'' {
            single_balanced = !single_balanced
        } else if char == '\"' {
            double_balanced = !double_balanced
        }
    }

    double_balanced && single_balanced
}

fn parse_words(command: &str) -> Vec<String> {
    let new_word_started = command.ends_with(' ') && quotes_balanced(command);
    let mut split_command = shlex::split(command).unwrap();
    if new_word_started {
        split_command.push("".to_string());
    }
    split_command
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Command {
    /// A brief overview of the command
    pub description: String,

    /// Any top-level arguments, including subcommands.
    #[serde(default)]
    pub keyword_arguments: HashMap<String, KeywordArgument>,

    /// Any top-level arguments which are positional, i.e. passed by position as opposed to by
    /// flag.
    #[serde(default)]
    pub positional_arguments: Vec<PositionalArgument>,
    // / Whether the top-level arguments above are valid anywhere in the command or must appear
    // / before any subcommands.
    // /
    // / If true and a subcommand has an argument with a clashing name, undefined behaviour will
    // / occur.
    // pub arguments_valid_anywhere: bool,
}

impl Command {
    pub fn generate_completions(
        &self,
        command: &str,
        cursor_position: usize,
    ) -> Result<Vec<String>, CommandParseError> {
        if cursor_position > command.len() {
            return Err(CommandParseError::CursorOutOfRange(cursor_position));
        }

        let (command, _) = command.split_at(cursor_position);

        let split_command = parse_words(command);
        let mut tokens: Vec<Token> = vec![];

        let mut positional_argument_index = 0;

        let mut iterator = split_command.into_iter().skip(1);
        while let Some(token) = iterator.next() {
            if let Some((name, argument)) = self
                .keyword_arguments
                .iter()
                .find(|(name, _arg)| name == &&token)
            {
                if argument.value_type == ValueType::Flag {
                    continue;
                }

                // FIXME: Fails on positional arguments - need to treat them differently.
                let value = iterator
                    .next()
                    .ok_or(CommandParseError::ArgumentMissingValue(token))?;

                tokens.push(Token::PopulatedKeywordArgument {
                    name,
                    argument,
                    value,
                })
            } else if positional_argument_index < self.positional_arguments.len() {
                let argument = &self.positional_arguments[positional_argument_index];
                positional_argument_index += 1;
                tokens.push(Token::PopulatedPositionalArgument {
                    argument,
                    value: token,
                })
            } else {
                tokens.push(Token::PartialKeywordArgument(token));
                continue;
            };
        }

        Ok(match tokens.last().unwrap() {
            Token::PopulatedKeywordArgument {
                name: _,
                argument,
                value,
            } => match &argument.value_type {
                ValueType::Flag => unreachable!(),
                ValueType::String => vec![],
                ValueType::Path => self.get_path_completions(value)?,
                ValueType::Enumeration(values) => values
                    .iter()
                    .cloned()
                    .filter(|member| member.starts_with(value))
                    .collect::<Vec<String>>(),
            },
            Token::PopulatedPositionalArgument { argument, value } => {
                let mut results = self.get_valid_keyword_arguments(&tokens, value);

                match &argument.value_type {
                    ValueType::Flag => unreachable!(),
                    ValueType::String => {}
                    ValueType::Path => results.extend(self.get_path_completions(value)?),
                    ValueType::Enumeration(values) => results.extend(
                        values
                            .iter()
                            .cloned()
                            .filter(|member| member.starts_with(value))
                            .collect::<Vec<String>>(),
                    ),
                };

                results
            }
            Token::PartialKeywordArgument(partial) => {
                self.get_valid_keyword_arguments(&tokens, partial)
            }
        })
    }

    fn get_valid_keyword_arguments(&self, tokens: &[Token], query: &str) -> Vec<String> {
        self.keyword_arguments
            .iter()
            .filter_map(|(outer_name, arg)| {
                if outer_name.starts_with(query) {
                    if !arg.repeatable
                        && tokens.iter().any(|token| {
                            if let Token::PopulatedKeywordArgument {
                                name,
                                argument,
                                value: _,
                            } = token
                            {
                                name == outer_name
                            } else {
                                false
                            }
                        })
                    {
                        return None;
                    }
                    Some(outer_name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_path_completions(&self, partial_path: &str) -> Result<Vec<String>, CommandParseError> {
        // This takes self so that future versions may have more advanced path filtering.
        Ok(glob::glob(&format!("./{partial_path}*"))
            .unwrap()
            .map(|maybe_path| maybe_path.map(|path| path.to_string_lossy().to_string()))
            .collect::<Result<Vec<String>, _>>()?)
    }
}

#[derive(Debug)]
enum Token<'a> {
    PopulatedKeywordArgument {
        name: &'a str,
        argument: &'a KeywordArgument,
        value: String,
    },
    PopulatedPositionalArgument {
        argument: &'a PositionalArgument,
        value: String,
    },
    PartialKeywordArgument(String),
}
