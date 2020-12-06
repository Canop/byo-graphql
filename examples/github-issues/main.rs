/*!
This example fetches the complete list of issues of a repository.

Run the example with

    GITHUB_API_TOKEN=your-github-api-token cargo run --example github-issues

To get a Github API token, see https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token

*/

use {
    anyhow::*,
    byo_graphql::*,
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct Issue {
    number: usize,
    title: String,
    state: String,
}

type Issues = List<Issue>;

#[derive(Debug, Deserialize)]
pub struct Repository {
    issues: Issues,
}

pub fn get_open_issues(owner: &str, name: &str) -> Result<Vec<Issue>> {
    let mut graphql_client = GraphqlClient::new("https://api.github.com/graphql")?;
    graphql_client.set_bearer_auth(std::env::var("GITHUB_API_TOKEN")?);
    let mut issues = Vec::new();
    // Querying long lists in graphql involves getting pages one after one.
    // Pagination is cursor based: we start with none and we set the one
    // returned by the response as page position in the following queries
    let mut cursor: Option<String> = None;
    let page_size = 10; // github supports 100 but I want to demonstrate pages
    loop {
        let query = format!(
            r#"{{ repository(owner: "{}" name: "{}") {{ issues{}{} }} }}"#,
            owner, name,
            Issues::query_page_selector(&cursor, page_size, "states: OPEN"),
            Issues::query_page_body("{ number title state }"),
        );
        // If you want to observe what happens, uncomment those 2 lines
        // println!("query: {}", &query);
        // println!("raw answer: {}", graphql_client.text(&query)?);
        let mut repo: Repository = graphql_client.get_first_item(&query)?;
        issues.append(&mut repo.issues.nodes);
        cursor = repo.issues.next_page_cursor();
        if cursor.is_none() {
            break;
        }
    }
    Ok(issues)
}

pub fn main() -> Result<()> {
    for issue in get_open_issues("Canop", "broot")? {
        println!("#{} {} {}", issue.number, issue.state, issue.title);
    }
    Ok(())
}
