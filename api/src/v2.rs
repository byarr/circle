use serde::{Serialize, Deserialize};


const BASE_PATH : &str = "https://circleci.com/api/v2";

const API_KEY_HEADER: &str = "Circle-Token";

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    slug: String,
    name: String,
    organization_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    id: String,
    project_slug: String,
    updated_at: Option<String>,
    number: i64,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineList<> {
    items: Vec<Pipeline>,
    next_page_token: String
}

pub struct Client {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl Client {

    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder().build()?;
        Ok(Client {
            api_key, client
        })
    }

    pub fn get_project(&self, slug: &str) -> Result<Project, reqwest::Error> {
        //GET /project/{project-slug}
        let url = format!("{}/project/{}", BASE_PATH, slug);

        let resp = self.client.get(&url)
            .header(API_KEY_HEADER, &self.api_key)
            .send()?;
        let resp = resp.error_for_status()?;
        resp.json()

    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_project() {
        let contents = std::fs::read_to_string("./testdata/v2/project.json").unwrap();
        let project: Project = serde_json::from_str(&contents).unwrap();
        assert_eq!(project.name, "api-preview-docs");
    }

    #[test]
    fn parse_pipeline() {
        let contents = std::fs::read_to_string("./testdata/v2/pipeline.json").unwrap();
        let pipeline: Pipeline = serde_json::from_str(&contents).unwrap();
        assert_eq!(pipeline.number, 0);
    }

    #[test]
    fn parse_pipeline_list() {
        let contents = std::fs::read_to_string("./testdata/v2/pipeline_list_response.json").unwrap();
        let pipeline: PipelineList = serde_json::from_str(&contents).unwrap();
        assert_eq!(pipeline.items.len(), 1);
    }
}
