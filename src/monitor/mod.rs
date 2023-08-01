pub use manager::{MonitorManager, MonitorManagerConfig};

pub use afl_status::{AflStatusMonitor, AflStautsResult};
pub use llvm_cov::{LLVMCoverageMonitor, LLVMCoverageResult};

mod afl_status;
mod llvm_cov;
mod manager;
