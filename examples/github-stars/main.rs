/*!
Run the example with

    GITHUB_API_TOKEN=your-github-api-token cargo run --example github-stars

To get a Github API token, see https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token

*/

use {
    anyhow::*,
    byo_graphql::*,
    serde::Deserialize,
};

static REPO_GQL_BODY: &str = r#"{
    stargazers {
        totalCount
    }
}"#;

#[derive(Debug, Deserialize)]
pub struct Repository {
    stargazers: RepoStargazers,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct RepoStargazers {
    totalCount: usize,
}

pub fn main() -> Result<()> {
    let repo_owner = "Canop";
    let repo_name = "bacon";
    let mut graphql_client = GraphqlClient::new("https://api.github.com/graphql")?;
    graphql_client.set_bearer_auth(std::env::var("GITHUB_API_TOKEN")?);
    // Let's make our own query
    // remember: to escape a { or } in a fmt string, you must double it
    let query = format!(
        r#"{{ repository(owner: "{}", name: "{}") {} }}"#,
        repo_owner,
        repo_name,
        REPO_GQL_BODY,
    );
    println!("query: {}", &query);
    println!("raw answer: {}", graphql_client.text(&query)?);
    let repo: Repository = graphql_client.get_first_item(query)?;
    println!("stars: {}", repo.stargazers.totalCount);
    Ok(())
}
