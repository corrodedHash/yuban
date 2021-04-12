mod authentication;
mod modify;
mod query;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LangCode(pub String);

impl<'r> rocket::request::FromParam<'r> for LangCode {
    type Error = ();

    fn from_param(param: &'r rocket::http::RawStr) -> Result<Self, Self::Error> {
        if param.is_empty() {
            return Err(());
        }
        if param.len() != 2 {
            return Err(());
        }
        Ok(Self(param.as_str().to_owned()))
    }
}

pub fn get_api_routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![
        authentication::login_post,
        authentication::logout_post,
        authentication::test_token,
        query::list_groups,
        query::list_threads,
        query::list_posts,
        query::single_post,
        modify::new_thread,
        modify::new_translation,
        modify::new_correction,
        modify::delete_thread,
        modify::delete_post,
        modify::delete_correction,
    ]
}
