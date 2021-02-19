use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ios-deployer", about = "Helpers for ios app development.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Device {
        #[structopt(short, long)]
        list: bool,
    },

    Test {
        #[structopt(short, long)]
        device: Option<String>,
        // #[structopt(short, parse(from_occurrences))]
        // args: Option<String>,
    },

    Bench {
        #[structopt(short, long)]
        device: Option<String>,
    },

    /// run binary on ios device.
    Run {
        /// device udid which to run on.
        #[structopt(short, long)]
        device: Option<String>,
        /// Path of binary which to run.
        /// Output of cargo build will be used, if not given.
        #[structopt(parse(from_os_str))]
        bin: Option<PathBuf>,
    },

    Criterion {
        #[structopt(short, long)]
        device: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
