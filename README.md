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

# Why not use it

* For now it's very young, the API may grow and change
* There's already a popular and well tested [GraphQL client in Rust](https://github.com/graphql-rust/graphql-client)

# How to use it

## Simple query

The [github stars example](examples/github-stars/main.rs) demonstrates querying GitHub's GraphQL API to get the number of stars of a repository.

First create a client, that you may keep and reuse:

```rust
let mut graphql_client = GraphqlClient::new("https://api.github.com/graphql")?;
graphql_client.set_bearer_auth("your-github-api-token");
```

You need the struct into which to deserialize the server's answer:

```rust
#[derive(Deserialize)]
pub struct Repository {
    stargazers: Count,
}
```
(`Count` is a utility struct provided by byo_graphql, it's just `struct Count { totalCount: usize }`)

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

Now you can fetch the data:

```rust
let repo: Repository = graphql_client.get_first_item(query)?;
let stars: usize = repo.stargazers.into();
```

## Querying a long list

The [github issues example](examples/github-issues/main.rs) demonstrates how to query a long list with a cursor based exchange.
