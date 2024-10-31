use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("Lemon")
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(Command::new("check").about("check lemon file.").arg(Arg::new("file").required(true)))
    .get_matches();
  return matches;
}
