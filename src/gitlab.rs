
use std::io::Read;  // Trait providing read_to_string()

use std::env;

use hyper;
use serde;
use serde_json;


use Version;
use Projects;
use Groups;


pub const API_VERSION: u16 = 3;




#[derive(Debug)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

#[derive(Debug)]
pub struct GitLab {
    scheme: String,
    domain: String,
    port: u16,
    private_token: String,
    client: hyper::Client,
    pagination: Pagination,
}


impl GitLab {

    pub fn new(scheme: &str, domain: &str, port: u16, private_token: &str) -> GitLab {
        GitLab {
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            port:   port,
            private_token: private_token.to_string(),
            client: match env::var("HTTP_PROXY") {
                Ok(proxy) => {
                    let proxy: Vec<&str> = proxy.trim_left_matches("http://").split(':').collect();
                    let hostname = proxy[0].to_string();
                    let port = proxy[1];

                    hyper::Client::with_http_proxy(hostname, port.parse().unwrap())
                },
                Err(_) => hyper::Client::new(),
            },
            pagination: Pagination {page: 1, per_page: 20},
        }
    }

    pub fn new_http(domain: &str, private_token: &str) -> GitLab {
        GitLab::new("http", &domain, 80, &private_token)
    }

    pub fn new_https(domain: &str, private_token: &str) -> GitLab {
        GitLab::new("https", &domain, 443, &private_token)
    }

    pub fn build_url(&self, command: &str) -> String {
        format!("{}://{}:{}/api/v{}/{}?private_token={}&page={}&per_page={}",
                                self.scheme,
                                self.domain,
                                self.port,
                                API_VERSION,
                                command,
                                self.private_token,
                                self.pagination.page,
                                self.pagination.per_page)
    }

    pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
        let url = self.build_url("version");
        // Close connections after each GET.
        let res = self.client.get(&url).header(hyper::header::Connection::close()).send();

        res
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = pagination;
    }

    pub fn get<T>(&self, command: &str) -> Result<T, serde_json::Error>
            where T: serde::Deserialize {

        let url = self.build_url(command);
        let mut res: hyper::client::Response =
                        self.client
                        .get(&url)
                        .header(hyper::header::Connection::close())
                        .send()
                        .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        serde_json::from_str(&body.as_str())
    }

    pub fn version(&self) -> Result<Version, serde_json::Error> {
        self.get("version")
    }

    pub fn groups(&self) -> Result<Groups, serde_json::Error> {
        self.get("groups")
    }

    pub fn projects(&self) -> Result<Projects, serde_json::Error> {
        self.get("projects")
    }
}


/*
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
*/
