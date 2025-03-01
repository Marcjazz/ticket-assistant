pub fn extract_numbers_from_text(text: String, max_threshhold: u32) -> Vec<u32> {
    let mut nums = vec![];
    let mut num_string = String::new();
    for char in text.chars() {
        if char.is_digit(10) {
            num_string.push(char);
        } else {
            if let Ok(num) = num_string.parse::<u32>() {
                if num <= max_threshhold && !nums.contains(&num) {
                    nums.push(num);
                    num_string = String::new()
                }
            }
        }
    }

    nums
}

pub fn parse_bool(input: &str) -> Option<bool> {
    match input {
        "1" | "true" => Some(true),
        "0" | "false" => Some(false),
        _ => None, // Handle invalid inputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pr_diff = r#"
            #[tokio::main]
            async fn main() -> Result<(), Box<dyn std::error::Error>> {
                let args = env::args().collect::<Vec<String>>();

                if args.len() < 2 {
                    eprintln!("Missing requirements")
                }
                let gh_token = env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN found!");
                let gh_repo = env::var("GITHUB_REPOSITORY").expect("No GITHUB_REPOSITORY found!");
                let gh_api = GhAPIClient::new(&gh_token, &gh_repo);

                let [enable_fib, max_threshhold, pr_number, ..] = args.as_slice() else {
                    panic!("Failed to read 03 args");
                };

                let pr_number = pr_number.parse().expect("Could Not parsed pr number");
                let pr_diff_entry = gh_api
                    .get_pull_request_files(pr_number)
                    .await;


                Ok(())
            }
        "#;

        assert_eq!(
            extract_numbers_from_text(pr_diff.to_string(), 5),
            vec![2, 3]
        );
        assert_eq!(extract_numbers_from_text(pr_diff.to_string(), 2), vec![2])
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("false"), Some(false));
        assert_eq!(parse_bool("yes"), None); // Invalid input
        assert_eq!(parse_bool("no"), None); // Invalid input
    }
}
