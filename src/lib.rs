/*!

A simple *"bring your own queries and types"* GraphQL client.

*/

mod client;
mod error;
mod list;

pub use {
    client::GraphqlClient,
    error::*,
    list::*,
};
