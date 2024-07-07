// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.

use std::fmt::Display;

#[derive(Debug)]
pub enum TitleError {
    EmptyTitleError,
    TooLongTileError
}

impl Display for TitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitleError::EmptyTitleError => write!(f, "The title cannot be empty"),
            Self::TooLongTileError => write!(f, "The title cannot be longer than 50 bytes")
        }
    }
}

#[derive(Debug, Clone)]
pub struct TicketTitle(String);

impl PartialEq<&str> for TicketTitle {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq for TicketTitle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 0 {
            Err(TitleError::EmptyTitleError)
        } else if value.len() > 50 {
            Err(TitleError::TooLongTileError)
        } else {
            Ok(TicketTitle(value))
        }
    }

}

impl TryFrom<&str> for TicketTitle {
    type Error = TitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() <= 0 {
            Err(TitleError::EmptyTitleError)
        } else if value.len() > 50 {
            Err(TitleError::TooLongTileError)
        } else {
            Ok(TicketTitle(value.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
