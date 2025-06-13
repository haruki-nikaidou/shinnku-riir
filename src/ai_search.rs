use compact_str::CompactString;
use kanau::processor::Processor;
use reqwest::Url;

#[derive(Debug)]
pub struct AiSearchProcessor {
    pub ai_search_root: Url,
}

impl AiSearchProcessor {
    pub fn new(ai_search_root: Url) -> Self {
        Self {
            ai_search_root,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AiResponse {
    pub ans: Box<[CompactString]>
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SearchRequest {
    pub q: CompactString,
}

pub struct AiSearchResponse {
    pub names: Box<[CompactString]>,
    pub intro: String
}

impl Processor<SearchRequest, reqwest::Result<AiSearchResponse>> for AiSearchProcessor {
    async fn process(&self, request: SearchRequest) -> reqwest::Result<AiSearchResponse> {
        let find_name_url = self
            .ai_search_root
            .join("findname")
            .unwrap()
            .join(&format!("?name={}", request.q))
            .unwrap();
        
        let intro_url = self
            .ai_search_root
            .join("intro")
            .unwrap()
            .join(&format!("?name={}", request.q))
            .unwrap();

        let find_name_response = reqwest::get(find_name_url).await?.json::<AiResponse>().await?;
        let intro_response = reqwest::get(intro_url).await?.text().await?;

        Ok(AiSearchResponse {
            names: find_name_response.ans,
            intro: intro_response,
        })
    }
}