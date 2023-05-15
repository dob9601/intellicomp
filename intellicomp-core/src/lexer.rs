use crate::error::CommandParseError;

#[derive(Debug, PartialEq, Eq)]
enum QuotingState {
    Balanced,
    UnbalancedSingleQuote,
    UnbalancedDoubleQuote,
}

fn get_quoting_state(string: &str) -> QuotingState {
    let mut single_balanced = true;
    let mut double_balanced = true;

    for char in string.chars() {
        if char == '\'' && double_balanced {
            single_balanced = !single_balanced
        } else if char == '\"' && single_balanced {
            double_balanced = !double_balanced
        }
    }

    use QuotingState::*;
    match (single_balanced, double_balanced) {
        (true, true) => Balanced,
        (true, false) => UnbalancedDoubleQuote,
        (false, true) => UnbalancedSingleQuote,
        (false, false) => unreachable!(),
    }
}

pub fn parse_words(mut command: String) -> Result<Vec<String>, CommandParseError> {
    let mut new_word_started = false;

    match get_quoting_state(&command) {
        QuotingState::Balanced => {
            new_word_started = command.ends_with(' ');
        }
        QuotingState::UnbalancedSingleQuote => command.push('\''),
        QuotingState::UnbalancedDoubleQuote => command.push('\"'),
    };

    let mut split_command = shlex::split(&command).ok_or(CommandParseError::UnparseableCommand)?;
    if new_word_started {
        split_command.push("".to_string());
    }
    Ok(split_command)
}

#[cfg(test)]
mod tests {
    use super::{get_quoting_state, parse_words, QuotingState};

    #[test]
    fn test_get_quoting_state_no_quotes() {
        let command = "the quick brown fox jumps over the lazy dog";

        assert_eq!(get_quoting_state(command), QuotingState::Balanced)
    }

    #[test]
    fn test_get_quoting_state_balanced_quotes() {
        let command = "the quick 'brown' \"fox\" jumps 'over' the \"lazy\" dog";

        assert_eq!(get_quoting_state(command), QuotingState::Balanced)
    }

    #[test]
    fn test_get_quoting_state_unbalanced_single_quotes() {
        let command = "the quick 'brown' \"fox\" jumps 'over the \"lazy\" dog";

        assert_eq!(
            get_quoting_state(command),
            QuotingState::UnbalancedSingleQuote
        )
    }

    #[test]
    fn test_get_quoting_state_unbalanced_double_quotes() {
        let command = "the quick 'brown' \"fox\" jumps 'over' the \"lazy dog";

        assert_eq!(
            get_quoting_state(command),
            QuotingState::UnbalancedDoubleQuote
        )
    }

    #[test]
    fn test_get_quoting_state_both_unbalanced() {
        let command = "the quick 'brown \"fox jumps 'over' the \"lazy dog";

        // The first unbalanced quote encountered was a single quote, thus this is a single
        assert_eq!(
            get_quoting_state(command),
            QuotingState::UnbalancedSingleQuote
        )
    }

    #[test]
    fn test_parse_words() {
        let command = "command-name 'longer positional argument' --flag";

        assert_eq!(
            parse_words(command.to_string())
                .expect("Failed to parse command")
                .as_ref(),
            vec!["command-name", "longer positional argument", "--flag"]
        )
    }

    #[test]
    fn test_parse_words_invalid_string() {
        let command = "command-name 'longer positional argument' --flag \\";

        parse_words(command.to_string()).expect_err("Command was unexpectedly treated as valid");
    }

    #[test]
    fn test_parse_words_unclosed_single_quote() {
        let command = "command-name 'longer positional argument' --flag 'partial";

        assert_eq!(
            parse_words(command.to_string())
                .expect("Failed to parse command")
                .as_ref(),
            vec![
                "command-name",
                "longer positional argument",
                "--flag",
                "partial"
            ]
        )
    }

    #[test]
    fn test_parse_words_unclosed_double_quote() {
        let command = "command-name 'longer positional argument' --flag \"partial";

        assert_eq!(
            parse_words(command.to_string())
                .expect("Failed to parse command")
                .as_ref(),
            vec![
                "command-name",
                "longer positional argument",
                "--flag",
                "partial"
            ]
        )
    }
}
