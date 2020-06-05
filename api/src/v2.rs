use serde;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const BASE_PATH: &str = "https://circleci.com/api/v2";

const API_KEY_HEADER: &str = "Circle-Token";

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    slug: String,
    name: String,
    organization_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub project_slug: String,
    pub updated_at: Option<String>,
    pub number: i64,
    pub state: String,
    pub vcs: Vcs,
    pub trigger: Trigger,
    #[serde(flatten)]
    pub extras: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "type")]
    trigger_type: String,
    received_at: String,
    actor: Actor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    login: String,
    avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vcs {
    pub commit: Option<Commit>,
    pub branch: Option<String>,
    #[serde(flatten)]
    extras: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    subject: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineList {
    pub items: Vec<Pipeline>,
    next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: String,
    pub duration: i64,
    pub created_at: String,
    pub stopped_at: String,
    pub status: String,
}

impl WorkflowRun {
    pub fn url(&self) -> String {
        format!("https://circleci.com/workflow-run/{}", self.id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentWorkflowRun {
    pub items: Vec<WorkflowRun>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub stopped_at: String,
    pub status: String,
    pub pipeline_id: String,
    pub pipeline_number: i64,
    pub project_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobDetail {
    web_url: String,
    name: String,
    #[serde(flatten)]
    extras: HashMap<String, Value>,
}

pub struct Client {
    api_key: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build()?;
        Ok(Client { api_key, client })
    }

    pub async fn get_project(&self, slug: &str) -> Result<Project, reqwest::Error> {
        //GET /project/{project-slug}
        self.get(format!("/project/{}", slug), None).await
    }

    async fn get<T>(&self, path: String, q: Option<Vec<(&str, &str)>>) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", BASE_PATH, path);
        let mut query = self.client.get(&url).header(API_KEY_HEADER, &self.api_key);
        query = match q {
            Some(params) => query.query(&params),
            None => query,
        };

        let resp = query.send().await?;
        let resp = resp.error_for_status()?;
        resp.json().await
    }

    pub async fn get_pipelines_mine(
        &self,
        slug: &str,
        page_token: Option<&str>,
    ) -> Result<PipelineList, reqwest::Error> {
        //GET /project/{project-slug}/pipeline/mine

        let params = page_token.map(|token| vec![("page_token", token)]);

        self.get(format!("/project/{}/pipeline/mine", slug), params).await
    }

    pub async fn get_recent_workflow_runs(
        &self,
        slug: &str,
        workflow_name: &str,
        branch: Option<&str>,
    ) -> Result<RecentWorkflowRun, reqwest::Error> {
        //GET /insights/{project-slug}/workflows/{workflow-name}

        let params = branch.map(|b| vec![("branch", b)]);

        self.get(
            format!("/insights/{}/workflows/{}", slug, workflow_name),
            params,
        ).await
    }

    pub async fn get_workflow(&self, id: &str) -> Result<Workflow, reqwest::Error> {
        //GET /workflow/{id}
        self.get(format!("/workflow/{}", id), None).await
    }

    pub async fn get_job_detail(&self, slug: &str, number: &str) -> Result<JobDetail, reqwest::Error> {
        //GET /project/{project-slug}/job/{job-number}
        self.get(format!("/project/{}/job/{}", slug, number), None).await
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
        let contents =
            std::fs::read_to_string("./testdata/v2/pipeline_list_response.json").unwrap();
        let pipeline: PipelineList = serde_json::from_str(&contents).unwrap();
        assert_eq!(pipeline.items.len(), 1);
    }
    #[test]
    fn parse_workflow() {
        let contents = std::fs::read_to_string("./testdata/v2/workflow.json").unwrap();
        let wf: Workflow = serde_json::from_str(&contents).unwrap();
        assert_eq!(0, wf.pipeline_number);
    }

    #[test]
    fn parse_job_details() {
        let contents = std::fs::read_to_string("./testdata/v2/job_details.json").unwrap();
        let jb: JobDetail = serde_json::from_str(&contents).unwrap();
        assert_eq!("string", jb.web_url);
    }
}
