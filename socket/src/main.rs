//https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::process::Command;

#[derive(Debug)]
pub struct Payload{
    action: String,
    argument: String
}

pub trait ConvertToStruct{
    fn convert_from_bytes(inp:Vec<u8>) -> Payload;
}

impl ConvertToStruct for Payload{
    fn convert_from_bytes(inp:Vec<u8>) -> Payload{
        let mut payload = Payload{
            action:String::new(),
            argument: String::new()
        };

        let mut buffStr = String::new();
        match String::from_utf8(inp.clone()){
            Ok(val) => {
                buffStr = val;
            },
            Err(_) => {
                buffStr = "Err".to_string();
            }
        };
        payload.action = buffStr[..3].to_string();
        payload.argument = buffStr[3..].to_string();
        return payload;
    }
}

fn execute_iis(argument:&str) -> String{

    let arg:&str;
    let mut return_message:String;

    if argument.eq_ignore_ascii_case("stop"){
        arg = "/STOP"
    } else if argument.eq_ignore_ascii_case("start"){
        arg = "/START"
    } else if argument.eq_ignore_ascii_case("status"){
        arg = "/STATUS"
    } else {
        return String::from("Error");
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .arg("/C")
                .arg("IISRESET")
                .arg(arg)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    
    if !output.status.success() {
        println!("Command executed with failing error code");
        let err = output.stderr;
        return_message = String::from_utf8(err).unwrap();
    }

    let stdout = output.stdout;
    return_message = String::from_utf8(stdout).unwrap();

    return return_message;
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            //stream.write(&data[0..size]).unwrap();
            let payload = Payload::convert_from_bytes(data[0..size].to_vec());
            println!("The payload is {:?}", payload);
            if payload.action.eq_ignore_ascii_case("iis"){
                let iis_message:String = execute_iis(&payload.argument);
                println!("The message written is {:?}",iis_message);
                stream.write(iis_message.as_bytes()).unwrap();
            }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9999").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 9999");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {:?}", stream);
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}