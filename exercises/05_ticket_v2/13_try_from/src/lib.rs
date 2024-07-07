// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
enum  StatusError {
    WrongStatus(String)
}

impl PartialEq<String> for Status {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::ToDo => {
                other == "todo"
            },
            Self::InProgress => {
                other == "inprogress"
            },
            Self::Done => {
                other == "done"
            }
        }
    }
}


impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut value = value.clone();
        value.make_ascii_lowercase();
        if Status::ToDo == value {
            Ok(Status::ToDo)
        } else if Status::InProgress == value {
            Ok(Status::InProgress)
        } else if Status::Done == value {
            Ok(Status::Done)
        } else {
            Err(StatusError::WrongStatus("Wrong status".into()))
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = String::from(value);
        value.make_ascii_lowercase();
        if Status::ToDo == value {
            Ok(Status::ToDo)
        } else if Status::InProgress == value {
            Ok(Status::InProgress)
        } else if Status::Done == value {
            Ok(Status::Done)
        } else {
            Err(Self::Error::WrongStatus("Wrong status".into()))
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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
