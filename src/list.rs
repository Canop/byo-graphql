use {
    serde::Deserialize,
    std::convert::AsRef,
};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct PageInfo {
    pub startCursor: Option<String>,
    pub endCursor: Option<String>,
    pub hasPreviousPage: Option<bool>,
    pub hasNextPage: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct List<Item> {
    pub totalCount: Option<usize>,
    pub nodes: Vec<Item>,
    pub pageInfo: Option<PageInfo>,
}

impl<Item> List<Item> {
    pub fn query_page_selector<S: AsRef<str>>(
        after: &Option<S>,
        first: usize, // page max size
        other_clauses: &str, // filters & ordering clauses
    ) -> String {
        if let Some(cursor) = after.as_ref() {
            format!(r#"(first:{} after:"{}" {})"#, first, cursor.as_ref(), other_clauses)
        } else {
            format!(r#"(first:{} {})"#, first, other_clauses)
        }
    }
    pub fn query_page_body(
        item_query_part: &str,
    ) -> String {
        format!(r#"{{
            totalCount
            nodes {}
            pageInfo {{
                startCursor
                endCursor
                hasPreviousPage
                hasNextPage
            }}
        }}"#, item_query_part)
    }
    pub fn next_page_cursor(self) -> Option<String> {
        if let Some(page_info) = self.pageInfo {
            if page_info.hasNextPage==Some(true) {
                return page_info.endCursor;
            }
        }
        None
    }
}

/// a structure matching `{ totalCount}`, convenient
/// when you want the number of items in a collections
#[derive(Debug, Deserialize, Clone, Copy)]
#[allow(non_snake_case)]
pub struct Count {
    pub totalCount: usize,
}
impl Into<usize> for Count {
    fn into(self) -> usize {
        self.totalCount
    }
}
impl Count {
    /// build the query part for the count of something.
    ///
    /// Can be used for a whole query or just a property.
    /// Example:
    /// ```
    /// use byo_graphql::*;
    /// assert_eq!(
    ///     Count::query("repositories", "isFork: false"),
    ///     r#"repositories(isFork: false){ totalCount }"#,
    /// );
    /// ```
    pub fn query(
        collection_name: &str,
        filter: &str,
    ) -> String {
        format!(r#"{}({}){{ totalCount }}"#, collection_name, filter)
    }

}
