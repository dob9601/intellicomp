use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use self::error::CommandParseError;

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
    pub keyword_arguments: Vec<Argument>,

    /// Any top-level arguments which are positional, i.e. passed by position as opposed to by
    /// flag.
    #[serde(default)]
    pub positional_arguments: Vec<Argument>,

    /// Whether the top-level arguments above are valid anywhere in the command or must appear
    /// before any subcommands.
    ///
    /// If true and a subcommand has an argument with a clashing name, undefined behaviour will
    /// occur.
    pub arguments_valid_anywhere: bool,
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
            if let Some(argument) = self.keyword_arguments.iter().find(|arg| arg.name == token) {
                if argument.arg_type == ArgumentType::Flag {
                    continue;
                }

                // FIXME: Fails on positional arguments - need to treat them differently.
                let value = iterator
                    .next()
                    .ok_or(CommandParseError::ArgumentMissingValue(token))?;

                tokens.push(Token::PopulatedArgument {
                    argument,
                    value,
                    positional: false,
                })
            } else if positional_argument_index < self.positional_arguments.len() {
                let argument = &self.positional_arguments[positional_argument_index];
                positional_argument_index += 1;
                tokens.push(Token::PopulatedArgument {
                    argument,
                    value: token,
                    positional: true,
                })
            } else {
                tokens.push(Token::PartialKeywordArgument(token));
                continue;
            };
        }

        Ok(match tokens.last().unwrap() {
            Token::PopulatedArgument {
                argument,
                value,
                positional,
            } => {
                let mut results = vec![];
                if *positional {
                    results.extend(self.get_valid_keyword_arguments(&tokens, value))
                }

                match &argument.arg_type {
                    ArgumentType::Flag => unreachable!(),
                    ArgumentType::String => {}
                    ArgumentType::Path => results.extend(self.get_path_completions(value)?),
                    ArgumentType::Enumeration(values) => results.extend(
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
            .filter_map(|arg| {
                if arg.name.starts_with(query) {
                    if !arg.repeatable
                        && tokens.iter().any(|token| {
                            if let Token::PopulatedArgument {
                                argument,
                                value: _,
                                positional: _,
                            } = token
                            {
                                argument.name == arg.name
                            } else {
                                false
                            }
                        })
                    {
                        return None;
                    }
                    Some(arg.name.clone())
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub description: String,
    pub shorthand: Option<String>,

    pub repeatable: bool,

    pub arg_type: ArgumentType,

    // TODO: Finalise the structure of this
    pub incompatible_with: Vec<Argument>,
}

impl Argument {
    pub fn consume_iter<'a, I>(&self, iterator: &mut I) -> Result<Vec<&'a str>, CommandParseError>
    where
        I: Iterator<Item = &'a str>,
    {
        match &self.arg_type {
            ArgumentType::Flag => Ok(vec![]),
            ArgumentType::String => Ok(vec![iterator
                .next()
                .ok_or(CommandParseError::ArgumentMissingValue(self.name.clone()))?]),
            ArgumentType::Path => todo!(),
            ArgumentType::Enumeration(_) => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", content = "content")]
#[non_exhaustive]
pub enum ArgumentType {
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

#[derive(Debug)]
enum Token<'a> {
    PopulatedArgument {
        argument: &'a Argument,
        value: String,
        positional: bool,
    },
    PartialKeywordArgument(String),
}
