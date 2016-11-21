
extern crate gitlab_api as gitlab;

use std::env;
use gitlab::GitLab;
use gitlab::Pagination;

fn main() {
    let token = match env::var("GITLAB_TOKEN") {
        Ok(val) => val,
        Err(_)  => panic!("Please set environment variable 'GITLAB_TOKEN'"),
    };

    let mut gl = GitLab::new_https("gitlab.com", &token);
    // for i in 1..82 {
    //     gl.set_pagination(Pagination{page: i, per_page: 1});
    //     println!("projects: {:?}", gl.projects().unwrap());
    // }
    gl.set_pagination(Pagination{page: 1, per_page: 100});
    let projects = gl.projects().unwrap();
    for project in projects {
        println!("{:?}", project.path_with_namespace);
    }
}
