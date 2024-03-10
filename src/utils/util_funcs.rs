use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Utc};

/// The User-Agent used in HTTP requests in some parser implmentations
/// ```
/// use consumet::utils::util_funcs::USER_AGENT;
/// use reqwest::{Client, header};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = Client::new();
///
///     let res = client
///         .get("https://www.rust-lang.org")
///         .header(header::USER_AGENT, USER_AGENT)
///         .send()
///         .await?
///         .text()
///         .await?;
///
///     Ok(())
/// }
/// ```
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36";

/// Days enum used for the get_day function
pub enum Days {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// Utility Functions for some parsers
pub trait UtilFuncs {
    /// Pass in a author string and get a Vector
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let author_string = "author1, author2";
    /// let author_vec = author_string.split_author();
    ///
    /// assert_eq!(vec!["author1", "author2"], author_vec);
    /// ```
    fn split_author(&self) -> Vec<String>;

    /// Pass in a id string and get the floored version
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let id = "123456";
    /// let floored_id = id.floor_id();
    ///
    /// assert_eq!(123000, floored_id);
    /// ```
    fn floor_id(&self) -> i32;

    /// Pass in a title and get the formatted version
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let title = "Remy Clarke123 The Movie   ";
    /// let formatted_title = title.format_title();
    ///
    /// assert_eq!("Remy Clarke The Movie", formatted_title);
    /// ```
    fn format_title(&self) -> String;

    /// Pass in a String and get the first letter capitalized
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let to_be_capitalized = "remy Clarke";
    /// let capitalized = to_be_capitalized.capitalize_first_letter();
    ///
    /// assert_eq!("Remy Clarke", capitalized);
    /// ```
    fn capitalize_first_letter(&self) -> String;

    /// Pass in a String and to_find and get the substring before it
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let substring = "remy Clarke";
    /// let found = substring.substring_before("Clarke").expect("Unable to find substring!");
    ///
    /// assert_eq!("remy ", found);
    /// ```
    fn substring_before(&self, to_find: &str) -> Result<String, String>;

    /// Pass in a String and to_find and get the substring after it
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;;
    ///
    /// let substring = "remy Clarke";
    /// let found = substring.substring_before("remy").expect("Unable to find substring!");
    ///
    /// assert_eq!("", found);
    /// ```
    fn substring_after(&self, to_find: &str) -> Result<String, String>;

    /// Pass in a String and to_find and get the substring before the last occurrence of it
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let substring = "remy Clarke remy";
    /// let found = substring.substring_before_last("remy").expect("Unable to find substring!");
    ///
    /// assert_eq!("remy Clarke ", found);
    /// ```
    fn substring_before_last(&self, to_find: &str) -> Result<String, String>;

    /// Pass in a String and to_find and get the substring after the last occurrence of it
    /// ```
    /// use consumet::utils::util_funcs::UtilFuncs;
    ///
    /// let substring = "remy Clarke remy Clarke";
    /// let found = substring.substring_after_last("remy").expect("Unable to find substring!");
    ///
    /// assert_eq!(" Clarke", found);
    /// ```
    fn substring_after_last(&self, to_find: &str) -> Result<String, String>;
}

/// Turns day numbers 0..6 into Unix Timestamps
pub fn get_day(day: Days) -> i64 {
    let day_index = match day {
        Days::Sunday => 0,
        Days::Monday => 1,
        Days::Tuesday => 2,
        Days::Wednesday => 3,
        Days::Thursday => 4,
        Days::Friday => 5,
        Days::Saturday => 6,
    };

    let local_date = Local::now();
    let current_day = local_date.weekday().num_days_from_sunday();

    let x = (day_index as u32 + 7 - current_day) % 7;

    Utc::now().timestamp() + (x as i64 * 86400)
}

/// Wrapper for the get_day function
/// ```
/// use consumet::utils::util_funcs::{get_days, Days};
///
/// // Assuming today is 8/24/23
/// let days = get_days(vec![Days::Sunday, Days::Saturday]); // Output: [1690723824, 1690637424]
/// ```
pub fn get_days(days: Vec<Days>) -> Vec<i64> {
    let mut day_vec: Vec<i64> = vec![];

    for day in days {
        day_vec.push(get_day(day));
    }

    day_vec
}

/// Turns milliseconds into 24 Hour Format
/// ```
/// use consumet::utils::util_funcs::convert_duration;
///
/// let milliseconds = 23409823;
/// let duration = convert_duration(milliseconds);
///
/// assert_eq!(duration, "PT00H36M40S".to_owned());
/// ```
pub fn convert_duration(milliseconds: i64) -> String {
    let timestamp = milliseconds * 1000;
    let native = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&native);
    datetime.format("PT%HH%MM%SS").to_string()
}

impl UtilFuncs for str {
    fn split_author(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut eater = String::new();

        for (i, c) in self.chars().enumerate() {
            if c == ' '
                && (self.chars().nth(i - 1) == Some(',') || self.chars().nth(i - 1) == Some(';'))
            {
                continue;
            }
            if c == ',' || c == ';' {
                res.push(eater.trim().to_string());
                eater.clear();
                continue;
            }
            eater.push(c);
        }
        res.push(eater);
        res
    }

    fn floor_id(&self) -> i32 {
        let mut imp = String::new();
        for (i, c) in self.chars().enumerate() {
            if i < self.len() - 3 {
                imp.push(c);
            }
        }
        let id_v: i32 = imp.parse().unwrap_or(0);
        id_v * 1000
    }

    fn format_title(&self) -> String {
        let result = self.chars().filter(|c| !c.is_numeric()).collect::<String>();
        result.trim().to_owned()
    }

    fn capitalize_first_letter(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            Some(f) => f.to_uppercase().chain(c).collect(),
            None => String::new(),
        }
    }

    fn substring_before(&self, to_find: &str) -> Result<String, String> {
        let text = self.split_once(to_find).map(|t| String::from(t.0));
        match text {
            Some(text) => Ok(text),
            None => Err(format!("to_find=({to_find})")),
        }
    }

    fn substring_after(&self, to_find: &str) -> Result<String, String> {
        let text = self.split_once(to_find).map(|t| String::from(t.1));
        match text {
            Some(text) => Ok(text),
            None => Err(format!("to_find=({to_find})")),
        }
    }

    fn substring_before_last(&self, to_find: &str) -> Result<String, String> {
        let text = self.rsplit_once(to_find).map(|t| String::from(t.0));
        match text {
            Some(text) => Ok(text),
            None => Err(format!("to_find=({to_find})")),
        }
    }

    fn substring_after_last(&self, to_find: &str) -> Result<String, String> {
        let text = self.rsplit_once(to_find).map(|t| String::from(t.1));
        match text {
            Some(text) => Ok(text),
            None => Err(format!("to_find=({to_find})")),
        }
    }
}
