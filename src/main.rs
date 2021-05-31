use std::env::args;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use crate::logger::Logger;
use crate::server::Server;
use crate::command::commands::Command;
use crate::structure_string2::StructureString;
use crate::structure_simple::StructureSimple;

mod command;
mod config_server;
mod logger;
mod threadpool;
mod server;
mod utils;
mod structure_string;
mod errors;
mod structure_string2;
mod structure_simple;

static THREAD_POOL_COUNT: usize = 4;

static END_FLAG: &str = "EOF";

const LOG_NAME: &str = "log";
const LOG_PATH: &str = "./";
const ERROR_LOG_CREATE: &str = "Error creating Logger";

fn main() -> Result<(), std::io::Error> {
    let argv = args().collect::<Vec<String>>();
    let mut server = Server::new(argv.clone());

    server.load_config(argv)?;

    //println!("Serger args: {:?}", &argv);
    println!("Server {} is up!", server.server_name());

    let config_server = server.get_config_server();

    let server_port = config_server.get_server_port(server.get_logger());

    println!();
    println!("Server address: {}", server_port);
    println!();

    println!("Execute listener ...");
    //let mut structure = Box::new(StructureString::new());

    let _listener = exec_server(&server_port, &server);

    Ok(())
}

fn exec_server<'a>(address: &String, server: &Server) -> Result<(), std::io::Error> {
    let threadpool = threadpool::ThreadPool::new(THREAD_POOL_COUNT.clone());

    let mut stt: & Arc<Mutex<HashMap<String,String>>> = &Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(&address)?;
    for stream in listener.incoming() {
        let stream = stream;
        println!("Handler stream request ...");
        let server = server.clone();
        let stream = stream?;
        let _id_global = -1;
        let stt_clone = Arc::clone(stt);

        threadpool.execute(move |  _id_global| {
            handle_connection(stream, &server, _id_global, & stt_clone);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, server: &Server, id: u32, structure: & Arc<Mutex<HashMap<String,String>>>) {
    handle_client(&mut stream, server, id,structure);
}

fn handle_client(stream: &mut TcpStream, server: &Server, id: u32, structure: & Arc<Mutex<HashMap<String,String>>>) {
    let stream_reader = stream.try_clone().expect("Cannot clone stream reader");
    let reader = BufReader::new(stream_reader);

    let mut lines = reader.lines();
    println!("Reading stream conections, job {} ...", id);

    while let Some(line) = lines.next() {
        let request = line.unwrap_or(String::from(END_FLAG));

        if request == END_FLAG {
            return;
        }

        println!("Server job {}, receive: {:?}", id, request);

        let response = process_request(request, server, id.clone(),structure);
        (*stream).write(response.as_bytes()).unwrap_or(0);
    }
    println!("End handle client, job {}", id);
}
//TODO: ver porque si vienen mal los args explota
fn process_request(request: String, server: &Server, id_job: u32, structure: & Arc<Mutex<HashMap<String,String>>>) -> String {

    //TODO: ver de meter el command_builder en el server.
    let mut command_builder = command::command_builder::CommandBuilder::new(id_job, server.get_logger());

    let mut comm = command_builder.get_command(&mut String::from(request.trim()));
    //TODO: ver porque si vienen mal los args explota
    let mut command_splited: Vec<& str> = request.split(" ").collect();
    command_splited.remove(0);

    match comm {
        Ok(comm) => comm.run(command_splited, structure).unwrap(),
        Err(comm) => comm.to_string(),
    }
}