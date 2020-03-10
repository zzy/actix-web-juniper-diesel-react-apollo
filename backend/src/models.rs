use super::schema::{users, projects};
use juniper::GraphQLInputObject;
use chrono::*;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub passord: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// applying #[derive(juniper::GraphQLObject)] to the User struct above
#[juniper::object]
impl User {
    #[graphql(name = "id")]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "email")]
    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    #[graphql(name = "username")]
    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    #[graphql(name = "created_at")]
    fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }

    #[graphql(name = "updated_at")]
    fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }
}

// Used to create new user
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

// The GraphQL input object for creating user
#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub username: String,
    pub password: String,
}

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub subject: String,
    pub website: String,
    pub source_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published: bool,
}

// applying #[derive(juniper::GraphQLObject)] to the project struct above
#[juniper::object]
impl Project {
    #[graphql(name = "id")]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "user_id")]
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    #[graphql(name = "subject")]
    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    #[graphql(name = "website")]
    pub fn website(&self) -> &str {
        self.website.as_str()
    }

    #[graphql(name = "source_code")]
    pub fn source_code(&self) -> &str {
        self.source_code.as_str()
    }

    #[graphql(name = "created_at")]
    fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }

    #[graphql(name = "updated_at")]
    fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }

    #[graphql(name = "published")]
    fn published(&self) -> bool {
        self.published
    }
}

// // Used to create new project
// #[derive(Insertable)]
// #[table_name = "projects"]
// pub struct NewProject<'a> {
//     pub user_id: &'a str,
//     pub subject: &'a str,
//     pub website: &'a str,
// }

// // The GraphQL input object for creating project
// #[derive(GraphQLInputObject)]
// pub struct CreateProjectInput {
//     pub email: String,
//     pub username: String,
//     pub password: String,
// }
