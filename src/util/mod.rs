pub enum CompareResult {
    Less,
    Greater,
    Equal,
}

pub trait Compare {
    fn cmp(&self, other: &f64) -> CompareResult;
}

impl Compare for f64 {
    fn cmp(&self, other: &f64) -> CompareResult {
        if (*self - other).abs() < 1e-9 {
            CompareResult::Equal
        } else if *self < *other {
            CompareResult::Less
        } else {
            CompareResult::Greater
        }
    }
}

/// Trims quotes around the passed string 
///  
/// # Examples
/// ```
/// let val = "\"something\"";
/// 
/// let result = valid_env::util::trim_quotes(val);
/// 
/// assert_eq!(result, "something");
/// ```
pub fn trim_quotes(val: &str) -> String {
    let mut trimmed = String::from(val);

    let quote_start = val.find("\"");

    if let Some(start) = quote_start {
        let quote_end = val.rfind("\"");

        if let Some(end) = quote_end {
            trimmed = String::from(&val[start + 1..end]);
        }
    }

    trimmed
}