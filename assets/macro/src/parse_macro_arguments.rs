use syn::{parse::ParseStream, Ident, LitBool, LitInt, LitStr, Token};

pub fn parse_named_string_argument(
    argument_name: &'static str,
    input: &ParseStream,
) -> Option<String> {
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

pub fn parse_named_bool_argument(argument_name: &'static str, input: &ParseStream) -> Option<bool> {
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

pub fn parse_named_u64_argument(argument_name: &'static str, input: &ParseStream) -> Option<u64> {
    // Parse the argument name.
    let parsed_argument_name: Ident = input.parse().ok()?;
    if parsed_argument_name != argument_name {
        return None;
    }

    // Parse the colon.
    let _: Token![:] = input.parse().ok()?;

    // Parse the argument value.
    let argument_value_literal: LitInt = input.parse().ok()?;
    let argument_value = argument_value_literal.base10_parse::<u64>().ok()?;

    // Parse the optional comma after the argument. Note how
    // we ignore the error here since the comma is optional.
    let _: Result<Token![,], _> = input.parse();

    Some(argument_value)
}
