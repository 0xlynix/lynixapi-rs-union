use actix_web::dev::HttpServiceFactory;

pub mod blog;

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::blog_get_articles,
        blog::blog_get_article_by_id,
        blog::blog_create_article,
        blog::blog_update_article,
        blog::blog_delete_article,
    )
}