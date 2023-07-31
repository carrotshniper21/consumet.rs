use std::collections::HashMap;

/// Parse post info and turn it into a formatted version
/// # Parameters
/// * `post` - the post string slice
pub fn parse_post_info(post: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut year = String::new();
    let mut size = String::new();
    let mut description = String::new();
    let mut size_done = false;

    let mut i = 0;
    while i < post.len() {
        if i + 5 < post.len() && &post[i..i + 5] == "Year " {
            year = post[i + 7..i + 11].to_string();
            i += 10;
        } else if i + 5 < post.len() && &post[i..i + 5] == "Size " {
            let mut j = i + 7;
            let temp = j;
            while j < temp + 4 {
                if post[j..j + 1].parse::<i32>().is_ok() {
                    size += &post[j..j + 1];
                } else {
                    break;
                }
                j += 1;
            }
            size += &post[j..j + 2];
            i += j - i;
            i += 2;
            size_done = true;
        }
        if size_done {
            description += &post[i..i + 1];
        }
        i += 1;
    }

    description = description[..description.len() - 12].to_string();
    result.insert("year".to_string(), year);
    result.insert("size".to_string(), size);
    result.insert("description".to_string(), description);

    result
}
