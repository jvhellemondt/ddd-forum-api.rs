use crate::modules::posts::domain::post::Post;
use crate::modules::posts::errors::PostsModuleErrors;
use crate::modules::posts::use_cases::list_posts;
use crate::modules::posts::use_cases::list_posts::view::ListPostsParams;

pub async fn handle(
    params: ListPostsParams
) -> Result<Vec<Post>, PostsModuleErrors> {
    list_posts::model::execute(params).await
}
