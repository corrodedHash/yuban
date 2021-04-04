mod fix;
pub use fix::get_fix_routes;
mod api;
pub use api::get_api_routes;
pub use api::LangCode;
mod admin;
pub use admin::get_admin_routes;
use rocket::http::Status;

#[derive(Debug, Clone)]
struct InternalDebugFailure(Status);

impl From<Status> for InternalDebugFailure {
    fn from(s: Status) -> Self {
        Self(s)
    }
}

impl From<()> for InternalDebugFailure {
    fn from(_: ()) -> Self {
        Self(Status::InternalServerError)
    }
}

impl From<mysql::Error> for InternalDebugFailure {
    fn from(err: mysql::Error) -> Self {
        dbg!(err);
        Self(Status::InternalServerError)
    }
}

impl From<serde_json::Error> for InternalDebugFailure {
    fn from(err: serde_json::Error) -> Self {
        dbg!(err);
        Self(Status::InternalServerError)
    }
}

impl<'r> rocket::response::Responder<'r> for InternalDebugFailure {
    fn respond_to(self, request: &rocket::Request) -> rocket::response::Result<'r> {
        self.0.respond_to(request)
    }
}
