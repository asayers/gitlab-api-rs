
extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
// use gitlab::Pagination;


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

    let gl = GitLab::new(&hostname, &token);
    // let gl = GitLab::new(&hostname, &token).scheme("http").port(80);
    // let gl = gl.scheme("http").port(80);

    let projects = gl.projects().list();
    println!("projects: {:?}", projects);

    let projects = gl.projects().archived(false).list();
    println!("projects: {:?}", projects);

    let projects = gl.projects().owned().archived(false).list();
    println!("projects: {:?}", projects);

    let projects = gl.projects().all().order_by(gitlab::projects::ListingOrderBy::Name).list();
    println!("projects: {:?}", projects);

}
