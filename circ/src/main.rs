use circ::git::RepoInfo;

fn main() {
    let info = RepoInfo::from_path(".").unwrap();
    let slug = info.slug().unwrap();

    println!("{:?}", info);

    let config = circ::load_config().unwrap();

    let client = api::v2::Client::new(config.token).unwrap();

    println!("Project");
    let project = client.get_project(& slug).unwrap();
    println!("{:?}", project);
    println!("=======");

    println!("Pipelines");
    let pipelines = client.get_pipelines_mine(&slug, None).unwrap();
    pipelines.items.iter().for_each(|p| println!("{:?}", p));
    println!("=======");

    let wf_runs = client
        .get_recent_workflow_runs(&slug, "workflow", None)
        .unwrap();
    wf_runs.items.iter().for_each(|r| println!("{:?}", r));
    println!("=======");
}
