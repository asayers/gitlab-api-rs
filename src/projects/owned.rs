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


use serde_json;
use serde_urlencoded;

use BuildQuery;
use gitlab::GitLab;
use Projects;
use projects::{OwnedProjectListerInternal, ListingOrderBy};


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    internal: OwnedProjectListerInternal,
}

impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            internal: OwnedProjectListerInternal {
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

    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut ProjectsLister {
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
    pub fn list(&self) -> Projects {
        // let query = serde_urlencoded::to_string(&self);
        let query = self.build_query();
        debug!("query: {:?}", query);

        let projects: Result<Projects, serde_json::Error> = self.gl.get(&query);

        projects.unwrap()
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


// impl BuildQuery for Listing {
//     fn build_query(&self) -> String {
//
//         let mut query = String::from("projects/owned");
//
//         let amp_char = "&";
//         let none_char = "";
//         let mut split_char = &none_char;
//
//         // Append a "?" only if at least one of the `Option` is `Some(_)` or if
//         // strings contain something.
//         query.push_str(match (&self.archived,
//                               &self.visibility,
//                               &self.order_by,
//                               &self.sort,
//                               self.search.is_empty()) {
//             (&None, &None, &None, &None, true) => "",
//             _ => "?",
//         });
//
//         self.archived.map(|archived| {
//             query.push_str(split_char);
//             split_char = &amp_char;
//
//             if archived {
//                 query.push_str("archived=true")
//             } else {
//                 query.push_str("archived=false")
//             }
//         });
//
//         self.visibility.map(|visibility| {
//             query.push_str(split_char);
//             split_char = &amp_char;
//
//             query.push_str("visibility=");
//             query.push_str(match visibility {
//                 ::ListingVisibility::Public => "public",
//                 ::ListingVisibility::Internal => "internal",
//                 ::ListingVisibility::Private => "private",
//             });
//         });
//
//         self.order_by.map(|order_by| {
//             query.push_str(split_char);
//             split_char = &amp_char;
//
//             query.push_str("order_by=");
//             query.push_str(match order_by {
//                 ListingOrderBy::Id => "id",
//                 ListingOrderBy::Name => "name",
//                 ListingOrderBy::Path => "path",
//                 ListingOrderBy::CreatedAt => "created_at",
//                 ListingOrderBy::UpdatedAt => "updated_at",
//                 ListingOrderBy::LastActivityAt => "last_activity_at",
//             });
//         });
//
//         self.sort.map(|sort| {
//             query.push_str(split_char);
//             split_char = &amp_char;
//
//             query.push_str("sort=");
//             query.push_str(match sort {
//                 ::ListingSort::Asc => "asc",
//                 ::ListingSort::Desc => "desc",
//             });
//         });
//
//         if !self.search.is_empty() {
//             query.push_str(split_char);
//             // split_char = &amp_char;
//
//             query.push_str("search=");
//             query.push_str(&self.search);
//         }
//
//         query
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    // #[test]
    // fn build_query_default() {
    //     let expected_string = "projects/owned";
    //     let listing: Listing = Default::default();
    //     let query = listing.build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned";
    //     let listing = Listing::new();
    //     let query = listing.build_query();
    //     assert_eq!(query, expected_string);
    // }
    //
    //
    // #[test]
    // fn build_query_archived() {
    //     let expected_string = "projects/owned?archived=true";
    //     let query = Listing::new().archived(true).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?archived=false";
    //     let query = Listing::new().archived(false).build_query();
    //     assert_eq!(query, expected_string);
    // }
    //
    //
    // #[test]
    // fn build_query_visibility() {
    //     let expected_string = "projects/owned?visibility=public";
    //     let query = Listing::new().visibility(::ListingVisibility::Public).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?visibility=internal";
    //     let query = Listing::new().visibility(::ListingVisibility::Internal).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?visibility=private";
    //     let query = Listing::new().visibility(::ListingVisibility::Private).build_query();
    //     assert_eq!(query, expected_string);
    // }
    //
    //
    // #[test]
    // fn build_query_order_by() {
    //     let expected_string = "projects/owned?order_by=id";
    //     let query = Listing::new().order_by(ListingOrderBy::Id).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?order_by=name";
    //     let query = Listing::new().order_by(ListingOrderBy::Name).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?order_by=path";
    //     let query = Listing::new().order_by(ListingOrderBy::Path).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?order_by=created_at";
    //     let query = Listing::new().order_by(ListingOrderBy::CreatedAt).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?order_by=updated_at";
    //     let query = Listing::new().order_by(ListingOrderBy::UpdatedAt).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?order_by=last_activity_at";
    //     let query = Listing::new().order_by(ListingOrderBy::LastActivityAt).build_query();
    //     assert_eq!(query, expected_string);
    // }
    //
    //
    // #[test]
    // fn build_query_sort() {
    //     let expected_string = "projects/owned?sort=asc";
    //     let query = Listing::new().sort(::ListingSort::Asc).build_query();
    //     assert_eq!(query, expected_string);
    //
    //     let expected_string = "projects/owned?sort=desc";
    //     let query = Listing::new().sort(::ListingSort::Desc).build_query();
    //     assert_eq!(query, expected_string);
    // }
    //
    //
    // #[test]
    // fn build_query_search() {
    //     let expected_string = "projects/owned?search=SearchPattern";
    //     let query = Listing::new().search(String::from("SearchPattern")).build_query();
    //     assert_eq!(query, expected_string);
    // }
    //
    //
    // #[test]
    // fn groups_build_query_multiple() {
    //     let expected_string = "projects/owned?archived=false&sort=asc";
    //     let query = Listing::new().archived(false).sort(::ListingSort::Asc).build_query();
    //     assert_eq!(query, expected_string);
    // }
}
