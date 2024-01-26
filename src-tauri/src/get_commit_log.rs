use tauri::{window, Window};

use crate::{parser_xml::parser, read_file::read_conf};
use std::error::Error as StdError;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
    fs,
    io::{self, Error},
    process::{Command, Output},
    thread,
};

pub fn get_log(
    window: Window,
    path: Vec<String>,
    current_date: [String; 2],
) -> Result<Vec<String>, Box<dyn StdError>> {
    let settings = read_conf(&window);
    // svn log --xml <PATH> -r {2023-11-07}:{2023-12-10}'
    let receiver = thread::spawn(move || {
        let logs = path.iter().map(|p| {
            let tool = match is_git_or_svn(p) {
                Ok(x) => x,
                Err(e) => {
                    print!("error");
                    window.emit("error", e.to_string()).unwrap();
                    VersionTool::Notfound
                }
            };

            match tool {
                VersionTool::Git => (VersionTool::Git, log_git(p, &current_date)),
                VersionTool::Svn => (
                    VersionTool::Svn,
                    log_svn(&settings.svn_path, p, &current_date),
                ),
                VersionTool::Notfound => (
                    VersionTool::Notfound,
                    Result::Err(io::Error::new(
                        io::ErrorKind::Other,
                        "not found version tool",
                    )),
                ),
            }
        });

        let mut temp: Vec<String> = Vec::new();
        for (vt, item) in logs.into_iter() {
            match item {
                Ok(log) => {
                    let binding = String::from_utf8_lossy(&log.stdout);

                    let _ = window.emit("error", &binding);
                    let mut log_text = match vt {
                        VersionTool::Svn => parser(&binding),
                        VersionTool::Git => binding
                            .to_string()
                            .split('\n')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect(),
                        VersionTool::Notfound => vec![binding.to_string()],
                    };
                    temp.append(&mut log_text)
                }
                Err(err) => {
                    let _ = window.emit("error", &err.to_string());
                    temp.push(err.to_string())
                }
            }
        }
        temp
    });

    if let Ok(v) = receiver.join() {
        Ok(v)
    } else {
        return Err(Box::from("錯誤71"));
    }
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
        "cd {} ; git log --pretty=format:\"%s\" --since={} --until={}",
        entrepot_path, current_date[0], current_date[1]
    );
    let mut command: Command;

    if cfg!(windows) {
        command = Command::new("powershell");
        #[cfg(target_os = "windows")]
        command.creation_flags(CREATE_NO_WINDOW);
    } else {
        command = Command::new("sh");
    }
    command.arg("-c").arg(args).output()
}

// 判断当前路下是否存在.git 或 .svn 文件夹

enum VersionTool {
    Git,
    Svn,
    Notfound,
}

fn is_git_or_svn(entrepot_path: &str) -> Result<VersionTool, Box<dyn StdError>> {
    if let Ok(entries) = fs::read_dir(entrepot_path) {
        for entry in entries.map(|item| item.unwrap()) {
            if entry.file_name().to_str().ok_or(".git文件沒有找到")? == ".git" {
                return Ok(VersionTool::Git);
            } else if entry.file_name().to_str().ok_or("svn文件沒有找到")? == ".svn" {
                return Ok(VersionTool::Svn);
            }
        }
    }
    Ok(VersionTool::Notfound)
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
