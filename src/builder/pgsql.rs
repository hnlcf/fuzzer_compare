use crate::{
    constants::{CFLAGS, DB_DIR, FUZZERS_DIR, INSTALL_DIR, PGSQL_VERSION},
    utils::{self, ShellCommand},
};

use super::Builder;

pub struct PgsqlBuilder;

impl Builder for PgsqlBuilder {
    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        Self::build_with_squirrel()?;

        Ok(())
    }
}

impl PgsqlBuilder {
    fn download() -> Result<(), Box<dyn std::error::Error>> {
        let target = format!("postgresql-{}.tar.bz2", PGSQL_VERSION);
        let target_path = format!("{}/{}", DB_DIR.as_str(), target);

        let url = format!(
            "https://ftp.postgresql.org/pub/source/v{}/{}",
            PGSQL_VERSION, target
        );

        if !std::path::Path::new(&target_path).exists() {
            let download_cmd = ["wget", &url];

            ShellCommand::new()
                .args(&download_cmd)
                .current_dir(DB_DIR.as_str())
                .spawn()?
                .wait_with_output()?;
        }

        let extract_cmd = ["tar", "xvf", &target_path];
        ShellCommand::new()
            .args(&extract_cmd)
            .current_dir(DB_DIR.as_str())
            .spawn()?
            .wait_with_output()?;

        let raw_src_dir = format!("postgresql-{}", PGSQL_VERSION);
        let other_cmd = ["mv", &raw_src_dir, "postgresql"];
        ShellCommand::new()
            .args(&other_cmd)
            .current_dir(DB_DIR.as_str())
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }

    fn build_with_squirrel() -> Result<(), Box<dyn std::error::Error>> {
        let squirrel_fuzzers_dir = format!("{}/squirrel", FUZZERS_DIR.as_str());

        let pgsql_install_dir = format!("{}/postgresql", INSTALL_DIR.as_str());
        if std::path::Path::new(&pgsql_install_dir).exists() {
            return Ok(());
        }

        let pgsql_src_dir = format!("{}/postgresql", DB_DIR.as_str());
        if !std::path::Path::new(&pgsql_src_dir).exists() {
            Self::download()?;
        }

        let pgsql_bin_dir = format!("{}/build", pgsql_src_dir);
        utils::create_dir(&pgsql_bin_dir)?;

        let config_cmd = [
            "../configure".to_string(),
            format!("--prefix={}", pgsql_install_dir),
            format!("--with-CC={}/AFLplusplus/afl-cc", squirrel_fuzzers_dir),
            format!("CFLAGS='{}'", CFLAGS),
        ];
        ShellCommand::new()
            .args(&config_cmd)
            .current_dir(&pgsql_bin_dir)
            .spawn()?
            .wait_with_output()?;

        let build_cmd = ["make", "world-bin", "&&", "make", "install-world-bin"];
        ShellCommand::new()
            .args(&build_cmd)
            .current_dir(&pgsql_bin_dir)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}
