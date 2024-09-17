use crate::modules::posts::domain::post::Post;
use crate::modules::posts::errors::PostsModuleErrors;
use crate::modules::posts::repositories::implementations::postgres_post_repository::PostgresPostRepository;
use crate::modules::posts::repositories::post_repository::PostRepository;
use crate::modules::posts::use_cases::list_posts::view::ListPostsParams;

pub async fn execute(
    params: ListPostsParams,
) -> Result<Vec<Post>, PostsModuleErrors> {
    let repository = PostgresPostRepository::new();
    repository.list_posts(params.sort).await
}
