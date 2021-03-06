use std::{net::IpAddr, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Uploads a bitstream (FPG or BOF) file to the SNAP
    Upload {
        path: PathBuf,
        /// The port to upload data through (separate from the katcp port)
        #[clap(long, default_value_t = 3000)]
        port: u16,
    },
    /// Configures the 10GbE Core
    ConfigGBE {
        /// The name of the 10GbE Core to configure (from Simulink)
        core: String,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub(crate) struct Args {
    /// Address of the SNAP tcpborph server
    pub(crate) address: IpAddr,
    #[clap(subcommand)]
    pub(crate) command: Command,
    /// Port of the SNAP katcp tcpborph server
    #[clap(short, long, default_value_t = 7147)]
    pub(crate) port: u16,
    /// Print all log messages and debug information
    #[clap(short, long)]
    pub(crate) verbose: bool,
}
