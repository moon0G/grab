use std::{
    io::{prelude::*, BufReader, Result},
    net::{TcpListener, TcpStream},
    env,
    process,
    fs,
};

fn main() {
    let args: Vec<_>= env::args().collect();
    if args.len() == 1 {
        println!("No Args Supplied");
        process::exit(1)
    }

    let mut addr = "";
    let mut port = "";
    let mut aflag: bool = false;
    let mut pflag: bool = false;

    if args[1] == "-s" {
        println!("Setting Up Server...");
        let mut fflag: bool = false;
           let mut _file = "";
                for mut i in 1..args.len() {
            if args[i] == "-f" && fflag != true {
                i+=1;
                _file = &args[i];
                fflag = true;
            }
            if args[i] == "-a" && aflag != true {
                i+=1;
                addr = &args[i];
                aflag = true;
            }
            if args[i] == "-p" && pflag != true {
                i+=1;
                port = &args[i];
                pflag = true;
            }
        }

        if fflag == false || aflag == false || pflag == false {
            println!("Error, not enough flags supplied.");
            process::exit(1)
        }

        //println!("File: {}, Address: {}, Port: {}", _file, addr, port);


        let file_cont: Vec<u8> = readFile(_file).unwrap();
         
        //println!("{}", String::from_utf8_lossy(&fileCont));

        let listener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New Connection: {:?}", stream.peer_addr());

                    stream.write_all(&file_cont).unwrap();

                    println!("File contents sent to {:?}", stream.peer_addr());
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }

    } else if args[1] == "-c" {
        println!("Setting Up Client...");
        let mut output = "";
        let mut oflag = false;

        for mut i in 1..args.len() {
             if args[i] == "-a" && aflag != true {
                i+=1;
                addr = &args[i];
                aflag = true;
            }
            if args[i] == "-p" && pflag != true {
                i+=1;
                port = &args[i];
                pflag = true;
            }
            if args[i] == "-o" && oflag != true {
                i+=1;
                output = &args[i];
                oflag = true;
            }
        }
        if aflag == false || pflag == false || oflag == false {
            println!("Error, not enough flags supplied.");
            process::exit(1)
        }

        let mut stream = TcpStream::connect(format!("{}:{}", addr, port)).unwrap();

        let mut file_contents = Vec::new();
        stream.read_to_end(&mut file_contents).unwrap();
        fs::write(output, &file_contents);
    }
}

fn readFile(fileName: &str) -> Result<Vec<u8>> {
     match fs::read(fileName) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e),
     }
}
