use std::fs;

use crate::utils::ShellCommand;

pub struct LLVMCoverageMonitor {
    exec_path: String,
    profile_dir: String,
}

#[derive(Default)]
pub struct LLVMCoverageResult {
    pub region_cov: LLVMCoverageItem,
    pub func_cov: LLVMCoverageItem,
    pub line_cov: LLVMCoverageItem,
    pub branch_cov: LLVMCoverageItem,
}

#[derive(Default)]
pub struct LLVMCoverageItem {
    pub missed: i32,
    pub total: i32,
    pub percent: f64,
}

impl LLVMCoverageMonitor {
    pub fn new(exec_path: String, profile_dir: String) -> Self {
        Self {
            exec_path,
            profile_dir,
        }
    }

    pub fn get_result(&self) -> LLVMCoverageResult {
        self.merge().expect("Failed to merge llvm profraws");
        let raw_output = self.report().unwrap();

        let mut unparsed_output = raw_output.lines().last().unwrap().split_whitespace();
        unparsed_output.next();

        let region_total = unparsed_output.next().unwrap().parse().unwrap();
        let region_missed = unparsed_output.next().unwrap().parse().unwrap();
        let region_percent = unparsed_output
            .next()
            .unwrap()
            .strip_suffix('%')
            .unwrap()
            .parse()
            .unwrap();

        let region_cov = LLVMCoverageItem {
            missed: region_missed,
            total: region_total,
            percent: region_percent,
        };

        let func_total = unparsed_output.next().unwrap().parse().unwrap();
        let func_missed = unparsed_output.next().unwrap().parse().unwrap();
        let func_percent = unparsed_output
            .next()
            .unwrap()
            .strip_suffix('%')
            .unwrap()
            .parse()
            .unwrap();

        let func_cov = LLVMCoverageItem {
            missed: func_missed,
            total: func_total,
            percent: func_percent,
        };

        let line_total = unparsed_output.next().unwrap().parse().unwrap();
        let line_missed = unparsed_output.next().unwrap().parse().unwrap();
        let line_percent = unparsed_output
            .next()
            .unwrap()
            .strip_suffix('%')
            .unwrap()
            .parse()
            .unwrap();

        let line_cov = LLVMCoverageItem {
            missed: line_missed,
            total: line_total,
            percent: line_percent,
        };

        let branch_total = unparsed_output.next().unwrap().parse().unwrap();
        let branch_missed = unparsed_output.next().unwrap().parse().unwrap();
        let branch_percent = unparsed_output
            .next()
            .unwrap()
            .strip_suffix('%')
            .unwrap()
            .parse()
            .unwrap();

        let branch_cov = LLVMCoverageItem {
            missed: branch_missed,
            total: branch_total,
            percent: branch_percent,
        };

        LLVMCoverageResult {
            region_cov,
            func_cov,
            line_cov,
            branch_cov,
        }
    }

    fn merge(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut profraws = vec![];
        let paths = fs::read_dir(&self.profile_dir).unwrap();
        for f in paths {
            let f = f.unwrap().path();
            if f.ends_with("profraw") {
                profraws.push(f.display().to_string());
            }
        }

        let llvm_merge_cmd = [
            "llvm-profdata",
            "merge",
            &profraws.join(" "),
            "-o",
            "tmp.profdata",
        ];
        ShellCommand::new()
            .args(&llvm_merge_cmd)
            .current_dir(&self.profile_dir)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }

    fn report(&self) -> Option<String> {
        let llvm_cov_cmd = [
            "llvm-cov".to_string(),
            "report".to_string(),
            "--instr-profile".to_string(),
            format!("{}/tmp.profdata", &self.profile_dir),
            self.exec_path.clone(),
        ];
        let output = ShellCommand::new()
            .args(&llvm_cov_cmd)
            .current_dir(&self.profile_dir)
            .pipe_stdio()
            .spawn()
            .ok()?
            .wait_with_output()
            .ok()?;

        String::from_utf8(output.stdout).ok()
    }
}
