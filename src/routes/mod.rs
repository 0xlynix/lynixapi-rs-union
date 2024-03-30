use actix_web::dev::HttpServiceFactory;

pub mod blog;
pub mod boop;

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::blog_get_all_articles,
    )
}


pub fn boop() -> impl HttpServiceFactory {
    (
        boop::get_all_boops,
        boop::get_boop_by_event,
        boop::count_boops,
        boop::count_boops_by_event,
        boop::boop_test,
        boop::boop
    )
}