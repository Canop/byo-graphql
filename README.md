[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/byo-graphql.svg
[l1]: https://crates.io/crates/byo-graphql

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/byo-graphql/badge.svg
[l3]: https://docs.rs/byo-graphql/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3


A simple *"bring your own queries and types"* GraphQL client.

# Design Principles

* I want to use my own Rust structures and not have them generated by macros
* I want to use my own queries and not have them generated by macros
* I want to manage queries as strings in my Rust code
* I don't know what I'm doing

# Why not use it

* There's already a popular and well tested [GraphQL client in Rust](https://github.com/graphql-rust/graphql-client)
* I only tested basic queries, no mutations
* I don't know what I'm doing

# How to use it

The [github example](examples/github-stars/main.rs) demonstrates fetching the number of stars of a repository.

You create a client, that you may keep and reuse:

```rust
let mut graphql_client = GraphqlClient::new("https://api.github.com/graphql")?;
graphql_client.set_bearer_auth("your-github-api-token");
```

You need the structs into which to deserialize the server's answer:

```rust
#[derive(Deserialize)]
pub struct Repository {
    stargazers: RepoStargazers,
}
#[derive(Deserialize)]
pub struct RepoStargazers {
    totalCount: usize,
}
```
And you need a query:
```rust
let query = r#"{
	repository(owner: "Canop", name: "bacon") {
	    stargazers {
		totalCount
	    }
	}
}"#;
```
**note:** in the example's complete code, the query is dynamically built with `format!`, as you'll usually do.

With this, you can fetch and display the data:

```rust
let repo: Repository = graphql_client.get_first(query)?;
println!("stars: {}", repo.stargazers.totalCount);
```
