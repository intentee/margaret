use margaret::framework::macros::acts_as_oauth_client;

#[acts_as_oauth_client(blog, admitted_as = blog_app)]
pub struct BlogClient;
