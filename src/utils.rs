pub fn parse_os(user_agent: &str) -> Option<String> {
    if user_agent.contains("Windows") {
        Some("Windows".to_string())
    } else if user_agent.contains("Mac OS X") {
        Some("macOS".to_string())
    } else if user_agent.contains("Linux") {
        Some("Linux".to_string())
    } else if user_agent.contains("Android") {
        Some("Android".to_string())
    } else if user_agent.contains("iOS") {
        Some("iOS".to_string())
    } else {
        Some("Other".to_string())
    }
}

pub fn parse_browser(user_agent: &str) -> (Option<String>, Option<String>) {
    // Common browser patterns
    let patterns = [
        ("Firefox/", "Firefox"),
        ("Chrome/", "Chrome"),
        ("Safari/", "Safari"),
        ("Edge/", "Edge"),
        ("Opera/", "Opera"),
    ];

    for (pattern, name) in patterns {
        if let Some(idx) = user_agent.find(pattern) {
            // Extract version
            let version_start = idx + pattern.len();
            let version = user_agent[version_start..]
                .split_whitespace()
                .next()
                .map(|v| v.split('/').next().unwrap_or(v))
                .map(String::from);

            return (Some(name.to_string()), version);
        }
    }

    (Some("Other".to_string()), None)
}

pub fn parse_language(accept_language: &str) -> Option<String> {
    accept_language.split(',').next().map(|lang| {
        lang.split(';')
            .next()
            .unwrap_or("unknown")
            .trim()
            .to_string()
    })
}
