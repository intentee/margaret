use margaret_http::response::Response;
use margaret_macros::interceptor;
use margaret_macros::process;
use margaret_macros::singleton;

use crate::margaret::routes::Routes;
use crate::views::view::View;

#[singleton]
#[interceptor]
pub struct ViewInterceptor;

impl ViewInterceptor {
    #[process]
    pub async fn process(&self, view: Box<dyn View>, routes: &Routes) -> Response {
        let home = routes.public.get_greeting.url();

        Response::html(
            200,
            format!(
                "<main>{}</main><nav><a href=\"{home}\">home</a></nav>",
                view.body()
            ),
        )
    }
}
