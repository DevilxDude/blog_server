use axum::{extract, response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    id: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlog {
    title: String,
    content: String,
}

impl Blog {
    fn new(new_blog: NewBlog) -> Self {
        Blog {
            id: "100".to_string(),
            title: new_blog.title,
            content: new_blog.content,
        }
    }
}

// Use this for better error macros debug info
// #[axum_macros::debug_handler]
pub async fn create_blog(extract::Json(payload): extract::Json<NewBlog>) -> response::Json<Blog> {
    response::Json(Blog::new(payload))
}

pub async fn get_blogs() -> response::Json<Vec<Blog>> {
    response::Json(vec![Blog {
        id: "1".to_string(),
        title: "Title".to_string(),
        content: "Content".to_string(),
    }])
}
