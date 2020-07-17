#[macro_use]
extern crate log;
extern crate log4rs;
use serde_json::{Result, Value};
use std::process::{Command, Stdio};
use std::io::{self, Write};
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

fn main() -> Result<()> {
//n main() -> Result<()>{
    let conf = Ini::load_from_file("conf.ini").unwrap();
    let section = conf.section(Some("User")).unwrap();
    let gh_user = section.get("user_id").unwrap();;
    let gh_pass = section.get("password").unwrap();;
    let level = log::LevelFilter::Info;
    let file_path = "Log/rust.log";
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build(file_path).unwrap();

    let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(level)))
            .build("stderr", Box::new(stderr)),
    )
    .build(
        Root::builder()
            .appender("logfile")
            .appender("stderr")
            .build(LevelFilter::Trace),
    )
    .unwrap();
    
    let _handle = log4rs::init_config(config);

    info!("Started");

    let section = conf.section(Some("Jira")).unwrap();
    let request_url = section.get("base_url").unwrap();

    info!("before JIRA Call");

    let  response = reqwest::blocking::Client::new()
        .get(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .query(&[("jql","project = \"SCI Marketplace Production Support\" AND status  in (closed, Cancelled, Resolved, \"Awaiting User Confirmation\", \"Awaiting User Confirmation - 1st Follow Up\", \"Awaiting User Confirmation - 2nd Follow Up\") AND issuetype = \"L2 Incident\" AND component != \"Fast Track - Old\" AND status != Duplicate AND resolved >= startOfDay(-2) and resolved <= endOfDay() and \"MPS RCA\" = Miscellaneous"),("fields","key,summary,assignee")])
        .send();
    
        match response{
            Ok(x) => {match x.bytes(){
                Ok(z) => {
                    info!("Reading response from Jira");
                    let j: Value = serde_json::from_slice(&z).unwrap();
                    
                    let mut initial_string: String = "".to_owned();
                    for item in j["issues"].as_array().unwrap() {
                        //println!("{:?}\n", item);
                        //println!("{:?} - {:?} - {:?}\n",item["key"], item["fields"]["summary"],item["fields"]["assignee"]["displayName"]);
                        initial_string.push_str(&item["key"].to_string());
                        initial_string.push_str(&"$$$".to_string());
                        initial_string.push_str(&item["fields"]["summary"].to_string());
                        initial_string.push_str(&"$$$".to_string());
                        initial_string.push_str(&item["fields"]["assignee"]["displayName"].to_string());
                        initial_string.push_str(&"\n".to_string());
                    }
                    info!("Read the data {}",initial_string);
                    println!("{:?}", initial_string);
                    info!("Calling Shell");
                    let mut output = Command::new("cmd")
                            .args(&["/C", "python send.py"])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("failed to execute process");
                    {
                        let stdin = output.stdin.as_mut().expect("Failed to open stdin");
                        stdin.write_all(initial_string.as_bytes()).expect("Failed to write to stdin");
                    }

                    info!("Reading STD Out");
                    let s  = output.wait_with_output().expect("Failed to read stdout");
                    println!("{:?}", io::stdout().write_all(&s.stdout).unwrap());
                },
                Err(e) => {
                    warn!("Error Calling reading response. error message {:?}", e);
                    println!("Error while reading")}
            }},
            Err(e) => {
                warn!("Error Calling jira.  - error message {:?}",e);
                println!("Error")}
        }
        //println!("Created {:?}", response);

    Ok(())
}