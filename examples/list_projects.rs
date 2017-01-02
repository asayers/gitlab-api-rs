
extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
// use gitlab::Pagination;
use gitlab::Lister;

use gitlab::errors::*;


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
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

    let gl = GitLab::new(&hostname, &token).chain_err(|| "failure to create GitLab instance")?;
    // let gl = GitLab::new(&hostname, &token)
    //     .chain_err(|| "failure to create GitLab instance")?
    //     .scheme("http")
    //     .port(80);
    // let gl = gl.scheme("http").port(80);

    let projects = gl.projects().list().chain_err(|| "cannot get projects")?;
    println!("projects: {:?}", projects);

    let projects = gl.projects().archived(false).list().chain_err(|| "cannot get projects")?;
    println!("projects: {:?}", projects);

    let projects =
        gl.projects().owned().archived(false).list().chain_err(|| "cannot get projects")?;
    println!("projects: {:?}", projects);

    let projects = gl.projects()
        .all()
        .order_by(gitlab::projects::ListingOrderBy::Name)
        .list()
        .chain_err(|| "cannot get projects")?;
    println!("projects: {:?}", projects);

    let project = gl.projects()
        .id(gitlab::projects::ListingId::Id(9))
        .list()
        .chain_err(|| "failure to find project")?;
    println!("project: {:?}", project);

    let issues = project.issues(&gl)
        .list()
        .chain_err(|| "failure to find project's issues")?;
    println!("issues: {:?}", issues);

    let merge_requests = project.merge_requests(&gl)
        .list()
        .chain_err(|| "failure to find project's merge_requests")?;
    println!("merge_requests: {:?}", merge_requests);

    let issues = gl.projects()
        .id(gitlab::projects::ListingId::Id(9))
        .issues()
        .chain_err(|| "failure to build a gitlab::issues::project::IssuesLister")?
        .list()
        .chain_err(|| "failure to find issues")?;
    println!("issues: {:?}", issues);

    let merge_requests = gl.projects()
        .id(gitlab::projects::ListingId::Id(9))
        .merge_requests()
        .chain_err(|| "failure to build a gitlab::merge_requests::project::IssuesLister")?
        .list()
        .chain_err(|| "failure to find merge requests")?;
    println!("merge_requests: {:?}", merge_requests);

    Ok(())
}
