use std::{env, ffi::OsString, io, process, process::Command};

fn main() -> Result<(), io::Error> {
	let bash = env::var_os("BASH").unwrap_or_else(|| OsString::from("bash"));
	let script = env::args().skip(2).collect::<Vec<_>>();
	// TODO: escape? shlex::join(script.iter().map(|x| &**x))
	let script = script.join(" ");
	let mut bash = Command::new(bash);
	bash.args([
		"--noprofile",
		"--norc",
		"-o",
		"errexit",
		"-o",
		"nounset",
		"-o",
		"pipefail",
		"-O",
		"nullglob",
		"-O",
		"dotglob",
		"-c",
	])
	.arg(&*script);
	let exit = bash.spawn()?.wait()?;
	if !exit.success() {
		process::exit(exit.code().unwrap_or(1));
	}
	Ok(())
}
