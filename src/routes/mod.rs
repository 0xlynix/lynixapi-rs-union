use actix_web::dev::HttpServiceFactory;

pub mod blog;
pub mod boop;

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::blog_get_articles,
        blog::blog_get_article_by_id,
        blog::blog_create_article,
        blog::blog_update_article,
        blog::blog_delete_article,
    )
}

pub fn boop() -> impl HttpServiceFactory {
    (
        boop::get_all_boops,
        boop::get_boop_by_event,
        boop::boop,
    )
}