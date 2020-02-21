use serde::{Serialize, Deserialize};


const BASE_PATH : &str = "https://circleci.com/api/v2";

const API_KEY_HEADER: &str = "Circle-Token";

#[derive(Debug, Serialize, Deserialize)]
pub struct Project<'a> {
    slug: &'a str,
    name: &'a str,
    organization_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline<'a> {
    id: &'a str,
    project_slug: &'a str,
    updated_at: Option<&'a str>,
    number: i64,
    state: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineList<'a> {
    items: Vec<Pipeline<'a>>,
    next_page_token: &'a str
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
