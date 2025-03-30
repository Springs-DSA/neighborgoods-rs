#![allow(clippy::all, warnings)]
pub struct GetUser;
pub mod get_user {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetUser";
    pub const QUERY : & str = "query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n  }\n}\n\n# query GetUsers {\n#   users {\n#     id\n#     name\n#   }\n# }" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub user: Option<GetUserUser>,
    }
    #[derive(Deserialize)]
    pub struct GetUserUser {
        pub id: Int,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for GetUser {
    type Variables = get_user::Variables;
    type ResponseData = get_user::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_user::QUERY,
            operation_name: get_user::OPERATION_NAME,
        }
    }
}
