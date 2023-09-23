use serde::Deserialize;
use std::fmt::Display;
use worker::*;

#[derive(Debug, Deserialize)]
pub struct ContactFormSubmission {
    name: String,
    email: String,
    message: String,
}

impl ContactFormSubmission {
    pub async fn from_request(mut req: Request) -> Result<Self> {
        let form_data = req.form_data().await?;
        form_data.try_into()
    }
}

impl Display for ContactFormSubmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ContactFormSubmission {{ name: {}, email: {}, message: {} }}",
            self.name, self.email, self.message
        )
    }
}

impl TryFrom<worker::FormData> for ContactFormSubmission {
    type Error = Error;
    fn try_from(form_data: worker::FormData) -> std::result::Result<Self, Self::Error> {
        let name = form_data
            .get_field("name")
            .ok_or(Error::RustError("Missing name.".to_string()))?;
        let email = form_data
            .get_field("email")
            .ok_or(Error::RustError("Missing email.".to_string()))?;
        let message = form_data
            .get_field("message")
            .ok_or(Error::RustError("Missing message.".to_string()))?;

        Ok(Self {
            name,
            email,
            message,
        })
    }
}

trait FormDataExtension {
    fn get_field(&self, field_name: &str) -> Option<String>;
}

impl FormDataExtension for worker::FormData {
    fn get_field(&self, field_name: &str) -> Option<String> {
        let field = match self.get(field_name) {
            Some(form_entry) => form_entry,
            None => return None,
        };

        match field {
            worker::FormEntry::Field(text) => Some(text),
            worker::FormEntry::File(_file) => None,
        }
    }
}
