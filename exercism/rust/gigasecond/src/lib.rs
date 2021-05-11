use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    use chrono::TimeZone;
    
    let start_in_milis = start.timestamp();
    
    Utc.timestamp(start_in_milis + 1_000_000_000, 0)
}
