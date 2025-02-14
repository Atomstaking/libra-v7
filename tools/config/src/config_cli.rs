use crate::make_profile;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
/// Generate a libra config file in the home .libra directory
pub struct ConfigCli {
    #[clap(subcommand)]
    subcommand: Option<ConfigSub>,
}

#[derive(clap::Subcommand)]
enum ConfigSub {
    /// Generate config.yaml file that stores 0L configuration
    Init {
        /// Ed25519 public key
        #[clap(short, long)]
        public_key: String,

        /// Profile name to use when saving the config. Defaults to "default"
        ///
        /// This will be used to override associated settings such as
        /// the REST URL, and the private key arguments.
        #[clap(long)]
        profile: Option<String>,

        /// In libra we default to the configs being global in $HOME/.libra
        /// Otherwise you should pass -w to use the workspace configuration.
        /// Uses this directory as the workspace, instead of using the global
        /// parameters in $HOME/.libra
        #[clap(short, long)]
        workspace: bool,
    },
}

impl ConfigCli {
    pub async fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(ConfigSub::Init {
                public_key,
                profile,
                workspace,
            }) => make_profile::run(
              public_key, 
              profile.as_deref().to_owned(),
              *workspace,
            ).await,
            _ => Ok(()),
        }
    }
}
