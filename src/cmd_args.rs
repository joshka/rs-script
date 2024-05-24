use bitflags::bitflags;
use clap::Parser;
use core::{fmt, str};
use std::error::Error;

use crate::errors::BuildRunError;

/// Clap command-line options
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Parser, Debug)]
#[command(version = "1.0", author = "durbanlegend")]
pub(crate) struct Cli {
    /// Set the script to run
    pub(crate) script: Option<String>,
    /// Set the arguments for the script
    #[arg(last = true)]
    pub(crate) args: Vec<String>,
    /// Set verbose mode
    #[arg(short, long)]
    pub(crate) verbose: bool,
    /// Display timings
    #[arg(short, long)]
    pub(crate) timings: bool,
    /// Generate Rust source and individual cargo .toml if compiled file is stale
    #[arg(short = 'g', long = "gen")]
    pub(crate) generate: bool,
    /// Build script if compiled file is stale
    #[arg(short, long)]
    pub(crate) build: bool,
    /// Force generation of Rust source and individual Cargo.toml, and build, even if compiled file is not stale
    #[arg(short, long)]
    pub(crate) force: bool,
    ///  (Default) Carry out generation and build steps (if necessary or forced) and run the compiled script
    #[arg(short, long, default_value = "true")]
    pub(crate) all: bool,
    /// Run compiled script if available
    #[arg(short, long)]
    pub(crate) run: bool,
    /// Run in REPL mode (read–eval–print loop). Existing script name is optional.
    #[arg(short = 'l', long, conflicts_with_all(["all", "generate", "build", "run"]))]
    pub(crate) repl: bool,
    #[arg(short, long = "expr", conflicts_with_all(["all", "generate", "build", "run", "repl", "script", "stdin"]))]
    pub(crate) expression: Option<String>,
    #[arg(short, long, conflicts_with_all(["all", "expression", "generate", "build", "run", "repl", "script"]))]
    pub(crate) stdin: bool,
    #[arg(short, long, conflicts_with("verbose"))]
    pub(crate) quiet: bool,
}

/// Getter for clap command-line options
pub(crate) fn get_opt() -> Cli {
    Cli::parse()
}

bitflags! {
    // You can `#[derive]` the `Debug` trait, but implementing it manually
    // can produce output like `A | B` instead of `Flags(A | B)`.
    // #[derive(Debug)]
    #[derive(Clone, PartialEq, Eq)]
    /// Processing flags for ease of handling command-line options
    pub struct ProcFlags: u32 {
        const GENERATE = 1;
        const BUILD = 2;
        const FORCE = 4;
        const RUN = 8;
        const ALL = 16;
        const VERBOSE = 32;
        const TIMINGS = 64;
        const REPL = 128;
        const EXPR = 256;
        const STDIN = 512;
        const QUIET = 1024;
    }
}

impl fmt::Debug for ProcFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}

impl fmt::Display for ProcFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}

impl str::FromStr for ProcFlags {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str(flags)
    }
}

/// Set up the processing flags from the command line arguments and pass them back.
pub(crate) fn get_proc_flags(options: &Cli) -> Result<ProcFlags, Box<dyn Error>> {
    let is_expr = options.expression.is_some();
    let proc_flags = {
        let mut proc_flags = ProcFlags::empty();
        // TODO: out? once clap default_value_ifs is working
        proc_flags.set(
            ProcFlags::GENERATE,
            options.generate | options.force | options.all | is_expr,
        );
        proc_flags.set(
            ProcFlags::BUILD,
            options.build | options.force | options.all | is_expr,
        );
        proc_flags.set(ProcFlags::FORCE, options.force);
        proc_flags.set(ProcFlags::QUIET, options.quiet);
        proc_flags.set(ProcFlags::VERBOSE, options.verbose);
        proc_flags.set(ProcFlags::TIMINGS, options.timings);
        proc_flags.set(ProcFlags::RUN, options.run | options.all);
        proc_flags.set(ProcFlags::ALL, options.all);
        if !(proc_flags.contains(ProcFlags::ALL)) {
            proc_flags.set(
                ProcFlags::ALL,
                options.generate & options.build & options.run,
            );
        }
        proc_flags.set(ProcFlags::REPL, options.repl);
        proc_flags.set(ProcFlags::EXPR, is_expr);
        proc_flags.set(ProcFlags::STDIN, options.stdin);

        // if options.all && options.run {
        //     // println!(
        //     //     "Conflicting options {} and {} specified",
        //     //     options.all, options.run
        //     // );
        //     return Err(Box::new(BuildRunError::Command(format!(
        //         "Conflicting options {} and {} specified",
        //         options.all, options.run
        //     ))));
        // }
        let formatted = proc_flags.to_string();
        let parsed = formatted
            .parse::<ProcFlags>()
            .map_err(|e| BuildRunError::FromStr(e.to_string()))?;

        assert_eq!(proc_flags, parsed);

        Ok::<ProcFlags, BuildRunError>(proc_flags)
    }?;
    Ok(proc_flags)
}

#[allow(dead_code)]
fn main() {
    let opt = Cli::parse();

    if opt.verbose {
        println!("Verbosity enabled");
    }

    if opt.timings {
        println!("Timings enabled");
    }

    if opt.generate {
        println!("Generating source and cargo .toml file");
    }

    if opt.build {
        println!("Building something");
    }

    if opt.force {
        println!("Generating and building something");
    }

    if opt.run {
        println!("Running script");
    }

    println!("Running script: {:?}", opt.script);
    if !opt.args.is_empty() {
        println!("With arguments:");
        for arg in &opt.args {
            println!("{arg}");
        }
    }
}
