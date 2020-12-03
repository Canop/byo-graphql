use {
    anyhow::*,
    serde::{*, de::DeserializeOwned},
    std::{
        collections::HashMap,
    },
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
    data: D,
}

impl GraphqlClient {
    /// create a client
    pub fn new<S: Into<String>>(url: S) -> Result<Self> {
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
    pub fn raw<S: Into<String>>(&self, query: S) -> Result<reqwest::blocking::Response> {
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
    pub fn text<S: Into<String>>(&self, query: S) -> Result<String> {
        Ok(self.raw(query)?.text()?)
    }
    /// get the `data` part of the answer in the desired type
    /// (it usually looks like a map)
    pub fn get<S: Into<String>, D: DeserializeOwned>(&self, query: S) -> Result<D> {
        let res = self.raw(query)?;
        let response: GraphqlResponse<D> = res.json()?;
        Ok(response.data)
    }
    /// get the first item in the answer, if present.
    /// This is a convenience method for the simplest case.
    pub fn get_first<S: Into<String>, D: DeserializeOwned>(&self, query: S) -> Result<D> {
        let mut map: HashMap<String, D> = self.get(query)?;
        let single = map.drain()
            .next()
            .map(|e| e.1)
            .ok_or_else(|| format_err!("empty data in server response"));
        single
    }
}
