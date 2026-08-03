use regex::Regex;

pub fn is_cn_mobile(s: &str) -> bool {
    static RE: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"^1[3-9]\d{9}$").unwrap());

    RE.is_match(s)
}

pub fn is_email(s: &str) -> bool {
    static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap()
    });

    RE.is_match(s)
}
