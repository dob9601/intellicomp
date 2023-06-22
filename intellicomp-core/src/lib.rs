use lexer::parse_words;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use self::error::CommandParseError;

mod argument;
pub use argument::{KeywordArgument, KeywordArgumentStyle, PositionalArgument, ValueType};
mod error;

mod lexer;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Command {
    /// A brief overview of the command
    pub description: String,

    /// Any top-level arguments, including subcommands.
    #[serde(default)]
    pub keyword_arguments: Vec<KeywordArgument>,

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

        let split_command = parse_words(command.to_string())?;
        let mut tokens: Vec<Token> = vec![];

        let mut positional_argument_index = 0;

        let mut iterator = split_command.into_iter().skip(1);
        while let Some(token) = iterator.next() {
            if let Some(argument) = self
                .keyword_arguments
                .iter()
                .find(|arg| arg.name == token.trim_start_matches("-"))
            {
                if argument.value_type == ValueType::Flag {
                    continue;
                }

                // FIXME: Fails on positional arguments - need to treat them differently.
                let value = iterator
                    .next()
                    .ok_or(CommandParseError::ArgumentMissingValue(token))?;

                tokens.push(Token::PopulatedKeywordArgument { argument, value })
            } else if positional_argument_index < self.positional_arguments.len() {
                let argument = &self.positional_arguments[positional_argument_index];
                positional_argument_index += 1;
                tokens.push(Token::PopulatedPositionalArgument {
                    argument,
                    value: token,
                })
            } else {
                tokens.push(Token::PartialKeywordArgument(
                    token.trim_start_matches('-').to_string(),
                ));
                continue;
            };
        }

        dbg!(&tokens);

        Ok(match tokens.last().unwrap() {
            Token::PopulatedKeywordArgument { argument, value } => match &argument.value_type {
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
            .filter_map(|outer_argument| {
                if outer_argument.name.starts_with(query) {
                    if !outer_argument.repeatable
                        && tokens.iter().any(|token| {
                            if let Token::PopulatedKeywordArgument { argument, value: _ } = token {
                                argument.name == outer_argument.name
                            } else {
                                false
                            }
                        })
                    {
                        return None;
                    }
                    Some(outer_argument.to_string())
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
        argument: &'a KeywordArgument,
        value: String,
    },
    PopulatedPositionalArgument {
        argument: &'a PositionalArgument,
        value: String,
    },
    PartialKeywordArgument(String),
}

#[cfg(test)]
mod tests {

    use crate::{
        argument::KeywordArgumentStyle, error::CommandParseError, Command, KeywordArgument,
        PositionalArgument, ValueType,
    };
    use lazy_static::lazy_static;

    lazy_static! {
        static ref MOCK_COMMAND: Command = Command {
            description: "This is a mock command used for testing".to_string(),
            keyword_arguments: vec![
                KeywordArgument {
                    name: "enum".to_string(),
                    description: "Some argument".to_string(),
                    incompatible_with: vec![],
                    style: KeywordArgumentStyle::Standard,
                    repeatable: false,
                    shorthand: Some('s'),
                    value_type: ValueType::Enumeration(vec![
                        "foo".to_string(),
                        "bar".to_string(),
                        "baz".to_string()
                    ]),
                },
                KeywordArgument {
                    name: "file".to_string(),
                    description: "Some argument".to_string(),
                    incompatible_with: vec![],
                    style: KeywordArgumentStyle::Standard,
                    repeatable: false,
                    shorthand: Some('s'),
                    value_type: ValueType::Path,
                }
            ],
            positional_arguments: vec![PositionalArgument {
                name: "positional".to_string(),
                description: "Some positional argument".to_string(),
                value_type: ValueType::Enumeration(vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string()
                ]),
                incompatible_with: vec![]
            }],
        };
    }

    #[test]
    fn test_generate_enum_completions() {
        let command = "command-name --enum ";
        let completions = MOCK_COMMAND
            .generate_completions(command, command.len())
            .unwrap();

        assert_eq!(completions.as_ref(), vec!["foo", "bar", "baz"])
    }

    #[test]
    fn test_generate_file_completions() {
        let command = "command-name --file ";
        let mut completions = MOCK_COMMAND
            .generate_completions(command, command.len())
            .unwrap();

        let mut expected = std::fs::read_dir("./")
            .unwrap()
            .map(|path| {
                path.map(|path| {
                    path.path()
                        .to_string_lossy()
                        .strip_prefix("./")
                        .unwrap()
                        .to_string()
                })
            })
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        completions.sort();
        expected.sort();

        assert_eq!(completions, expected)
    }

    #[test]
    fn test_generate_keyword_and_positional_completions() {
        let command = "command-name ";
        let completions = MOCK_COMMAND
            .generate_completions(command, command.len())
            .unwrap();

        assert_eq!(
            completions.as_ref(),
            vec!["--enum", "--file", "1", "2", "3"]
        )
    }

    #[test]
    fn test_generate_keyword_completions() {
        let command = "command-name 1 --";
        let completions = MOCK_COMMAND
            .generate_completions(command, command.len())
            .unwrap();

        assert_eq!(completions.as_ref(), vec!["--enum", "--file"])
    }

    #[test]
    fn test_provide_completions_from_partial_argument() {
        let command = "command-name --enum ba";
        let completions = MOCK_COMMAND
            .generate_completions(command, command.len())
            .unwrap();

        assert_eq!(completions.as_ref(), vec!["bar", "baz"])
    }
    #[test]
    fn test_cursor_out_of_range() {
        let command = "command-name --enum ";
        let index = command.len() + 1;
        let error = MOCK_COMMAND
            .generate_completions(command, index)
            .unwrap_err();

        if let CommandParseError::CursorOutOfRange(position) = error {
            assert_eq!(position, index)
        } else {
            panic!("Wrong error variant: {error:?}")
        }
    }
}
