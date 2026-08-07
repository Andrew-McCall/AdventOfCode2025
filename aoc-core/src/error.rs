use std::fmt::Display;

/// Declares an error enum along with its `Display` and `Error` impls.
///
/// Each variant is written as `Name { field: Type, .. } => "message"`, where the
/// message is a format string that can name the variant's own fields:
///
/// ```
/// # use aoc_core::error;
/// error! {
///     pub enum Bad {
///         TooBig { limit: usize } => "over the limit of {limit}",
///     }
/// }
/// ```
///
/// Fields have to implement `Display`, since that is all the message can do with
/// them. Paths are the usual catch: store `path.display().to_string()`.
#[macro_export]
macro_rules! error {
    (
        $(#[$outer:meta])*
        $visibility:vis enum $name:ident {
            $(
                $variant:ident { $($field:ident: $type:ty),* $(,)? } => $message:literal
            ),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug)]
        $visibility enum $name {
            $( $variant { $($field: $type),* } ),*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        Self::$variant { $($field),* } => {
                            // Not every field has to appear in the message.
                            $( let _ = $field; )*
                            write!(f, $message)
                        }
                    ),*
                }
            }
        }

        impl ::std::error::Error for $name {}
    };
}

error! {
    pub enum Error {
        MissingInput { day: u8, path: String } =>
            "no input for day {day}: put it in {path}",
        Unreadable { path: String, cause: std::io::Error } =>
            "could not read {path}: {cause}",
        NotUtf8 { path: String } =>
            "{path} is not valid UTF-8",
        Parse { detail: String } =>
            "bad input: {detail}",
        Solve { detail: String } =>
            "{detail}",
    }
}

impl Error {
    pub fn parse(detail: impl Display) -> Self {
        Error::Parse {
            detail: detail.to_string(),
        }
    }

    pub fn solve(detail: impl Display) -> Self {
        Error::Solve {
            detail: detail.to_string(),
        }
    }
}
