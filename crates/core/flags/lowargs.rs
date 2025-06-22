
/// A "special" mode that supercedes everything else.
///
/// When one of these modes is present, it overrides everything else and causes
/// ripgrep to short-circuit. In particular, we avoid converting low-level
/// argument types into higher level arguments types that can fail for various
/// reasons related to the environment. (Parsing the low-level arguments can
/// fail too, but usually not in a way that can't be worked around by removing
/// the corresponding arguments from the CLI command.) This is overall a hedge
/// to ensure that version and help information are basically always available.
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub(crate) enum SpecialMode {
    HelpShort,
    HelpLong,
    VersionShort,
    VersionLong,
    VersionPCRE2,
}
