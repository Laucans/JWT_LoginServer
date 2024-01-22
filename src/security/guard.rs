use actix_web::{guard::Guard, http::header};
// /!\ Unused code.
// An example of Guard implementation to check the content type.
pub struct ContentTypeGuard;

impl Guard for ContentTypeGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        ctx.header::<header::ContentType>();
        todo!()
    }
}
