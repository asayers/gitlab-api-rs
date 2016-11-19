// Inspired by http://python-gitlab.readthedocs.io/en/stable/

extern crate hyper;

pub mod gitlab;
pub mod projects;

pub use gitlab::GitLab;
// pub use projects::ProjectManager;




#[cfg(test)]
mod tests {
    use gitlab::GitLab;
    use hyper;

    #[test]
    fn it_works() {

        let gl = GitLab::new("http", "gitlab.com", 80, "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);
        assert_eq!(gl.attempt_connection().unwrap().status, hyper::status::StatusCode::Unauthorized);

        let gl = GitLab::new_http("gitlab.com", "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);
        assert_eq!(gl.attempt_connection().unwrap().status, hyper::status::StatusCode::Unauthorized);

        let gl = GitLab::new_https("gitlab.com", "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);
        assert_eq!(gl.attempt_connection().unwrap().status, hyper::status::StatusCode::Unauthorized);

        println!("gl.build_url(): {:?}", gl.build_url("test"));

        // Example from GitLab: https://docs.gitlab.com/ce/api/#basic-usage
        let expected_url = "https://gitlab.example.com:443/api/v3/projects?private_token=9koXpg98eAheJpvBs5tK";
        let gl = GitLab::new_https("gitlab.example.com", "9koXpg98eAheJpvBs5tK");
        assert_eq!(gl.build_url("projects"), expected_url);
    }
}
