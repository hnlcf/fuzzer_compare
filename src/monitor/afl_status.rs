use crate::utils::ShellCommand;

pub struct AflStatusMonitor {
    pub status_file: String,
}

pub struct AflStautsResult {
    pub case_num: i32,
    pub cycle_num: i32,
    pub crash_num: i32,
    pub edge_covered: i32,
    pub edge_percent: f64,
}

impl AflStatusMonitor {
    pub fn new(status_file: String) -> Self {
        Self { status_file }
    }

    pub fn get_result(&self) -> AflStautsResult {
        let case_num = self
            .grep_item("corpus_count")
            .map(|v| v.parse().unwrap_or(-1))
            .unwrap();
        let cycle_num = self
            .grep_item("execs_done")
            .map(|v| v.parse().unwrap_or(-1))
            .unwrap();
        let crash_num = self
            .grep_item("saved_crashes")
            .map(|v| v.parse().unwrap_or(-1))
            .unwrap();
        let edge_covered = self
            .grep_item("edges_found")
            .map(|v| v.parse().unwrap_or(-1))
            .unwrap();
        let edge_percent = self
            .grep_item("bitmap_cvg")
            .map(|v| v.strip_suffix('%').unwrap().parse().unwrap_or(-1.0))
            .unwrap();

        AflStautsResult {
            case_num,
            cycle_num,
            crash_num,
            edge_covered,
            edge_percent,
        }
    }

    fn grep_item(&self, pattern: &str) -> Option<String> {
        let grep_cmd = ["grep", pattern, &self.status_file];

        let output = ShellCommand::new()
            .args(&grep_cmd)
            .pipe_stdio()
            .spawn()
            .ok()?
            .wait_with_output()
            .ok()?;

        String::from_utf8(output.stdout).ok()
    }
}
