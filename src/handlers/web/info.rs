use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "about.html")]
pub struct AboutPage {}

pub async fn get_about_page() -> AboutPage {
    AboutPage {}
}
