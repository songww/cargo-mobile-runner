use std::path::PathBuf;
use structopt::StructOpt;

use ios_deployer::mobile_device::AMDevice;

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
        /// Lists all connected devices.
        #[structopt(short, long)]
        list: bool,
    },

    Test {
        /// udid of device which to run on.
        #[structopt(short, long)]
        device: Option<String>,
        // #[structopt(short, parse(from_occurrences))]
        // args: Option<String>,
    },

    Bench {
        /// udid of device which to run on.
        #[structopt(short, long)]
        device: Option<String>,
    },

    /// run binary on ios device.
    Run {
        /// udid of device which to run on.
        #[structopt(short, long)]
        device: Option<String>,
        /// Path of binary which to run.
        /// Output of cargo build will be used, if not given.
        #[structopt(parse(from_os_str))]
        bin: Option<PathBuf>,
    },

    Criterion {
        /// udid of device which to run on.
        #[structopt(short, long)]
        device: Option<String>,
    },

    Miri {
        /// udid of device which to run on.
        #[structopt(short, long)]
        device: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt.command {
        Command::Device { list } => {
            if list {
                for mut dev in AMDevice::devices() {
                    println!(
                        "model: {}, udid: ({}), class: {}, build version: {}, product version: {}",
                        dev.get_value("HardwareModel", None),
                        dev.device_identifier(),
                        dev.get_value("DeviceClass", None),
                        dev.get_value("BuildVersion", None),
                        dev.get_value("ProductVersion", None)
                    );
                }
            }
        }
        _ => {}
    }
    println!("{:?}", opt);
}
