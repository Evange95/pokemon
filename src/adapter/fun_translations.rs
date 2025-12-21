use crate::{
    domain::{Description, TranslationErrors, TranslationStyle},
    ports::Translator,
};

pub struct FunTranslationsAdapter {
    client: reqwest::Client,
    base_url: String,
}

impl FunTranslationsAdapter {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
struct FunTranslationsResponse {
    contents: Contents,
}

#[derive(serde::Deserialize, Debug)]
struct Contents {
    translated: String,
}

#[async_trait::async_trait]
impl Translator for FunTranslationsAdapter {
    async fn translate(
        &self,
        description: Description,
        style: TranslationStyle,
    ) -> Result<Description, TranslationErrors> {
        let endpoint = match style {
            TranslationStyle::Yoda => "yoda",
            TranslationStyle::Shakespeare => "shakespeare",
        };

        let url = format!("{}/translate/{}", self.base_url, endpoint);

        let response = self
            .client
            .post(&url)
            .form(&[("text", description.as_str())])
            .send()
            .await
            .map_err(|_| TranslationErrors::TranslationUnavailable)?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let translation_response: FunTranslationsResponse = response
                    .json()
                    .await
                    .map_err(|e| TranslationErrors::InvalidTranslationData(e.to_string()))?;

                Ok(translation_response.contents.translated)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(TranslationErrors::TranslationRateLimited)
            }
            _ => Err(TranslationErrors::TranslationUnavailable),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn test_translate_yoda_success() {
        let mock_server = MockServer::start().await;

        let response_body = r#"{
            "success": {
                "total": 1
            },
            "contents": {
                "translated": "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.",
                "text": "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.",
                "translation": "yoda"
            }
        }"#;

        Mock::given(method("POST"))
            .and(path("/translate/yoda"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let adapter = FunTranslationsAdapter::new(mock_server.uri());
        let result = adapter
            .translate(
                "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
                TranslationStyle::Yoda,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was."
        );
    }

    #[tokio::test]
    async fn test_translate_shakespeare_success() {
        let mock_server = MockServer::start().await;

        let response_body = r#"{
            "success": {
                "total": 1
            },
            "contents": {
                "translated": "'twas did create by a scientist after years of horrific gene splicing and dna engineering experiments.",
                "text": "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.",
                "translation": "shakespeare"
            }
        }"#;

        Mock::given(method("POST"))
            .and(path("/translate/shakespeare"))
            .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
            .mount(&mock_server)
            .await;

        let adapter = FunTranslationsAdapter::new(mock_server.uri());
        let result = adapter
            .translate(
                "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".to_string(),
                TranslationStyle::Shakespeare,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "'twas did create by a scientist after years of horrific gene splicing and dna engineering experiments."
        );
    }

    #[tokio::test]
    async fn test_translate_returns_rate_limited_on_429() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/translate/yoda"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&mock_server)
            .await;

        let adapter = FunTranslationsAdapter::new(mock_server.uri());
        let result = adapter
            .translate("Test description".to_string(), TranslationStyle::Yoda)
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TranslationErrors::TranslationRateLimited
        );
    }

    #[tokio::test]
    async fn test_translate_returns_unavailable_on_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/translate/shakespeare"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&mock_server)
            .await;

        let adapter = FunTranslationsAdapter::new(mock_server.uri());
        let result = adapter
            .translate(
                "Test description".to_string(),
                TranslationStyle::Shakespeare,
            )
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TranslationErrors::TranslationUnavailable
        );
    }

    #[tokio::test]
    async fn test_translate_returns_invalid_data_on_parse_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/translate/yoda"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let adapter = FunTranslationsAdapter::new(mock_server.uri());
        let result = adapter
            .translate("Test description".to_string(), TranslationStyle::Yoda)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            TranslationErrors::InvalidTranslationData(_) => (),
            _ => panic!("Expected InvalidTranslationData error"),
        }
    }
}
