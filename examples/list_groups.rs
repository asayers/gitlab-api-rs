
extern crate gitlab_api as gitlab;

use std::env;
use gitlab::GitLab;
use gitlab::Pagination;
use gitlab::{GroupListerOptions, GroupListerOptionsOrderBy};


fn main() {
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
    let mut gl = GitLab::new_https(&hostname, &token);
    gl.set_pagination(Pagination {
        page: 1,
        per_page: 100,
    });
    println!("gl: {:?}", gl);

    let groups = gl.groups(Default::default());
    // let groups = gl.groups(GroupListerOptions { order_by: Some(GroupListerOptionsOrderBy::Path), ..Default::default() });
    println!("groups: {:?}", groups);
}
