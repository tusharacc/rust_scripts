/*

    Arguments:
    1. root folder: The path of hotfix

    Return Code:
    1 => Invalid Arguments
*/

use std::env;
use std::process;
use std::fs;
use std::path::{Path, PathBuf};

static DEBUG: bool = false;
static INSTALL_PATH: &str = "C:\\Program Files (x86)\\Insurity";

#[derive(Debug)]
struct Response {
    // The 'a defines a lifetime
    code: u8,
    messages: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2{
        debug_message("Processing");
        let root = &args[1];
        let mut elements:Vec<String> = Vec::new();

        copy_files(root,&mut elements);
    } else {
        print_message("Invalid Number of arguments");
        exit_program(10);
    }
    
}

fn copy_files(folder_path: &str, elements: &mut Vec<String>){
    
    debug_message(folder_path);
    let entry = fs::read_dir(folder_path);

    match entry {
        Ok(paths) => 
            for p in paths{
                let new_path =  p.unwrap().path();
                if new_path.is_dir(){
                    match new_path.to_str(){
                        Some(np) => {
                            let components = new_path.components();
                            let last = components.last();
                            match last{
                                Some(val) => {
                                    match val.as_os_str().to_str(){
                                        Some(v) => {
                                            elements.push(v.to_string());
                                            debug_message(&format!("Is Dir {:?}",elements));
                                        },
                                        None =>  {println!("Error ");}
                                    }
                                },
                                None => {println!("Error");}
                            }
                            copy_files(np,elements);
                            elements.pop();
                        },
                        _ => {
                            print_message(&format!("Error "));
                            exit_program(5);
                        }
                    }
                    
                } else {
                    debug_message(&format!("This is a file, before Popping {:?}",elements));
                    let root = Path::new(&INSTALL_PATH);
                    let mut dest_path = PathBuf::new();
                    dest_path.push(root);
                    for item in elements.iter(){
                        dest_path.push(item);
                    }
                    match new_path.file_name(){
                        Some(v) => {dest_path.push(v);},
                        None => {print_message("Something went wrong");}
                    }
                    
                    print_message(&format!("The destination path generated is {:?}",dest_path));
                    print_message(&format!("The Source is {:?}",new_path));
                    let result = fs::copy(new_path,dest_path);
                    match result{
                        Ok(_) => {print_message(&format!("Copy Successful"));},
                        Err(e) => {print_message(&format!("Copy Failed {:?}",e.kind()));}
                    }
                    debug_message(&format!("After Popping {:?}",elements));
                }
            }
        ,
        Err(e) => {print_message(&format!("{:?}",e.kind()));}
    }
            //print_message(&format!("The Entry is {:?}",entry))
        //}
    
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
