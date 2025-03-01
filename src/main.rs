mod fibbot;
mod gh_api;
mod utils;

use num_bigint::BigUint;
use std::env;
use tokio;

use fibbot::fibonacci;
use gh_api::GhAPIClient;
use utils::{extract_numbers_from_text, parse_bool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();

    println!("Agrs: {:?}", args);
    
    if args.len() != 4 {
        panic!("Missmatch required params, exactly two params are required: enable_fib, max_threshhold and pr_number")
    }

    let gh_token = env::var("GITHUB_TOKEN").expect("Could not read GITHUB_TOKEN from env.");
    let gh_repo =
        env::var("GITHUB_REPOSITORY").expect("Could not read GITHUB_REPOSITORY form env.");

    let gh_api = GhAPIClient::new(&gh_token, &gh_repo);

    let [_, enable_fib, max_threshhold, pr_number] = args.as_slice() else {
        panic!("Failed to read args");
    };

    let enable_fib =
        parse_bool(&enable_fib.trim()).expect("Could not parsed enable_fib from params");

    let max_threshhold = max_threshhold
        .trim()
        .parse()
        .expect("Could not parsed max threshhold from params");

    let pr_number = pr_number.parse().expect("Could not parsed pr number");
    let pr_diff_entry = gh_api.get_pull_request_files(pr_number).await?;

    let mut numbers = Vec::new();

    for item in pr_diff_entry {
        if let Some(text) = item.patch {
            numbers.append(&mut extract_numbers_from_text(text, max_threshhold));
        }
    }

    println!("Numbers from PR content: {:?}", numbers);

    if enable_fib {
        let fibonaccies = numbers
            .iter()
            .map(|num| (*num, fibonacci(*num)))
            .collect::<Vec<(u32, BigUint)>>();

        let comment = if fibonaccies.len() == 0 {
            format!("Numberless PR: Nothing to Compute...")
        } else {
            format!("Fobonaccies: {:?}", fibonaccies)
        };

        println!("{comment}");

        // Post response as PR comments
        gh_api.post_issue_comment(pr_number, &comment).await?;
    } else {
        println!("Fibbot was enabled!")
    }

    Ok(())
}
