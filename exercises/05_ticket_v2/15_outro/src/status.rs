use std::fmt::Display;

// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.
#[derive(Debug, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug)]
pub enum StatusError {
    InvalidStatusError
}

impl Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid status error")
    }
}

impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tmp = value.to_lowercase();
        if tmp == "todo" {
            Ok(Status::ToDo)
        } else if tmp == "inprogress" {
            Ok(Status::InProgress)
        } else if tmp == "done" {
            Ok(Status::Done)
        } else {
            Err(StatusError::InvalidStatusError)
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = String::from(value);
        let tmp = value.to_ascii_lowercase();
        if tmp == "todo" {
            Ok(Status::ToDo)
        } else if tmp == "inprogress" {
            Ok(Status::InProgress)
        } else if tmp == "done" {
            Ok(Status::Done)
        } else {
            Err(StatusError::InvalidStatusError)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
