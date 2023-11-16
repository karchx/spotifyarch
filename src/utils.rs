
/// formats a time duration into a "{minutes}:{seconds}" format
pub fn format_duration(duration: &chrono::Duration) -> String {
    let secs = duration.num_seconds();
    format!("{}:{:02}", secs / 60, secs % 60)
}
