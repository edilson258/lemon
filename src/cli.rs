use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("lemon")
    .about("An cross-platform functional language for native mobile apps.")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("check")
        .about("check a lemon file.")
        .arg(Arg::new("file").help("the lemon file to check.").required(true)),
    )
    .subcommand(
      Command::new("compile")
        .about("compile a lemon file.")
        .arg(Arg::new("file").help("the lemon file to compile.").required(true)),
    )
    .subcommand(
      Command::new("run")
        .about("run a lemon file.")
        .arg(Arg::new("file").help("the lemon file to run.").required(true)),
    )
    .get_matches();

  return matches;
}
