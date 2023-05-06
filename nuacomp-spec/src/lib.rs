use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Command {
    /// A brief overview of the command
    description: String,


    subcommands: Vec<Subcommand>,

    /// Any top level arguments, typically prefixed with "--", that are present in the command
    arguments: Vec<Argument>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Subcommand {
    /// The name of the subcommand, as it should be typed by the user.
    name: String,

    /// A brief description of what the subcommand does.
    description: String,

    /// Any arguments, typically prefixed with "--", that are present specifically for this
    /// subcommand.
    arguments: Vec<Argument>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Argument {
    name: String,
    description: String,

    // TODO: Finalise the structure of this
    incompatible_with: Vec<Argument>
}
