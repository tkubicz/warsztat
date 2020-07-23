use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    #[serde(rename = "postId")]
    pub post_id: i32,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Debug, Deserialize)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[derive(Debug, Deserialize)]
pub struct Company {
    pub name: String,
    #[serde(rename = "catchPhrase")]
    pub catch_phrase: String,
    pub bs: String,
}

#[derive(Debug, Deserialize)]
pub struct Todo {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
}
