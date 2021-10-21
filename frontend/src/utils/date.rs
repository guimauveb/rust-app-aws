use chrono::naive::NaiveDateTime;

pub fn format_date(date_string: &str) -> Result<String, chrono::ParseError> {
    let datetime = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%dT%H:%M:%S.%f")?;
    Ok(datetime.format("%B %d, %Y").to_string())
}
