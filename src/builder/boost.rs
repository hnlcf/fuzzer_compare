use crate::{
    constants::{BOOST_VERSION, BOOST_VERSION_1, DB_DIR},
    utils::ShellCommand,
};

use super::Builder;

pub struct BoostBuilder;

impl Builder for BoostBuilder {
    fn setup() -> Result<(), Box<dyn std::error::Error>> {
        let boost_src_dir = format!("{}/boost", DB_DIR.as_str());
        if !std::path::Path::new(&boost_src_dir).exists() {
            Self::download()?;
        }

        Ok(())
    }
}

impl BoostBuilder {
    fn download() -> Result<(), Box<dyn std::error::Error>> {
        let file = format!("boost_{}.tar.bz2", BOOST_VERSION_1);
        let target = format!("{}/{}", DB_DIR.as_str(), file);

        if !std::path::Path::new(&target).exists() {
            let download_cmd = [
                "wget".to_string(),
                format!(
                    "https://boostorg.jfrog.io/artifactory/main/release/{}/source/{}",
                    BOOST_VERSION, file
                ),
            ];

            ShellCommand::new()
                .args(&download_cmd)
                .current_dir(DB_DIR.as_str())
                .spawn()?
                .wait_with_output()?;
        }

        let extract_cmd = ["tar", "xvf", &target];
        ShellCommand::new()
            .args(&extract_cmd)
            .current_dir(DB_DIR.as_str())
            .spawn()?
            .wait_with_output()?;

        let raw_src_dir = format!("boost_{}", BOOST_VERSION_1);
        let other_cmd = ["mv", &raw_src_dir, "boost"];
        ShellCommand::new()
            .args(&other_cmd)
            .current_dir(DB_DIR.as_str())
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}
