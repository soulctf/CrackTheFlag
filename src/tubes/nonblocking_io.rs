#![feature(io)]

use std::io::BufReadExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::sync_channel(bound: u128);
use std::thread;

// Struct for defining what is recieved from other programs(Commands & Arguments of those commands)
// #[derive(Clone)]
// enum EnVars {
//     command(String),
//     args(Vec<String>)
// }
// Creates an enum for differentiating types for stdin and the expected output as a child object
enum ByteStream {
    Child(Vec<u8>),
    Stdin(Vec<u8>)
}

fn main() {
    // Grabs the command
    let cmd = Command::new("bash")
    // Grabs the arguments
        .arg("-c")
        .arg("for i in $(seq 1 3); do sleep 1; echo line $i;done")
        // Grabs stdout as data piped from the program
        .stdout(Stdio::piped())
        // Spawns the process and unwraps the data recieved via stdout
        .spawn().unwrap();

    // Creates a temporary storage for unwrapped data recieved from stdout(child object)
    let child_buf = std::io::BufReader::new(cmd.stdout.unwrap());

    // Creates a synchronous streaming channel(sender, reciever)(send data, recieve data)
    let (sd, rd) = sync_channel(1);
    // Clone the sender as the port to send the child data through
    let sd_child = sd.clone();

    // Spawn a detached thread(allows it to outlive the parent)
    thread::spawn(move || {
        // Loops through the child buffer and looks for any data(including None)
        for data in child_buf.lines() {
            // Matches data to any data found and makes sure it is of the specified type issued through the enum
            let line = match data {
                Ok(line) => ByteStream::Child(line),
                Err(e) => {println!("Child error: {}", e); break;}
            };
            // Sends the lines of unwrapped child data through the channel its receiver
            sd_child.send(line).unwrap()
        }
    }); 

    // Repeats the previous process on another thread except based upon the global stdin global buffer
    thread::spawn(move || {
        // Constructs a handle to the standard input from the current process
        let stdin = std::io:stdin();
        // Manually locks the global buffer returned through each instance of the handle
        let stdin_buf = stdin.lock()
        
        for data in stdin_buf.lines() {
            let line = match data {
                Ok(line) => ByteStream::Stdin(line),
                Err(e) => {println!("Error: {}", e); break;}
            };
            // Sends the lines of unwrapped stdin data through the channel to its reciever
            sd.send(line).unwrap();
        }
    });

    // Iterates through the data recieved into rx and matches the results
    for data in rx.iter() {
        match data {
            ByteStream::Child(line) => println!("CHILD: {}", line),
            ByteStream::Stdin(line) => println!("STDIN: {}", line),
        }
    }


}