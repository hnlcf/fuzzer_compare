use crate::{
    constants::{CFLAGS, DB_DIR, FUZZERS_DIR, INSTALL_DIR, MARIADB_VERSION},
    utils::{self, ShellCommand},
};

use super::Builder;

pub struct MariadbBuilder;

impl Builder for MariadbBuilder {
    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        Self::build_with_squirrel()?;

        Ok(())
    }
}

impl MariadbBuilder {
    fn download() -> Result<(), Box<dyn std::error::Error>> {
        let ma_src_dir = format!("{}/mariadb", DB_DIR.as_str());
        utils::git_clone("https://github.com/MariaDB/server.git", Some(&ma_src_dir))?;

        let cmd = ["git", "checkout", MARIADB_VERSION];
        ShellCommand::new()
            .args(&cmd)
            .current_dir(&ma_src_dir)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }

    fn build_with_squirrel() -> Result<(), Box<dyn std::error::Error>> {
        let ma_install_dir = format!("{}/mariadb", INSTALL_DIR.as_str());
        if std::path::Path::new(&ma_install_dir).exists() {
            return Ok(());
        }

        let ma_src_dir = format!("{}/mariadb", DB_DIR.as_str());
        if !std::path::Path::new(&ma_src_dir).exists() {
            Self::download()?;
        }

        let ma_bin_dir = format!("{}/build", ma_src_dir);
        utils::create_dir(&ma_bin_dir)?;

        let cmake_vars = [
            format!(
                "-DCMAKE_C_COMPILER={}/squirrel/AFLplusplus/afl-cc",
                FUZZERS_DIR.as_str()
            ),
            format!(
                "-DCMAKE_CXX_COMPILER={}/squirrel/AFLplusplus/afl-c++",
                FUZZERS_DIR.as_str()
            ),
            format!("-DCMAKE_INSTALL_PREFIX={}/mariadb", INSTALL_DIR.as_str()),
            format!("-DCMAKE_CXX_FLAGS='{}'", CFLAGS),
            "-DWITH_UNIT_TESTS=OFF".to_string(),
        ];
        utils::cmake_config(&ma_src_dir, Some(&ma_bin_dir), Some(&cmake_vars))?;
        utils::cmake_build::<&str>(&ma_src_dir, None, None, None, None)?;
        utils::cmake_build::<&str>(&ma_src_dir, None, None, Some("install"), None)?;

        Ok(())
    }
}
