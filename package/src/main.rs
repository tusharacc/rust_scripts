/*
    Arguments:
        1. root directory of hotfixes
        2. Hotfix
        3. Type: cimi or afm

    Return Code:
        1 => Invalid Arguments
        2 => Invaid Type
*/

use std::env;
use std::process;
use std::process::Command;

static DEBUG: bool = true;
static ENV: &str = "sit";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4{
        debug_message("Processing");
        let root = &args[1];
        let hf = &args[2];
        let hf_type = &args[3];
        process_hotfix(root,hf,hf_type);
    } else {
        print_message("Invalid Number of arguments");
        exit_program(10);
    }
}
fn process_hotfix(root:&str,hf:&str,hf_type:&str){
    
    let ini_path;
    if hf_type == "cimi"
    {
        ini_path = format!("{}\\InstallScripts\\INIFiles\\{}-CIMI-setup.ini",root,ENV);
    }
    else if hf_type == "afm"
    {
        ini_path = format!("{}\\InstallScripts\\INIFiles\\{}-AFMMetaDataImport.ini",root,ENV);
    } else {
        ini_path = "Invalid".to_string();
        print_message(&format!("Invalid Type Received {}",hf_type));
        exit_program(2);
    }

    let install_path = format!("{}\\Fixes\\{}",root,hf);
    let exe_path = format!("{}\\Application\\InsurityInstallManager\\InsurityInstallManager.exe",root);
    //let cmd = format!("{} /installpackage:{} /inifile:{}",exe_path,install_path,ini_path);
    debug_message(&format!("The install path is {}",install_path));
    debug_message(&format!("The exe path is {}",exe_path));
    debug_message(&format!("The ini path is {}",ini_path));
    let path = env::current_dir();
    println!("The current directory is {:?}", path);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .arg("/C")
                .arg(exe_path)
                .arg("/installpackage:")
                .arg(install_path)
                .arg("/inifile:")
                .arg(ini_path)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    
}

fn debug_message(msg:&str){
    if DEBUG{
        println!("{}",msg);
    }
}

fn print_message(msg:&str){
    println!("{}",msg);
}

fn exit_program(code:i32){
    process::exit(code);
}