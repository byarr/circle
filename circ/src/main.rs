use std::env::args;

fn main() {
    println!("Hello, world!");
    let a: Vec<String> = args().collect();
    println!("{:?}", a);


    let client = api::v2::Client::new(a.get(1).unwrap().to_string()).unwrap();

    let project = client.get_project("gh/byarr/dug").unwrap();
    println!("{:?}", project);

    let pipelines = client.get_pipelines_mine("gh/byarr/dug", None).unwrap();
    pipelines.items.iter().for_each(|p| println!("{:?}", p));




}
