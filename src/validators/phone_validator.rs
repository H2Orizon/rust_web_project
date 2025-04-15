use validator::ValidationError;

pub fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    let cleaned: String = phone.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect();

    let re = regex::Regex::new(r"^\+?\d{10,15}$").unwrap();

    if re.is_match(&cleaned) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_phone");
        err.message = Some("Невірний формат номера телефону".into());
        Err(err)
    }
}