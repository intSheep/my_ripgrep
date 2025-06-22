use std::path::PathBuf;

/// A high level representation of CLI arguments.
///
/// The distinction between low and high level arguments is somewhat arbitrary
/// and wishy washy. The main idea here is that high level arguments generally
/// require all of CLI parsing to be finished. For example, one cannot
/// construct a glob matcher until all of the glob patterns are known.
///
/// So while low level arguments are collected during parsing itself, high
/// level arguments aren't created until parsing has completely finished.
#[derive(Debug)]
pub(crate) struct HiArgs {
    
}
