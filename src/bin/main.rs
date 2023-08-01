use fuzzer_compare::{
    builder::{BoostBuilder, Builder, MariadbBuilder, MysqlBuilder, PgsqlBuilder, SquirrelBuilder},
    constants::{CONFIG_DIR, DB_DIR, FUZZERS_DIR, INSTALL_DIR, OUTPUT_DIR, TEST_DIR, TMP_DIR},
    utils,
};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pre_setup()?;

    SquirrelBuilder::setup()?;
    PgsqlBuilder::setup()?;
    BoostBuilder::setup()?;
    MysqlBuilder::setup()?;
    MariadbBuilder::setup()?;

    Ok(())
}
