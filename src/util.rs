use validator::ValidationError;
use errors::Errors;

pub fn extract_string<'a>(
    maybe_string: &'a Option<String>,
    field_name: &'static str,
    errors: &mut Errors,
) -> &'a str {
    maybe_string
        .as_ref()
        .map(String::as_str)
        .unwrap_or_else(|| {
            errors.add(field_name, ValidationError::new("can't be blank"));
            ""
        })
}
