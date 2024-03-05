use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, EnumString)]
pub enum Visibility {
    #[strum(ascii_case_insensitive, serialize = "public", serialize = "pub")]
    Public,

    #[strum(
        ascii_case_insensitive,
        serialize = "internal",
        serialize = "pub(module)"
    )]
    Internal,

    #[default]
    #[strum(ascii_case_insensitive, serialize = "private", serialize = "priv")]
    Private,
}
