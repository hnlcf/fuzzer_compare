use std::{ffi::OsStr, process::Stdio};

use crate::constants::NPROCS;

pub struct ShellCommand {
    cmd: std::process::Command,
}

impl Default for ShellCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellCommand {
    pub fn new() -> Self {
        let cmd = std::process::Command::new("/bin/sh");
        ShellCommand { cmd }
    }

    pub fn args<S: AsRef<str>>(&mut self, args: &[S]) -> &mut ShellCommand {
        let args = args
            .iter()
            .map(|e| e.as_ref())
            .collect::<Vec<&str>>()
            .join(" ");

        self.cmd.arg("-c").arg(&args);

        self
    }

    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, cwd: P) -> &mut ShellCommand {
        self.cmd.current_dir(cwd);
        self
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut ShellCommand
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.cmd.envs(vars);
        self
    }

    pub fn pipe_stdio(&mut self) -> &mut ShellCommand {
        self.cmd
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped());
        self
    }

    pub fn spawn(&mut self) -> Result<std::process::Child, Box<dyn std::error::Error>> {
        let child = self.cmd.spawn()?;

        Ok(child)
    }
}

pub fn create_dir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

pub fn create_new_dir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(path).exists() {
        remove_dir(path)?;
    }

    create_dir(path)
}

pub fn remove_dir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(path)?;

    Ok(())
}

pub fn git_clone(
    url: &str,
    target_location: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = vec!["git", "clone", "--recursive", url];
    if let Some(location) = target_location {
        cmd.push(location);
    }

    let output = ShellCommand::new()
        .args(&cmd)
        .current_dir(".")
        .spawn()?
        .wait_with_output()?;
    for l in String::from_utf8(output.stdout)?.lines() {
        println!("{:?}", l);
    }

    Ok(())
}

pub fn cmake_config<S: AsRef<str>>(
    src_dir: &str,
    bin_dir: Option<&str>,
    cmake_vars: Option<&[S]>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = if let Some(inner) = bin_dir {
        inner.to_owned()
    } else {
        format!("{}/build", src_dir)
    };

    create_dir(&bin_dir)?;

    let mut cmd = vec![
        "cmake",
        "-S",
        src_dir,
        "-B",
        &bin_dir,
        "-DCMAKE_BUILD_TYPE=Release",
        "-DCMAKE_C_COMPILER=/usr/bin/clang",
        "-DCMAKE_CXX_COMPILER=/usr/bin/clang++",
    ];
    if let Some(inner) = cmake_vars {
        let inner = inner.iter().map(|e| e.as_ref()).collect::<Vec<&str>>();
        cmd.extend_from_slice(&inner);
    }

    ShellCommand::new()
        .args(&cmd)
        .current_dir(src_dir)
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

pub fn cmake_build<S: AsRef<str>>(
    src_dir: &str,
    bin_dir: Option<&str>,
    build_args: Option<&[S]>,
    target: Option<&str>,
    nproc: Option<i8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = if let Some(inner) = bin_dir {
        inner.to_owned()
    } else {
        format!("{}/build", src_dir)
    };

    if !std::path::Path::new(&bin_dir).exists() {
        panic!("Build directory {} is not exist.", bin_dir);
    }

    let nproc = match nproc {
        Some(inner) if inner <= NPROCS => inner.to_string(),
        _ => NPROCS.to_string(),
    };

    let mut cmd = vec![
        "cmake",
        "--build",
        &bin_dir,
        "--target",
        target.unwrap_or("all"),
        "-j",
        &nproc,
    ];

    if let Some(inner) = build_args {
        let inner = inner.iter().map(|e| e.as_ref()).collect::<Vec<&str>>();
        cmd.extend_from_slice(&inner);
    }

    ShellCommand::new()
        .args(&cmd)
        .current_dir(src_dir)
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_command() -> Result<(), Box<dyn std::error::Error>> {
        let cmd = vec!["ls", "-l", "-a"];
        let output = ShellCommand::new()
            .args(&cmd)
            .current_dir(".")
            .spawn()?
            .wait_with_output()?;

        for l in String::from_utf8(output.stdout)?.lines() {
            println!("{:?}", l);
        }

        Ok(())
    }
}
