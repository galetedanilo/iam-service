pub trait Event: Send + Sync {
    fn event_type(&self) -> &str;
    fn to_json(&self) -> String;
    fn occurred_at(&self) -> &chrono::DateTime<chrono::Utc>;
}
