use circle::git::RepoInfo;
use structopt::StructOpt;
use tokio::prelude::*;

#[derive(StructOpt)]
#[structopt(name = "circle", about = "Circleci info.")]
struct Circ {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Pipelines(Pipelines),
    Runs(WorkflowRuns),
    Workflow(Workflow),
    JobDetail(JobDetail),
}

#[derive(StructOpt)]
#[structopt(about = "Gets a list of your pipeline runs")]
struct Pipelines {}

#[derive(StructOpt)]
#[structopt(about = "runs stuff")]
struct WorkflowRuns {
    #[structopt(short, long, help = "Workflow name")]
    name: Option<String>,
    #[structopt(short, long, help = "branch name")]
    branch: Option<String>
}

#[derive(StructOpt)]
#[structopt(about = "get workflow info")]
struct Workflow {
    #[structopt(short, long)]
    id: String,
}

#[derive(StructOpt)]
#[structopt(about = "get job details")]
struct JobDetail {
    #[structopt(short, long)]
    number: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circ = Circ::from_args();

    let info = RepoInfo::from_path(std::env::current_dir().unwrap()).unwrap();
    let slug = info.slug();
    let config = circle::load_config().unwrap();
    let client = api::v2::Client::new(config.token).unwrap();

    match circ.cmd {
        Command::Pipelines(_) => {
            let pipelines = client.get_pipelines_mine(&slug.unwrap(), None).await.unwrap();
            match info.branch {
                None => pipelines.items.iter().for_each(|p| println!("{:?}", p)),
                Some(current_branch) => pipelines.items.iter()
                    .filter(|&p| p.vcs.branch.as_ref().map(|b| b.eq(&current_branch)).unwrap_or(false) )
                    .for_each(|p| println!("{:?}", p))

            }

        }
        Command::Runs(wf) => {
            let wf_name = wf.name.unwrap_or("workflow".to_string());
            let runs = client
                .get_recent_workflow_runs(&slug.unwrap(), &wf_name, wf.branch.as_deref()).await
                .unwrap();
            runs.items
                .iter()
                .take(1)
                .for_each(|r| println!("{}\t{}\t{}", r.status, r.created_at, r.url()));
        }
        Command::Workflow(wf) => {
            let wf = client.get_workflow(&wf.id).await.unwrap();
            println!("{:?}", wf);
        }
        Command::JobDetail(jd) => {
            let details = client.get_job_detail(&slug.unwrap(), &jd.number).await.unwrap();
            println!("{:?}", details);
        }
    }
    Ok(())
}
