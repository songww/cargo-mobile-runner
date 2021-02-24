use serde_json::json as jsonify;
use std::path::PathBuf;
use structopt::StructOpt;

use ios_deployer::mobile_device::{amd_set_log_level, AMDevice, AMDeviceInterfaceType};

#[derive(Debug, StructOpt)]
#[structopt(name = "ios-deployer", about = "Helpers for ios app development.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    verbose: bool,

    /// format output as JSON
    #[structopt(short, long)]
    json: bool,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Device(DeviceCommand),

    Install {
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

#[derive(Debug, StructOpt)]
enum DeviceCommand {
    /// List the device which is connected.
    List {
        /// ignore wifi devices
        #[structopt(long)]
        no_wifi: bool,
    },

    Get {
        /// Installed applications.
        #[structopt(long)]
        installed: bool,

        ///udid of device which to run on, use first one, if not specifyed .
        #[structopt(short, long)]
        device: Option<String>,
    },
}

fn main() {
    amd_set_log_level(5);
    let opt = Opt::from_args();
    let json_out = opt.json;
    match opt.command {
        Command::Device(device_cmd) => match device_cmd {
            DeviceCommand::List { no_wifi } => {
                for mut dev in AMDevice::devices() {
                    let interface_type = dev.interface_type();
                    if no_wifi {
                        match interface_type {
                            AMDeviceInterfaceType::Wifi => {
                                continue;
                            }
                            _ => {}
                        }
                    }
                    dev.connect().unwrap();
                    let device_udid = dev.device_identifier();
                    let device_name = dev.get_value("DeviceName", None);
                    let device_class = dev.get_value("DeviceClass", None);
                    let build_version = dev.get_value("BuildVersion", None);
                    let product_type = dev.get_value("ProductType", None);
                    let product_version = dev.get_value("ProductVersion", None);
                    let model = dev.get_model();

                    let model_name = model.name();
                    let sdk_name = model.sdk();
                    let arch_name = model.arch();

                    if json_out {
                        let v = jsonify!({
                        "event" : "DeviceDetected",
                        "device" : {
                          "build_version" : build_version,
                          "model_sdk" : sdk_name,
                          "device_identifier" : device_udid,
                          "device_class" : device_class,
                          "product_type" : product_type,
                          "device_name" : device_name,
                          "product_version" : product_version,
                          "model_arch" : arch_name,
                          "hardware_model" : model.model(),
                          "model_name" : model_name
                        }
                                              });
                        println!("{}", serde_json::to_string_pretty(&v).unwrap());
                    } else {
                        println!(
                            "{} ({}, {}, {}, {}, {}, {}) a.k.a. '{}' connected through {}",
                            device_udid,
                            model.model(),
                            model_name,
                            sdk_name,
                            arch_name,
                            product_version.unwrap_or_default(),
                            build_version.unwrap_or_default(),
                            device_name.unwrap_or_default(),
                            interface_type.to_string(),
                        );
                    }
                    dev.disconnect().unwrap();
                }
            }
            DeviceCommand::Get { installed, device } => {
                let devices = AMDevice::devices();
                let device = if device.is_none() {
                    if devices.is_empty() {
                        // panic!("No connected device found.");
                        None
                    } else {
                        devices.into_iter().next()
                    }
                } else {
                    let udid = device.unwrap();
                    let mut devices = AMDevice::devices();
                    devices
                        .iter_mut()
                        .position(|dev| dev.device_identifier() == udid)
                        .and_then(|pos| devices.into_iter().take(pos).last())
                };
                let mut device = device.expect("No one device connected.");
                device.connect().unwrap();
                assert!(device.is_paired());
                device.validate_pairing().unwrap();
                device.start_session().unwrap();

                let attrs = &[
                    "CFBundleIdentifier",
                    "CFBundleName",
                    "CFBundleDisplayName",
                    "CFBundleVersion",
                    "CFBundleShortVersionString",
                ];

                //
            }
        },
        _ => {
            println!("{:?}", opt);
        }
    }
}
