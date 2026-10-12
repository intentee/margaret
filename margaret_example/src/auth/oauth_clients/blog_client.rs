use margaret::framework::macros::oauth_client;

#[oauth_client(blog, admitted_as = blog_app)]
pub struct BlogClient;
