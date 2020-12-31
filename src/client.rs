use {
    crate::*,
    serde::{ de::DeserializeOwned, Deserialize, Serialize },
    std:: collections::HashMap,
};

/// A client that you may reuse to do several queries
pub struct GraphqlClient {
    requester: reqwest::blocking::Client,
    url: String,
    bearer_auth: Option<String>,
}

#[derive(Debug, Serialize)]
struct GraphqlRequest {
    query: String,
}

#[derive(Debug, Deserialize)]
struct GraphqlResponse<D> {
    data: Option<D>,
    errors: Option<Vec<GraphqlError>>,
}

impl GraphqlClient {
    /// create a client
    pub fn new<S: Into<String>>(url: S) -> ByoResult<Self> {
        let requester = reqwest::blocking::Client::builder()
            .user_agent("byo/0.1")
            .build()?;
        Ok(Self {
            requester,
            url: url.into(),
            bearer_auth: None,
        })
    }
    /// specify the optional authentication token which will be used
    /// in all requests
    pub fn set_bearer_auth<S: Into<String>>(&mut self, auth: S) {
        self.bearer_auth = Some(auth.into());
    }
    /// return a raw reqwest Response. You should usually not
    /// need this function
    pub fn raw<S: Into<String>>(&self, query: S) -> ByoResult<reqwest::blocking::Response> {
        let mut builder = self.requester.post(&self.url);
        if let Some(auth) = &self.bearer_auth {
            builder = builder.bearer_auth(auth);
        }
        let res = builder
            .json(&GraphqlRequest{ query: query.into() })
            .send()?
            .error_for_status()?;
        Ok(res)
    }
    /// get the server's answer as unparsed text.
    /// This is mainly useful to debug and tune your structures or query
    pub fn text<S: Into<String>>(&self, query: S) -> ByoResult<String> {
        Ok(self.raw(query)?.text()?)
    }
    /// get the `data` part of the answer in the desired type
    /// (it usually looks like a map)
    pub fn get_data<S: Into<String>, Data: DeserializeOwned>(&self, query: S) -> ByoResult<Data> {
        let res = self.raw(query)?;
        let response: GraphqlResponse<Data> = res.json()?;
        if let Some(errors) = response.errors {
            Err(ByoError::Graphql(errors))
        } else {
            response.data.ok_or(ByoError::NoData)
        }
    }
    /// get the first item in the answer, if present.
    /// This is a convenience method for the simplest case, most
    /// especially for when you query a unique item.
    pub fn get_first_item<S: Into<String>, Item: DeserializeOwned>(&self, query: S) -> ByoResult<Item> {
        let mut map: HashMap<String, Option<Item>> = self.get_data(query)?;
        let single = map.drain().next().and_then(|e| e.1);
        single.ok_or(ByoError::NoData)
    }
}
