use std::path::PathBuf;
use structopt::StructOpt;

mod config;
mod deploy;
mod model;
mod printer;
mod s3;

#[derive(Debug, StructOpt)]
#[structopt(name = "webapp-deployer", about = "Deploy webapp to s3")]
struct Opt {
    #[structopt(parse(from_os_str))]
    config_file: PathBuf,

    #[structopt(parse(from_os_str))]
    app_dir: PathBuf,

    bucket_name: String,

    #[structopt(short, long)]
    dry_run: bool,
}

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    deploy::run(
        &printer::Printer::new(),
        &opt.app_dir,
        &model::BucketName(opt.bucket_name.to_owned()),
        &opt.config_file,
        opt.dry_run,
    )
}
