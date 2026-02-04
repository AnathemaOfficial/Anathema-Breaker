/// Progression: canonical topological position.
/// Lexicon: { RZ, EP, IZ } with IZ terminal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Progression {
    RZ,
    EP,
    IZ,
}
