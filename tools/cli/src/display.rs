use chrono::{DateTime, Duration, Utc};

/// Display information about encrypted tokens with status and expiry
pub fn display_encrypted_token_info(
    token_name: &str,
    token: &Option<String>,
    expiry: &Option<i64>,
) {
    match token {
        Some(token_value) => {
            println!("{}:", token_name);

            // Display expiry if available
            if let Some(expiry_ms) = expiry {
                if let Some(expiry_dt) = DateTime::from_timestamp_millis(*expiry_ms) {
                    let now = Utc::now();
                    let time_left = expiry_dt - now;

                    let status = if expiry_dt > now {
                        if time_left.num_minutes() < 10 {
                            "⚠️ Expires Soon"
                        } else {
                            "✅ Valid"
                        }
                    } else {
                        "❌ Expired"
                    };

                    println!(
                        "  Expires: {} UTC ({})",
                        expiry_dt.format("%Y-%m-%d %H:%M:%S"),
                        status
                    );

                    if expiry_dt > now {
                        println!("  Time Left: {}", format_duration(time_left));
                    }
                }
            } else {
                println!("  Expires: ❓ Unknown (no expiry data)");
            }

            println!(
                "  Preview: {}...",
                &token_value[..20.min(token_value.len())]
            );
            println!("  Length: {} characters\n", token_value.len());
        }
        None => {
            println!("{}: ❌ Not found\n", token_name);
        }
    }
}

/// Format a Duration into a human-readable string
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if days > 0 {
        if hours > 0 {
            format!("{}d {}h", days, hours)
        } else {
            format!("{}d", days)
        }
    } else if hours > 0 {
        if minutes > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}h", hours)
        }
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
