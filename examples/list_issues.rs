
extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
// use gitlab::Pagination;
use gitlab::issues;


fn main() {
    env_logger::init().unwrap();
    info!("starting up");

    let hostname = match env::var("GITLAB_HOSTNAME") {
        Ok(val) => val,
        Err(_) => {
            let default = String::from("gitlab.com");
            println!("Please set environment variable 'GITLAB_HOSTNAME'. Using default '{}'.",
                     default);
            default
        }
    };

    let token = match env::var("GITLAB_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set environment variable 'GITLAB_TOKEN'. Take it from \
                    http://{}/profile/account",
                   hostname);
        }
    };

    let gl = GitLab::new_https(&hostname, &token);

    let issues = gl.issues(issues::Listing::new()).unwrap();
    println!("issues: {:?}", issues);

    let listing = issues::Listing::new().state(issues::ListingState::Opened).clone();
    let opened_issues = gl.issues(listing).unwrap();
    println!("opened_issues: {:?}", opened_issues);

    let listing = issues::Listing::new().state(issues::ListingState::Closed).clone();
    let closed_issues = gl.issues(listing).unwrap();
    println!("closed_issues: {:?}", closed_issues);
}
