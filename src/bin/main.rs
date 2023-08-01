use fuzzer_compare::{
    builder::{BoostBuilder, Builder, MariadbBuilder, MysqlBuilder, PgsqlBuilder, SquirrelBuilder},
    constants::{CONFIG_DIR, DB_DIR, FUZZERS_DIR, INSTALL_DIR, OUTPUT_DIR, TEST_DIR, TMP_DIR},
    utils,
};

use clap::Parser;

fn pre_setup() -> Result<(), Box<dyn std::error::Error>> {
    utils::create_dir(TMP_DIR.as_str())?;
    utils::create_dir(DB_DIR.as_str())?;
    utils::create_dir(FUZZERS_DIR.as_str())?;
    utils::create_dir(INSTALL_DIR.as_str())?;
    utils::create_dir(CONFIG_DIR.as_str())?;
    utils::create_new_dir(TEST_DIR.as_str())?;
    utils::create_new_dir(OUTPUT_DIR.as_str())?;

    Ok(())
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    pre_setup()?;

    SquirrelBuilder::setup()?;
    PgsqlBuilder::setup()?;
    BoostBuilder::setup()?;
    MysqlBuilder::setup()?;
    MariadbBuilder::setup()?;

    Ok(())
}

/// A cli for comparing fuzzers.
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
        setup()?;
    } else if args.run {
        println!("Running");
    } else {
        println!("Uknown arguments");
    }

    Ok(())
}
