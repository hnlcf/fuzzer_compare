use std::collections::HashMap;

use crate::{
    constants::{FUZZERS_DIR, INSTALL_DIR, SQUIRREL_ENVS_TABLE, TEST_DIR},
    utils::{self, ShellCommand},
};

use super::Runner;

pub struct SquirrelRunner;

impl Runner for SquirrelRunner {
    fn run_pgsql() -> Result<(), Box<dyn std::error::Error>> {
        let fuzzer_test_dir = SQUIRREL_ENVS_TABLE
            .get("postgresql")
            .unwrap()
            .get("TEST_PATH")
            .unwrap();
        let fuzzer_db_data_dir = format!("{}/data", fuzzer_test_dir.as_str());
        utils::create_dir(fuzzer_test_dir)?;
        utils::create_dir(&fuzzer_db_data_dir)?;

        let pg_envs: HashMap<&str, String> =
            SQUIRREL_ENVS_TABLE.get("postgresql").unwrap().to_owned();

        let init_cmd = [
            format!("{}/postgresql/bin/initdb", INSTALL_DIR.as_str()),
            "-D".to_string(),
            "$TEST_PATH/data".to_string(),
        ];
        ShellCommand::new()
            .args(&init_cmd)
            .current_dir(TEST_DIR.as_str())
            .envs(&pg_envs)
            .spawn()?
            .wait_with_output()?;

        let run_cmd = [
            format!("{}/squirrel/AFLplusplus/afl-fuzz", FUZZERS_DIR.as_str()),
            "-i".to_string(),
            format!(
                "{}/squirrel/data/fuzz_root/pqsql_input",
                FUZZERS_DIR.as_str()
            ),
            "-o".to_string(),
            "$TEST_PATH".to_string(),
            "-t".to_string(),
            "120000".to_string(),
            "-S".to_string(),
            "1".to_string(),
            "--".to_string(),
            format!("{}/squirrel/build/db_driver", FUZZERS_DIR.as_str()),
        ];
        ShellCommand::new()
            .args(&run_cmd)
            .current_dir(TEST_DIR.as_str())
            .envs(&pg_envs)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }

    fn run_mysql() -> Result<(), Box<dyn std::error::Error>> {
        let fuzzer_test_dir = SQUIRREL_ENVS_TABLE
            .get("mysql")
            .unwrap()
            .get("TEST_PATH")
            .unwrap();
        let fuzzer_db_data_dir = format!("{}/data", fuzzer_test_dir.as_str());
        utils::create_dir(fuzzer_test_dir)?;
        utils::create_dir(&fuzzer_db_data_dir)?;

        let my_envs: HashMap<&str, String> = SQUIRREL_ENVS_TABLE.get("mysql").unwrap().to_owned();

        let init_cmd = [
            format!("{}/mysql/bin/mysqld", INSTALL_DIR.as_str()),
            "-D".to_string(),
            "$TEST_PATH/data".to_string(),
        ];
        ShellCommand::new()
            .args(&init_cmd)
            .current_dir(TEST_DIR.as_str())
            .envs(&my_envs)
            .spawn()?
            .wait_with_output()?;

        let run_cmd = [
            format!("{}/squirrel/AFLplusplus/afl-fuzz", FUZZERS_DIR.as_str()),
            "-i".to_string(),
            format!(
                "{}/squirrel/data/fuzz_root/mysql_input",
                FUZZERS_DIR.as_str()
            ),
            "-o".to_string(),
            "$TEST_PATH".to_string(),
            "-t".to_string(),
            "120000".to_string(),
            "-S".to_string(),
            "1".to_string(),
            "--".to_string(),
            format!("{}/squirrel/build/db_driver", FUZZERS_DIR.as_str()),
        ];
        ShellCommand::new()
            .args(&run_cmd)
            .current_dir(TEST_DIR.as_str())
            .envs(&my_envs)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }

    fn run_mariadb() -> Result<(), Box<dyn std::error::Error>> {
        let fuzzer_test_dir = SQUIRREL_ENVS_TABLE
            .get("mariadb")
            .unwrap()
            .get("TEST_PATH")
            .unwrap();
        let fuzzer_db_data_dir = format!("{}/data", fuzzer_test_dir.as_str());
        utils::create_dir(fuzzer_test_dir)?;
        utils::create_dir(&fuzzer_db_data_dir)?;

        let ma_envs: HashMap<&str, String> = SQUIRREL_ENVS_TABLE.get("mariadb").unwrap().to_owned();

        let init_cmd = [
            format!(
                "{}/mariadb/scripts/mariadb-install-db",
                INSTALL_DIR.as_str()
            ),
            format!("--basedir={}/mariadb", INSTALL_DIR.as_str()),
            format!("--datadir={}/data", fuzzer_db_data_dir),
        ];
        ShellCommand::new()
            .args(&init_cmd)
            .current_dir(TEST_DIR.as_str())
            .envs(&ma_envs)
            .spawn()?
            .wait_with_output()?;

        let run_cmd = [
            format!("{}/squirrel/AFLplusplus/afl-fuzz", FUZZERS_DIR.as_str()),
            "-i".to_string(),
            format!(
                "{}/squirrel/data/fuzz_root/mysql_input",
                FUZZERS_DIR.as_str()
            ),
            "-o".to_string(),
            "$TEST_PATH".to_string(),
            "-t".to_string(),
            "120000".to_string(),
            "-S".to_string(),
            "1".to_string(),
            "--".to_string(),
            format!("{}/squirrel/build/db_driver", FUZZERS_DIR.as_str()),
        ];
        ShellCommand::new()
            .args(&run_cmd)
            .current_dir(TEST_DIR.as_str())
            .envs(&ma_envs)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}
