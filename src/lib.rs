//    Copyright 2018 webd
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

extern crate reqwest;
use reqwest::{Client as HttpClient, Response as HttpResponse, Result as HttpResult};

pub struct Client<'a> {
    pub api_key: &'a str,
    pub http_client: HttpClient,
    endpoint: &'a str,
}

impl<'a> Client<'a> {
    pub fn new<T>(api_key: T) -> Client<'a>
    where
        T: Into<&'a str>,
    {
        Client {
            api_key: api_key.into(),
            http_client: HttpClient::new(),
            endpoint: "https://api.github.com/graphql",
        }
    }

    /**
     * Makes a request to the API and returns the response
     */
    pub fn make_request(&self, body: &'static str) -> HttpResult<HttpResponse> {
        let response: HttpResponse = self
            .http_client
            .post(self.endpoint)
            .bearer_auth(self.api_key)
            .body(body)
            .send()?;
        Ok(response)
    }
}
