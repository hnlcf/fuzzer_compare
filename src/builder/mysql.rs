use crate::{
    constants::{
        CFLAGS, DB_DIR, FUZZERS_DIR, INSTALL_DIR, MYSQL_TCP_PORT, MYSQL_UNIX_ADDR, MYSQL_VERSION,
    },
    utils::{self, ShellCommand},
};

use super::Builder;

pub struct MysqlBuilder;

impl Builder for MysqlBuilder {
    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        Self::build_with_squirrel()?;

        Ok(())
    }
}

impl MysqlBuilder {
    fn download() -> Result<(), Box<dyn std::error::Error>> {
        let my_src_dir = format!("{}/mysql", DB_DIR.as_str());
        utils::git_clone(
            "https://github.com/mysql/mysql-server.git",
            Some(&my_src_dir),
        )?;

        let cmd = ["git", "checkout", MYSQL_VERSION];
        ShellCommand::new()
            .args(&cmd)
            .current_dir(&my_src_dir)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }

    fn build_with_squirrel() -> Result<(), Box<dyn std::error::Error>> {
        let my_install_dir = format!("{}/mysql", INSTALL_DIR.as_str());
        if std::path::Path::new(&my_install_dir).exists() {
            return Ok(());
        }

        let my_src_dir = format!("{}/mysql", DB_DIR.as_str());
        if !std::path::Path::new(&my_src_dir).exists() {
            Self::download()?;
        }

        let my_bin_dir = format!("{}/build", my_src_dir);
        utils::create_dir(&my_bin_dir)?;

        let cmake_vars = [
            format!(
                "-DCMAKE_C_COMPILER={}/squirrel/AFLplusplus/afl-cc",
                FUZZERS_DIR.as_str()
            ),
            format!(
                "-DCMAKE_CXX_COMPILER={}/squirrel/AFLplusplus/afl-c++",
                FUZZERS_DIR.as_str()
            ),
            format!("-DCMAKE_INSTALL_PREFIX={}/mysql", INSTALL_DIR.as_str()),
            format!("-DCMAKE_CXX_FLAGS='{}'", CFLAGS),
            format!("-DMYSQL_TCP_PORT={}", MYSQL_TCP_PORT),
            format!("-DMYSQL_UNIX_ADDR={}", MYSQL_UNIX_ADDR),
            format!("-DWITH_BOOST={}/boost", DB_DIR.as_str()),
            "-DDOWNLOAD_BOOST=1".to_string(),
            "-DWITH_UNIT_TESTS=OFF".to_string(),
        ];
        utils::cmake_config(&my_src_dir, Some(&my_bin_dir), Some(&cmake_vars))?;
        utils::cmake_build::<&str>(&my_src_dir, None, None, None, None)?;
        utils::cmake_build::<&str>(&my_src_dir, None, None, Some("install"), None)?;

        Ok(())
    }
}
