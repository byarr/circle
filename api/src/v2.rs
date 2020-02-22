use serde::{Serialize, Deserialize};
use serde;

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
    pub items: Vec<Pipeline>,
    next_page_token: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRun {
    id: String,
    duration: i64,
    created_at: String,
    stopped_at: String,
    status: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentWorkflowRun {
    pub items: Vec<WorkflowRun>
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

    pub fn get_pipelines_mine(&self, slug: &str, page_token: Option<&str>) -> Result<PipelineList, reqwest::Error> {
        //GET /project/{project-slug}/pipeline/mine

        let url = format!("{}/project/{}/pipeline/mine", BASE_PATH, slug);
        let mut query = self.client.get(&url)
            .header(API_KEY_HEADER, &self.api_key);
        query = match page_token {
            Some(token) => query.query(&[("page_token", token)]),
            None => query
        };

        let resp = query.send()?;
        let resp = resp.error_for_status()?;
        resp.json()
    }

    pub fn get_recent_workflow_runs(&self, slug: &str, workflow_name: &str, branch: Option<&str>) -> Result<RecentWorkflowRun, reqwest::Error> {
        //GET /insights/{project-slug}/workflows/{workflow-name}

        let url = format!("{}/insights/{}/workflows/{}", BASE_PATH, slug, workflow_name);
        let mut query = self.client.get(&url)
            .header(API_KEY_HEADER, &self.api_key);
        query = match branch {
            Some(b) => query.query(&[("branch", b)]),
            None => query
        };

        let resp = query.send()?;
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
