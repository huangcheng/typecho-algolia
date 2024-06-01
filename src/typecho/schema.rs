#[derive(Debug, sqlx::FromRow)]
pub struct Post {
    pub cid: u32,
    pub title: String,
    pub slug: String,
    pub text: String,
    pub created: u32,
    pub modified: u32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GenericName {
    pub name: String,
}
