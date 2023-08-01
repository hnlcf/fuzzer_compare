use crate::{constants::FUZZERS_DIR, utils};

use super::Builder;

pub struct SquirrelBuilder;

impl Builder for SquirrelBuilder {
    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        Self::download()?;
        Self::build()?;

        Ok(())
    }
}

impl SquirrelBuilder {
    fn download() -> Result<(), Box<dyn std::error::Error>> {
        let squirrel_src_dir = format!("{}/squirrel", FUZZERS_DIR.as_str());

        if !std::path::Path::new(&squirrel_src_dir).exists() {
            utils::git_clone(
                "https://github.com/s3team/Squirrel.git",
                Some(&squirrel_src_dir),
            )?;
        }

        Ok(())
    }

    fn build() -> Result<(), Box<dyn std::error::Error>> {
        let squirrel_src_dir = format!("{}/squirrel", FUZZERS_DIR.as_str());
        let squirrel_bin_dir = format!("{}/build", &squirrel_src_dir);
        utils::create_dir(&squirrel_bin_dir)?;

        let cmake_vars = [
            "-Wno-dev",
            "-DALL=ON",
            "-DSQLITE=ON",
            "-DMYSQL=ON",
            "-DPOSTGRESQL=ON",
        ];

        utils::cmake_config(&squirrel_src_dir, None, Some(&cmake_vars))?;
        utils::cmake_build::<&str>(&squirrel_src_dir, None, None, None, None)?;

        let afl_src_dir = format!("{}/AFLplusplus", &squirrel_src_dir);
        let build_afl_cmd = ["make", "-j"];
        utils::ShellCommand::new()
            .args(&build_afl_cmd)
            .current_dir(&afl_src_dir)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}
