/*!
This example fetches the number of stars of a github repository.

Run the example with

    GITHUB_API_TOKEN=your-github-api-token cargo run --example github-stars

To get a Github API token, see https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token

*/

use {
    anyhow::*,
    byo_graphql::*,
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct Repository {
    stargazers: Count,
}

pub fn get_repo_stars(owner: &str, name: &str) -> Result<usize> {
    let mut graphql_client = GraphqlClient::new("https://api.github.com/graphql")?;
    graphql_client.set_bearer_auth(std::env::var("GITHUB_API_TOKEN")?);
    let query = format!(
        // remember: to escape a { or } in a fmt string, you must double it
        r#"{{ repository(owner: "{}", name: "{}") {} }}"#,
        owner,
        name,
        r#"{ stargazers { totalCount } }"#,
    );
    // If you want to see what is exchanged, uncomment those two lines
    //println!("query: {}", &query);
    //println!("raw answer: {}", graphql_client.text(&query)?);
    let repo: Repository = graphql_client.get_first_item(query)?;
    Ok(repo.stargazers.into())
}

pub fn main() -> Result<()> {
    let (owner, name) = ("Canop", "bacon");
    println!(
        "Repository {}/{} has {} stars",
        owner, name,
        get_repo_stars(owner, name)?
    );
    Ok(())
}
