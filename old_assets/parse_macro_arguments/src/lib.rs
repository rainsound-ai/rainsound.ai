use syn::{parse::ParseStream, Ident, LitBool, LitStr, Token};

pub fn parse_named_string_argument(
    argument_name: &'static str,
    input: &ParseStream,
    // argument_position: ArgumentPosition,
) -> Option<String> {
    // if let ArgumentPosition::NotFirst = argument_position {
    //     // Parse the comma that separates this argument from the previous one.
    //     let _: Token![,] = input.parse().ok()?;
    // }

    // Parse the argument name.
    let parsed_argument_name: Ident = input.parse().ok()?;
    if parsed_argument_name != argument_name {
        return None;
    }

    // Parse the colon.
    let _: Token![:] = input.parse().ok()?;

    // Parse the argument value.
    let argument_value_literal: LitStr = input.parse().ok()?;

    // Parse the optional comma after the argument. Note how
    // we ignore the error here since the comma is optional.
    let _: Result<Token![,], _> = input.parse();

    Some(argument_value_literal.value())
}

pub fn parse_named_bool_argument(
    argument_name: &'static str,
    input: &ParseStream,
    // argument_position: ArgumentPosition,
) -> Option<bool> {
    // if let ArgumentPosition::NotFirst = argument_position {
    //     // Parse the comma that separates this argument from the previous one.
    //     let _: Token![,] = input.parse().ok()?;
    // }

    // Parse the argument name.
    let parsed_argument_name: Ident = input.parse().ok()?;
    if parsed_argument_name != argument_name {
        return None;
    }

    // Parse the colon.
    let _: Token![:] = input.parse().ok()?;

    // Parse the argument value.
    let argument_value_literal: LitBool = input.parse().ok()?;

    // Parse the optional comma after the argument. Note how
    // we ignore the error here since the comma is optional.
    let _: Result<Token![,], _> = input.parse();

    Some(argument_value_literal.value())
}

// pub enum ArgumentPosition {
//     First,
//     NotFirst,
// }
