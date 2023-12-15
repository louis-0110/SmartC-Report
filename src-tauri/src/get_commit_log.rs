// use chrono::{DateTime, Duration, Local};
use crate::{parser_xml::parser, read_file::read_conf};
// use regex::Regex;
use std::{
    fs,
    io::{self, Error},
    os::windows::process::CommandExt,
    process::{Command, Output},
    thread,
};

pub fn get_log(path: Vec<String>, current_date: [String; 2]) -> Vec<String> {
    let settings = read_conf();
    // svn log --xml <PATH> -r {2023-11-07}:{2023-12-10}'
    let receiver = thread::spawn(move || {
        let logs = path.iter().map(|p| match is_git_or_svn(p) {
            VersionTool::GIT => (VersionTool::GIT, log_git(p, &current_date)),
            VersionTool::SVN => (
                VersionTool::SVN,
                log_svn(&settings.svn_path, p, &current_date),
            ),
            VersionTool::NOTFOUND => (
                VersionTool::NOTFOUND,
                Result::Err(io::Error::new(
                    io::ErrorKind::Other,
                    "not found version tool",
                )),
            ),
        });

        let mut temp: Vec<String> = Vec::new();
        for (vt, item) in logs.into_iter() {
            match item {
                Ok(log) => {
                    let binding = String::from_utf8_lossy(&log.stdout);

                    let mut log_text = match vt {
                        VersionTool::SVN => parser(&binding),
                        VersionTool::GIT => binding
                            .to_string()
                            .split("\n")
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect(),
                        VersionTool::NOTFOUND => vec![binding.to_string()],
                    };
                    temp.append(&mut log_text)
                }
                Err(err) => temp.push(err.to_string()),
            }
        }
        temp
    });

    receiver.join().expect("error: get log")
}

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;
fn log_svn(
    command_path: &str,
    entrepot_path: &str,
    current_date: &[String; 2],
) -> Result<Output, Error> {
    let mut command = Command::new(command_path);
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    command
        .args([
            "log",
            "--xml",
            entrepot_path,
            "-r",
            &format!("{{{}}}:{{{}}}", current_date[0], current_date[1]),
        ])
        .output()
}

fn log_git(entrepot_path: &str, current_date: &[String; 2]) -> Result<Output, Error> {
    //git log --pretty=format:"%s" --since={2023-02-16}
    let args = format!(
        "cd {} && git log --pretty=format:\"%s\" --since={} --until={}",
        entrepot_path, current_date[0], current_date[1]
    );
    //current_date,
    if cfg!(windows) {
        let mut command = Command::new("cmd");
        #[cfg(target_os = "windows")]
        command.creation_flags(CREATE_NO_WINDOW);
        command.arg("/C").arg(args).output()
    } else {
        let mut command = Command::new("sh");
        #[cfg(target_os = "windows")]
        command.creation_flags(CREATE_NO_WINDOW);
        command.arg("-c").arg(args).output()
    }
}

// 判断当前路下是否存在.git 或 .svn 文件夹

enum VersionTool {
    GIT,
    SVN,
    NOTFOUND,
}

fn is_git_or_svn(entrepot_path: &str) -> VersionTool {
    if let Ok(entries) = fs::read_dir(entrepot_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_name().to_str().unwrap() == ".git" {
                    return VersionTool::GIT;
                } else if entry.file_name().to_str().unwrap() == ".svn" {
                    return VersionTool::SVN;
                }
            }
        }
    }
    VersionTool::NOTFOUND
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_git_log() {
        let current_date = ["2023-02-16".to_string(), "2023-02-16".to_string()];
        let entrepot_path = "/Users/gaoluo/projects/SmartC-Report";
        let output = log_git(entrepot_path, &current_date);
        println!("{:?}", output);
    }
}
