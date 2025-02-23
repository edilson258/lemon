use clap::{Arg, ArgAction, Command};

pub fn command_line() -> clap::ArgMatches {
	let matches: clap::ArgMatches = Command::new("lemon")
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.subcommand_required(true)
		.arg_required_else_help(true)
		.subcommand(Command::new("check").about("check lemon.").arg(Arg::new("file").required(true)))
		.subcommand(
			Command::new("run")
				.about("run lemon.")
				.arg(Arg::new("file").help("path to the lemon file").required(true)),
		)
		.subcommand(
			Command::new("compile")
				.about("compile lemon to machine code.")
				.arg(Arg::new("file").help("path to the lemon file").required(true))
				.arg(
					Arg::new("target")
						.help("target triple for cross-compilation [default: host target]")
						.short('t')
						.long("target"),
				)
				.arg(
					Arg::new("linker")
						.help("choise linker")
						.short('l')
						.long("linker")
						.value_parser(["mold", "lld", "clang"])
						.default_value("clang"),
				)
				.arg(
					Arg::new("assembly")
						.help("generate assembly language")
						.short('s')
						.long("assembly")
						.action(ArgAction::SetTrue),
				)
				.arg(Arg::new("lnr").help("generate lemon ir").long("lnr").action(ArgAction::SetTrue))
				.arg(Arg::new("llr").help("generate llvm ir").long("llr").action(ArgAction::SetTrue))
				.arg(
					Arg::new("no-comptime")
						.help("disable comptime")
						.long("no-comptime")
						.action(ArgAction::SetTrue),
				)
				.arg(
					Arg::new("no-optimize")
						.help("disable optimize")
						.long("no-optimize")
						.action(ArgAction::SetTrue),
				)
				.arg(Arg::new("no-debug").help("disable debug").long("no-debug").action(ArgAction::SetTrue))
				.arg(Arg::new("output").help("custom output file path").short('o').long("output")),
		)
		.subcommand(Command::new("token").arg(Arg::new("file").required(true)).hide(true))
		.subcommand(Command::new("ast").arg(Arg::new("file").required(true)).hide(true))
		.get_matches();
	matches
}
