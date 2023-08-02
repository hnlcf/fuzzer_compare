pub mod constants;
pub mod utils;

pub mod builder;
pub mod monitor;
pub mod runner;

use std::thread::JoinHandle;

use builder::{BoostBuilder, Builder, MariadbBuilder, MysqlBuilder, PgsqlBuilder, SquirrelBuilder};
use constants::{
    CONFIG_DIR, DB_DIR, FUZZERS_DIR, INSTALL_DIR, OUTPUT_DIR, ROOT, TEST_DIR, TMP_DIR,
};
use monitor::{MonitorManager, MonitorManagerConfig};
use runner::{Runner, SquirrelRunner};
use utils::ShellCommand;

use clokwerk::{Scheduler, TimeUnits};

fn pre_setup() -> Result<(), Box<dyn std::error::Error>> {
    utils::create_dir(TMP_DIR.as_str())?;
    utils::create_dir(DB_DIR.as_str())?;
    utils::create_dir(FUZZERS_DIR.as_str())?;
    utils::create_dir(INSTALL_DIR.as_str())?;
    utils::create_dir(CONFIG_DIR.as_str())?;
    utils::create_new_dir(TEST_DIR.as_str())?;
    utils::create_new_dir(OUTPUT_DIR.as_str())?;

    Ok(())
}

fn post_setup() -> Result<(), Box<dyn std::error::Error>> {
    let mv_config_cmd = [
        "cp".to_string(),
        format!("{}/squirrel/", ROOT.as_str()),
        CONFIG_DIR.to_string(),
    ];

    ShellCommand::new()
        .args(&mv_config_cmd)
        .current_dir(ROOT.as_str())
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    pre_setup()?;

    SquirrelBuilder::setup()?;
    PgsqlBuilder::setup()?;
    BoostBuilder::setup()?;
    MysqlBuilder::setup()?;
    MariadbBuilder::setup()?;

    post_setup()?;

    Ok(())
}

pub fn setup_monitor() -> Vec<MonitorManager> {
    let pg_monitor_config = MonitorManagerConfig::new(
        format!("{}/squirrel-pg.csv", OUTPUT_DIR.as_str()),
        format!("{}/squirrel-pg/test/1/fuzzer_stats", TEST_DIR.as_str()),
        format!("{}/postgresql/bin/postgres", INSTALL_DIR.as_str()),
        format!("{}/squirrel-pg/prof", TEST_DIR.as_str()),
    );

    let pg_monitor = MonitorManager::new("squirrel-pg".to_string(), pg_monitor_config);

    let my_monitor_config = MonitorManagerConfig::new(
        format!("{}/squirrel-my.csv", OUTPUT_DIR.as_str()),
        format!("{}/squirrel-my/test/1/fuzzer_stats", TEST_DIR.as_str()),
        format!("{}/mysql/bin/mysqld", INSTALL_DIR.as_str()),
        format!("{}/squirrel-my/prof", TEST_DIR.as_str()),
    );

    let my_monitor = MonitorManager::new("squirrel-my".to_string(), my_monitor_config);

    let ma_monitor_config = MonitorManagerConfig::new(
        format!("{}/squirrel-ma.csv", OUTPUT_DIR.as_str()),
        format!("{}/squirrel-ma/test/1/fuzzer_stats", TEST_DIR.as_str()),
        format!("{}/mariadb/bin/mariadbd", INSTALL_DIR.as_str()),
        format!("{}/squirrel-ma/prof", TEST_DIR.as_str()),
    );

    let ma_monitor = MonitorManager::new("squirrel-ma".to_string(), ma_monitor_config);

    vec![pg_monitor, my_monitor, ma_monitor]
}

pub fn monitor_loop(monitors: &mut Vec<MonitorManager>) {
    for m in monitors {
        m.dump_to_csv();
    }
}

pub fn launch_monitors() {
    let mut monitors = setup_monitor();
    let mut scheduler = Scheduler::with_tz(chrono::Local);

    scheduler
        .every(5.minutes())
        .run(move || monitor_loop(&mut monitors));
}

pub fn launch_fuzzers() -> Vec<JoinHandle<()>> {
    let pg_handle = std::thread::spawn(|| {
        let _res = SquirrelRunner::run_pgsql();
    });
    let ma_handle = std::thread::spawn(|| {
        let _res = SquirrelRunner::run_mariadb();
    });
    let my_handle = std::thread::spawn(|| {
        let _res = SquirrelRunner::run_mysql();
    });

    vec![pg_handle, my_handle, ma_handle]
}
