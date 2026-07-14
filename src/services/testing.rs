use sqlx::SqlitePool;

use crate::{model::Url, services::urls::add_url};

pub fn template_urls() -> [Url; 6] {
    [Url::with_default_metadata(
                "en.wikipedia.org/wiki/Hatsune_Miku".to_string(),
                "Hatsune Miku".to_string(),
                "Hatsune Miku V6 / April 14, 2026".to_string(),
                "<Insert content about Miku, hit vocaloid, Here>".to_string(),
    ),
        Url::with_default_metadata(
                "en.wikipedia.org/wiki/Kasane_Teto".to_string(),
                 "Kasane Teto".to_string(),
                 "".to_string(),
                "<Not a vocaloid>".to_string(),
    ),
        Url::with_default_metadata(
                "en.wikipedia.org/wiki/Pieris_rapae".to_string(),
                 "Pieris rapae".to_string(),
                "Conservation status".to_string(),
                "<Insert content about Pieris Rapae Here>".to_string(),
    ),
        Url::with_default_metadata(
                "en.wikipedia.org/wiki/Kagamine_Rin/Len".to_string(),
                 "Kagamine Rin/Len".to_string(),
                "From Wikipedia, the free encyclopedia".to_string(),
                "<My fav duo>".to_string(),
    ),
        Url::with_default_metadata(
                "doc.rust-lang.org/std/primitive.str.html".to_string(),
                 "str - Rust".to_string(),
                "String slices.".to_string(),
                "I'm a string!".to_string(),
    ),
        Url::with_default_metadata(
                "www.codeblocks.org/".to_string(),
                 "www.codeblocks.org/".to_string(),
                "The IDE with all the features you need, having a consistent look, feel and operation across platforms.".to_string(),
                "C++, C, Fortran stuff".to_string(),
    ),
]
}

/// Add six template urls for testing purposes
/// The first is Hatsune Miku's wikipedia page
/// The second is the Teto's wikipedia page
/// The third is the wikipedia page for Pieris Rapae
/// The fourth is the page for Rin and Len also wikipedia
/// The Fifth is the rust strs
/// the sixth is code blocks, an ide
/// For testing purposes, the content isn't real
pub async fn setup_template_urls(pool: &SqlitePool) -> Vec<Url> {
    let template_urls = template_urls();
    let mut res: Vec<Url> = Vec::with_capacity(template_urls.len());
    for url in template_urls {
        res.push(add_url(url.url, url.title, url.description, url.content, pool)
            .await
            .unwrap())
    }
    res
}
