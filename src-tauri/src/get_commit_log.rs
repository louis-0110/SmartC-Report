// use chrono::{DateTime, Duration, Local};
use crate::parser_xml::parser;
use regex::Regex;
use std::{process::Command, thread};

pub fn get_log(path: Vec<String>, current_date: String) -> Vec<String> {
    // let local: DateTime<Local> = Local::now();
    // let tomorrow = local + Duration::days(1);
    // let current_date = format!(
    //     "{{{}}}:{{{}}}",
    //     local.format("%Y-%m-%d"),
    //     tomorrow.format("%Y-%m-%d"),
    // );
    // svn log -r {2023-11-07}:{2023-12-10} | awk '/^r[0-9]+ \|/ {getline; getline; print}'
    let receiver = thread::spawn(move || {
        let logs = path.iter().map(|p| {
            Command::new("/usr/local/bin/svn")
                .args(["log", "--xml", &p, "-r", &current_date])
                .output()
        });
        let mut temp: Vec<String> = Vec::new();
        for item in logs.into_iter() {
            match item {
                Ok(log) => {
                    let binding = String::from_utf8_lossy(&log.stdout);
                    let mut log_text = parser(&binding);
                    temp.append(&mut log_text)
                }
                Err(err) => temp.push(err.to_string()),
            }
        }

        temp
    });
    receiver.join().expect("asd")

    // println!("{:?}", commits_text);
}

fn _parse_log<'a>(log: &'a str) -> &'a str {
    let re = Regex::new(r"\n\n(.*?)\n").unwrap();
    let mut commit_log = "";
    if let Some(captures) = re.captures(log) {
        // 提取匹配的文本
        if let Some(matched_text) = captures.get(1).as_mut() {
            commit_log = matched_text.as_str();
        }
    }
    commit_log
}
