use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const DATETIME_FORMAT_STR: &str = "%Y-%m-%d %H:%M:%S";

pub const NPROCS: i8 = 6;

pub const CFLAGS: &str = "-fprofile-instr-generate -fcoverage-mapping";

pub const PGSQL_VERSION: &str = "15.0";

pub const BOOST_VERSION: &str = "1.77.0";
pub const BOOST_VERSION_1: &str = "1_77_0";

pub const MYSQL_VERSION: &str = "8.0";
pub const MYSQL_TCP_PORT: &str = "3307";
pub const MYSQL_UNIX_ADDR: &str = "/tmp/another_mysql.sock";

pub const MARIADB_VERSION: &str = "10.5";

/// Current workspace
pub static ROOT: Lazy<String> = Lazy::new(|| {
    std::env::current_dir()
        .expect("Cannot get current workspace")
        .display()
        .to_string()
});

/// All temporary files directory
pub static TMP_DIR: Lazy<String> = Lazy::new(|| format!("{}/tmp", ROOT.as_str()));

/// Database source code directory
pub static DB_DIR: Lazy<String> = Lazy::new(|| format!("{}/db", TMP_DIR.as_str()));

/// Fuzzers source code directory
pub static FUZZERS_DIR: Lazy<String> = Lazy::new(|| format!("{}/fuzzers", TMP_DIR.as_str()));

/// Binary installed directory
pub static INSTALL_DIR: Lazy<String> = Lazy::new(|| format!("{}/install", TMP_DIR.as_str()));

/// Configurations needed by fuzzer directory
pub static CONFIG_DIR: Lazy<String> = Lazy::new(|| format!("{}/config", TMP_DIR.as_str()));

/// Test files directory
pub static TEST_DIR: Lazy<String> = Lazy::new(|| format!("{}/test", TMP_DIR.as_str()));

/// Monitor status output directory
pub static OUTPUT_DIR: Lazy<String> = Lazy::new(|| format!("{}/output", TMP_DIR.as_str()));

/// Squirrel running envs for per database
pub static SQUIRREL_ENVS_TABLE: Lazy<HashMap<&'static str, HashMap<&'static str, String>>> =
    Lazy::new(|| {
        let envs = vec![
            (
                "postgresql",
                vec![
                    ("AFL_CUSTOM_MUTATOR_ONLY", "1".to_string()),
                    (
                        "AFL_CUSTOM_MUTATOR_LIBRARY",
                        format!(
                            "{}/squirrel/build/libpostgresql_mutator.so",
                            FUZZERS_DIR.as_str()
                        ),
                    ),
                    ("AFL_IGNORE_PROBLEMS", "1".to_string()),
                    ("AFL_DISABLE_TRIM", "1".to_string()),
                    ("AFL_MAP_SIZE", "2097152".to_string()),
                    ("AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES", "1".to_string()),
                    ("AFL_AUTORESUME", "1".to_string()),
                    ("AFL_DEBUG", "1".to_string()),
                    ("AFL_NO_UI", "1".to_string()),
                    ("AFL_FORKSRV_INIT_TMOUT", "1000000".to_string()),
                    (
                        "SQUIRREL_CONFIG",
                        format!("{}/squirrel/config_postgresql.yml", CONFIG_DIR.as_str()),
                    ),
                    (
                        "TEST_PATH",
                        format!("{}/squirrel-pg/test", TEST_DIR.as_str()),
                    ),
                    (
                        "PGSQL_INSTALL_PATH",
                        format!("{}/postgresql", INSTALL_DIR.as_str()),
                    ),
                    (
                        "LLVM_PROFILE_FILE",
                        format!(
                            "{}/squirrel-pg/prof/squirrel-postgresql-%p-%m.profraw",
                            TEST_DIR.as_str()
                        ),
                    ),
                ],
            ),
            (
                "mysql",
                vec![
                    ("AFL_CUSTOM_MUTATOR_ONLY", "1".to_string()),
                    (
                        "AFL_CUSTOM_MUTATOR_LIBRARY",
                        format!(
                            "{}/squirrel/build/libmysql_mutator.so",
                            FUZZERS_DIR.as_str()
                        ),
                    ),
                    ("AFL_IGNORE_PROBLEMS", "1".to_string()),
                    ("AFL_DISABLE_TRIM", "1".to_string()),
                    ("AFL_MAP_SIZE", "2097152".to_string()),
                    ("AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES", "1".to_string()),
                    ("AFL_AUTORESUME", "1".to_string()),
                    ("AFL_DEBUG", "1".to_string()),
                    ("AFL_NO_UI", "1".to_string()),
                    ("AFL_FORKSRV_INIT_TMOUT", "1000000".to_string()),
                    (
                        "SQUIRREL_CONFIG",
                        format!("{}/squirrel/config_mysql.yml", CONFIG_DIR.as_str()),
                    ),
                    (
                        "TEST_PATH",
                        format!("{}/squirrel-my/test", TEST_DIR.as_str()),
                    ),
                    (
                        "MYSQL_INSTALL_PATH",
                        format!("{}/mysql", INSTALL_DIR.as_str()),
                    ),
                    (
                        "LLVM_PROFILE_FILE",
                        format!(
                            "{}/squirrel-my/prof/squirrel-mysql-%p-%m.profraw",
                            TEST_DIR.as_str()
                        ),
                    ),
                ],
            ),
            (
                "mariadb",
                vec![
                    ("AFL_CUSTOM_MUTATOR_ONLY", "1".to_string()),
                    (
                        "AFL_CUSTOM_MUTATOR_LIBRARY",
                        format!(
                            "{}/squirrel/build/libmysql_mutator.so",
                            FUZZERS_DIR.as_str()
                        ),
                    ),
                    ("AFL_IGNORE_PROBLEMS", "1".to_string()),
                    ("AFL_DISABLE_TRIM", "1".to_string()),
                    ("AFL_MAP_SIZE", "2097152".to_string()),
                    ("AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES", "1".to_string()),
                    ("AFL_AUTORESUME", "1".to_string()),
                    ("AFL_DEBUG", "1".to_string()),
                    ("AFL_NO_UI", "1".to_string()),
                    ("AFL_FORKSRV_INIT_TMOUT", "1000000".to_string()),
                    (
                        "SQUIRREL_CONFIG",
                        format!("{}/squirrel/config_mariadb.yml", CONFIG_DIR.as_str()),
                    ),
                    (
                        "TEST_PATH",
                        format!("{}/squirrel-ma/test", TEST_DIR.as_str()),
                    ),
                    (
                        "MARIADB_INSTALL_PATH",
                        format!("{}/mariadb", INSTALL_DIR.as_str()),
                    ),
                    (
                        "LLVM_PROFILE_FILE",
                        format!(
                            "{}/squirrel-ma/prof/squirrel-mariadb-%p-%m.profraw",
                            TEST_DIR.as_str()
                        ),
                    ),
                ],
            ),
        ];

        envs.into_iter()
            .map(|(s, m)| {
                let tmp: HashMap<&str, String> = m.into_iter().map(|(k, v)| (k, v)).collect();
                (s, tmp)
            })
            .collect()
    });
