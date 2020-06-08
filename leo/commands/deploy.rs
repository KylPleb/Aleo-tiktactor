use crate::{cli::*, cli_types::*, commands::BuildCommand, errors::CLIError, files::Manifest};

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct DeployCommand;

impl CLI for DeployCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Deploy the current package as a program to the network (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "deploy";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (_program, _checksum_differs) = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let _package_name = Manifest::try_from(&path)?.get_package_name();

        log::info!("Unimplemented - `leo deploy`");

        Ok(())
    }
}
