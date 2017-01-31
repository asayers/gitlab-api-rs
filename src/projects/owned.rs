//! List owned projects
//!
//! https://docs.gitlab.com/ce/api/projects.html#list-owned-projects
//!
//! # List owned projects
//!
//! Get a list of projects which are owned by the authenticated user.
//!
//! ```text
//! GET /projects/owned
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `archived` | boolean | no | Limit by archived status |
//! | `visibility` | string | no | Limit by visibility `public`, `internal`, or `private` |
//! | `order_by` | string | no | Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, or `last_activity_at` fields. Default is `created_at` |
//! | `sort` | string | no | Return projects sorted in `asc` or `desc` order. Default is `desc` |
//! | `search` | string | no | Return list of authorized projects matching the search criteria |


use serde_urlencoded;

use BuildQuery;

use ::errors::*;


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    internal: ::projects::OwnedProjectListerInternal,
}

impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            internal: ::projects::OwnedProjectListerInternal {
                archived: None,
                visibility: None,
                order_by: None,
                sort: None,
                search: None,
            },
        }
    }

    pub fn archived(&'a mut self, archived: bool) -> &'a mut ProjectsLister {
        self.internal.archived = Some(archived);
        self
    }

    pub fn visibility(&'a mut self, visibility: ::ListingVisibility) -> &'a mut ProjectsLister {
        self.internal.visibility = Some(visibility);
        self
    }

    pub fn order_by(&'a mut self, order_by: ::projects::ListingOrderBy) -> &'a mut ProjectsLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut ProjectsLister {
        self.internal.sort = Some(sort);
        self
    }

    pub fn search(&'a mut self, search: String) -> &'a mut ProjectsLister {
        self.internal.search = Some(search);
        self
    }

    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> Result<::projects::Projects> {
        // let query = serde_urlencoded::to_string(&self);
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }
}

impl<'a> BuildQuery for ProjectsLister<'a> {
    fn build_query(&self) -> String {

        let encoded = serde_urlencoded::to_string(&self.internal).unwrap();
        let mut query = String::from("projects/owned");
        if !encoded.is_empty() {
            query.push_str("?");
            query.push_str(&encoded);
        }
        debug!("query: {}", query);

        query
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects/owned";

        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let projects_lister = gl.projects().owned();
        let query = projects_lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.projects().owned().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/owned?archived=true";

        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.archived(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?archived=false";

        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.archived(false).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_visibility() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/owned?visibility=public";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?visibility=internal";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?visibility=private";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/owned?order_by=id";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?order_by=name";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?order_by=path";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?order_by=created_at";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?order_by=updated_at";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?order_by=last_activity_at";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.order_by(::projects::ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().order_by(::projects::ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/owned?sort=asc";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/owned?sort=desc";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_search() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/owned?search=SearchPattern";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/owned?archived=true&sort=desc";
        let mut projects_lister = gl.projects().owned();
        let query = projects_lister.archived(true).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().owned().archived(true).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }
}
