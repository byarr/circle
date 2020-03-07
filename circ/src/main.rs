use std::env::args;

fn main() {
    let url = circ::origin_url(".");
    println!("{:?}", url);

    println!("Hello, world!");
    let a: Vec<String> = args().collect();
    println!("{:?}", a);

    let config = circ::load_config().unwrap();

    let client = api::v2::Client::new(config.token).unwrap();

    println!("Project");
    let slug = "gh/byarr/dug";
    let project = client.get_project(slug).unwrap();
    println!("{:?}", project);
    println!("=======");

    println!("Pipelines");
    let pipelines = client.get_pipelines_mine(slug, None).unwrap();
    pipelines.items.iter().for_each(|p| println!("{:?}", p));
    println!("=======");

    let wf_runs = client
        .get_recent_workflow_runs(slug, "workflow", None)
        .unwrap();
    wf_runs.items.iter().for_each(|r| println!("{:?}", r));
    println!("=======");
}
