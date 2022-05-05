use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}
#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    RequestResponseToString(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error),
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::RequestResponseToString(e))?;

    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParseFailed(e))?;
    Ok(articles)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
