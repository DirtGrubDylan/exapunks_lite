/// This enum dictates which communication mode the [`Exa`] is in.
///
/// * Global - The "M" register can be written/read by all other EXAs also in Global mode.
/// * Local - The "M" register can be written/read by all other EXAs in the same [`Host`] that are
///   also in Local mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommunicationMode {
    Global,
    Local,
}
