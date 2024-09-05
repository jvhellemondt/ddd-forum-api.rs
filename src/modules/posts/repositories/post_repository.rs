use async_trait::async_trait;
use crate::modules::posts::domain::post::Post;
use crate::modules::posts::errors::PostsModuleErrors;

#[async_trait]
pub trait PostRepository {
    async fn list_posts(&self, sort: Option<String>) -> Result<Vec<Post>, PostsModuleErrors>;
}
