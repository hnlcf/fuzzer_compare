use clap::Parser;

/// A Cli for comparing fuzzers.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Setup all fuzzers and databases.
    #[arg(short, long, default_value_t = false)]
    setup: bool,

    /// Launch fuzzers in parallel.
    #[arg(short, long, default_value_t = false)]
    run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.setup {
        fuzzer_compare::setup()?;
    } else if args.run {
        launch();
    } else {
        println!("Unknown arguments");
    }

    Ok(())
}

fn launch() {
    fuzzer_compare::launch_monitors();

    let handles = fuzzer_compare::launch_fuzzers();
    for i in handles {
        i.join().unwrap();
    }
}
