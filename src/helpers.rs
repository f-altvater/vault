#[derive(Debug)]
pub struct InternalError {
    code: String,
    message: String,
}
impl InternalError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: String::from(code),
            message: String::from(message),
        }
    }
    pub fn code(&self) -> String {
        String::from(&self.code)
    }
    pub fn message(&self) -> String {
        String::from(&self.message)
    }
    pub fn toast_message(&self) -> String {
        format!("{} - {}", &self.message, &self.code)
    }
}

pub fn timestamp_as_date(timestamp: i64) -> String {
    let date_time = chrono::DateTime::from_timestamp(timestamp, 0)
        .expect("invalid timestamp");
    let local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(date_time);
    let date_string = local.naive_local().to_string();
    let parts = date_string.split(' ')
        .collect::<Vec<&str>>();

    let raw_date = parts[0]
        .split('-')
        .collect::<Vec<&str>>();

    let mut date_formatted = String::new();
    let mut count = 1;

    for part in raw_date.iter().rev() {
        date_formatted.push_str(*part);
        if count < raw_date.len() {
            date_formatted.push_str(".");
        }
        count += 1;
    }

    let local_datetime = vec![&date_formatted, parts[1]];

    local_datetime.join(" ")

}