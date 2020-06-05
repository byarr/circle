use circ::git::RepoInfo;
use structopt::StructOpt;
use futures::executor::block_on;

#[derive(StructOpt)]
#[structopt(name = "circ", about = "Circleci info.")]
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

fn main() {
    let circ = Circ::from_args();

    let info = RepoInfo::from_path(std::env::current_dir().unwrap()).unwrap();
    let slug = info.slug();
    let config = circ::load_config().unwrap();
    let client = api::v2::Client::new(config.token).unwrap();

    match circ.cmd {
        Command::Pipelines(_) => {
            let pipelines = block_on(client.get_pipelines_mine(&slug.unwrap(), None)).unwrap();
            match info.branch {
                None => pipelines.items.iter().for_each(|p| println!("{:?}", p)),
                Some(current_branch) => pipelines.items.iter()
                    .filter(|&p| p.vcs.branch.as_ref().map(|b| b.eq(&current_branch)).unwrap_or(false) )
                    .for_each(|p| println!("{:?}", p))

            }

        }
        Command::Runs(wf) => {
            let wf_name = wf.name.unwrap_or("workflow".to_string());
            let runs = block_on(client
                .get_recent_workflow_runs(&slug.unwrap(), &wf_name, wf.branch.as_deref()))
                .unwrap();
            runs.items
                .iter()
                .take(1)
                .for_each(|r| println!("{}\t{}\t{}", r.status, r.created_at, r.url()));
        }
        Command::Workflow(wf) => {
            let wf = block_on(client.get_workflow(&wf.id)).unwrap();
            println!("{:?}", wf);
        }
        Command::JobDetail(jd) => {
            let details = block_on(client.get_job_detail(&slug.unwrap(), &jd.number)).unwrap();
            println!("{:?}", details);
        }
    }
}
