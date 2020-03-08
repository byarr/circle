use circ::git::RepoInfo;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Circ {
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    cmd: Command
}

#[derive(StructOpt)]
enum Command {
    Pipelines(Pipelines),
    Runs(WorkflowRuns),
    Workflow(Workflow)
}

#[derive(StructOpt)]
struct Pipelines {

}

#[derive(StructOpt)]
struct WorkflowRuns {
    #[structopt(short,long, help = "Workflow name")]
    name: Option<String>
}


#[derive(StructOpt)]
struct Workflow {
    #[structopt(short,long)]
    id: String
}

fn main() {

    let circ = Circ::from_args();

    let info = RepoInfo::from_path(".").unwrap();
    let slug = info.slug().unwrap();
    let config = circ::load_config().unwrap();
    let client = api::v2::Client::new(config.token).unwrap();

    match circ.cmd {
        Command::Pipelines(_) => {
            let pipelines = client.get_pipelines_mine(&slug, None).unwrap();
            pipelines.items.iter().for_each(|p| println!("{:?}", p));
        },
        Command::Runs(wf) => {
            let wf_name = wf.name.unwrap_or("workflow".to_string());
            let runs = client.get_recent_workflow_runs(&slug, &wf_name, None).unwrap();
            runs.items.iter().for_each(|r| println!("{}\t{}\t{}", r.status, r.created_at, r.url()));
        },
        Command::Workflow(wf) => {
            let wf = client.get_workflow(&wf.id).unwrap();
            println!("{:?}", wf);
        }

    }
    // let info = RepoInfo::from_path(".").unwrap();
    // let slug = info.slug().unwrap();
    //
    // println!("{:?}", info);
    //
    // let config = circ::load_config().unwrap();
    //
    // let client = api::v2::Client::new(config.token).unwrap();
    //
    // println!("Project");
    // let project = client.get_project(& slug).unwrap();
    // println!("{:?}", project);
    // println!("=======");
    //
    // println!("Pipelines");
    // let pipelines = client.get_pipelines_mine(&slug, None).unwrap();
    // pipelines.items.iter().for_each(|p| println!("{:?}", p));
    // println!("=======");
    //
    // let wf_runs = client
    //     .get_recent_workflow_runs(&slug, "workflow", None)
    //     .unwrap();
    // wf_runs.items.iter().for_each(|r| println!("{:?}", r));
    // println!("=======");
}
