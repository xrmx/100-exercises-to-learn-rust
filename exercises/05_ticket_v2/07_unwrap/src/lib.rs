// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let title_copy = title.clone();
    let status_copy = status.clone();
    match Ticket::new(title_copy, description, status_copy) {
        Ok(ticket) => {
            return ticket;
        },
        Err(Errors::TitleEmpty) => {
            panic!("Title cannot be empty");
        }
        Err(Errors::TitleTooLong) => {
            panic!("Title cannot be longer than 50 bytes");
        },
        Err(Errors::DescriptionEmpty) | Err(Errors::DescriptionTooLong) => {
            return easy_ticket(title, "Description not provided".to_string(), status);
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Errors {
    TitleEmpty,
    TitleTooLong,
    DescriptionEmpty,
    DescriptionTooLong,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, Errors> {
        if title.is_empty() {
            return Err(Errors::TitleEmpty);
        }
        if title.len() > 50 {
            return Err(Errors::TitleTooLong);
        }
        if description.is_empty() {
            return Err(Errors::DescriptionEmpty);
        }
        if description.len() > 500 {
            return Err(Errors::DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
