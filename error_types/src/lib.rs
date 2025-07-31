use chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: ((&str, String)),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            form_values: format!("({}: {})", field_name, field_value),
            date: now,
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name == "" {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }
        if self.password.len() < 8 {
            return Err(
                FormError::new(
                    "password",
                    self.password.clone(),
                    "Password should be at least 8 characters long"
                )
            );
        }
let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
let has_digit = self.password.chars().any(|c| c.is_ascii_digit());

        let has_symbol = self.password.chars().any(|c| {
            let c = c as u8;
            (33..=47).contains(&c) ||
                (58..=64).contains(&c) ||
                (91..=96).contains(&c) ||
                (123..=126).contains(&c)
        });

        if !has_letter || !has_symbol  || !has_digit{
            return Err(
                FormError::new(
                    "password",
                    self.password.clone(),
                    "Password must contain ASCII alphanumeric characters and symbols (e.g., <, &, /)"
                )
            );
        }

        Ok(())
    }
}
