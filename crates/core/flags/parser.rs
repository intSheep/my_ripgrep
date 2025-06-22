use anyhow::Context;
use crate::flags::lowargs::SpecialMode;

/// The result of parsing CLI arguments.
///
/// This is basically a `anyhow::Result<T>`, but with one extra variant that is
/// inhabited whenever ripgrep should execute a "special" mode. That is, when a
/// user provides the `-h/--help` or `-V/--version` flags.
///
/// This special variant exists to allow CLI parsing to short circuit as
/// quickly as is reasonable. For example, it lets CLI parsing avoid reading
/// ripgrep's configuration and converting low level arguments into a higher
/// level representation.
#[derive(Debug)]
pub(crate) enum ParseResult<T>{
    Special(SpecialMode),
    Ok(T),
    Err(anyhow::Error)
}

/// Parse CLI arguments and convert then to their high level representation.
pub(crate) fn parse() -> ParseResult<HiArgs> {
    parse_low().and_then(|low| match HiArgs::from_low_args(low) {
        Ok(hi) => ParseResult::Ok(hi),
        Err(err) => ParseResult::Err(err),
    })
}