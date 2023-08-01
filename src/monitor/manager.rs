use std::fs;

use crate::constants::DATETIME_FORMAT_STR;
use crate::monitor::{AflStatusMonitor, AflStautsResult};

use super::{LLVMCoverageMonitor, LLVMCoverageResult};

pub struct MonitorManager {
    name: String,
    config: MonitorManagerConfig,
    dump_writer: csv::Writer<fs::File>,
    afl_status_monitor: AflStatusMonitor,
    llvm_cov_monitor: LLVMCoverageMonitor,
}

#[derive(Clone)]
pub struct MonitorManagerConfig {
    output_csv: String,
    afl_status_file: String,
    llvm_exec_path: String,
    llvm_prof_dir: String,
}

pub struct MonitorManagerResult {
    name: String,
    timestamp: String,
    afl_status: AflStautsResult,
    llvm_cov: LLVMCoverageResult,
}

impl MonitorManager {
    pub fn new(name: String, config: MonitorManagerConfig) -> Self {
        let csv_file = fs::File::open(config.output_csv.clone()).expect("Open csv file");
        let mut wtr = csv::Writer::from_writer(csv_file);
        wtr.write_record([
            "name",
            "timestamp",
            "case_num",
            "cycle_num",
            "crash_num",
            "edge_covered",
            "edge_percent",
            "region_cov_percent",
            "func_cov_percent",
            "line_cov_percent",
            "branch_cov_percent",
        ])
        .expect("Failed to write column header to csv file");
        wtr.flush()
            .expect("Failed to flush column header to csv file");

        Self {
            name,
            config: config.clone(),
            dump_writer: wtr,
            afl_status_monitor: AflStatusMonitor::new(config.afl_status_file),
            llvm_cov_monitor: LLVMCoverageMonitor::new(config.llvm_exec_path, config.llvm_prof_dir),
        }
    }

    pub fn get_result(&self) -> MonitorManagerResult {
        let now = chrono::Local::now().format(DATETIME_FORMAT_STR).to_string();
        MonitorManagerResult {
            name: self.name.clone(),
            timestamp: now,
            afl_status: self.afl_status_monitor.get_result(),
            llvm_cov: self.llvm_cov_monitor.get_result(),
        }
    }

    pub fn dump_to_csv(&mut self) {
        let res = self.get_result();
        self.dump_writer
            .write_record(&[
                res.name,
                res.timestamp,
                res.afl_status.case_num.to_string(),
                res.afl_status.cycle_num.to_string(),
                res.afl_status.crash_num.to_string(),
                res.afl_status.edge_covered.to_string(),
                res.afl_status.edge_percent.to_string(),
                res.llvm_cov.region_cov.percent.to_string(),
                res.llvm_cov.func_cov.percent.to_string(),
                res.llvm_cov.line_cov.percent.to_string(),
                res.llvm_cov.branch_cov.percent.to_string(),
            ])
            .expect("Failed to write record to csv file");

        self.dump_writer
            .flush()
            .expect("Failed to flush record to csv file");
    }
}

impl MonitorManagerConfig {
    pub fn new(
        output_csv: String,
        afl_status_file: String,
        llvm_exec_path: String,
        llvm_prof_dir: String,
    ) -> Self {
        Self {
            output_csv,
            afl_status_file,
            llvm_exec_path,
            llvm_prof_dir,
        }
    }
}
