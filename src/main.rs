// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rust_lsp;

mod mock_ls;

use std::env;
use std::io;
use std::io::Write;

use log::LogLevelFilter;
use env_logger::LogBuilder;

fn main() {
	
	// Prepare log 
    let mut builder = LogBuilder::new();
    // Set info as default log level
    builder.filter(None, LogLevelFilter::Info);
	
	if let Ok(rustlog_env_var) = env::var("RUST_LOG") {
		builder.parse(&rustlog_env_var);
	}
    builder.init().unwrap();
	
	
    info!("Starting example server.");
    
	if env::args().len() == 1  {
		// Use stdin/stdout
		
		let stdin = std::io::stdin();
		mock_ls::run_lsp_server(&mut stdin.lock(), move || std::io::stdout());
	} else {
		let mut args = env::args();
		args.next();
		let mut port_str = args.next().unwrap();
		
		info!("starting server on port: {}", port_str);
	
		// Workaround for a CDT-GDB bug on Windows that adds single quotes to params
		if port_str.starts_with("'") && port_str.ends_with("'"){
			port_str = port_str[1..port_str.len()-1].to_string();
		}
		
		let port : u16 = port_str.parse::<u16>().expect(&format!("Invalid port number: {}", port_str));
		
		let listener = TcpListener::bind(("127.0.0.1", port)).unwrap(); //FIXME - unwrap
		tcp_server(listener);
	}
	
}

use std::net::TcpListener;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread;


fn tcp_server(listener: TcpListener) {
	
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move|| {
					handle_client(stream)
				});
			}
			Err(err) => {
				writeln!(&mut io::stderr(), "TCP listen error : {:?}", err).expect("Failed writing to stderr");
			}
		}
	}
	
	drop(listener);
}

fn handle_client(stream: TcpStream) {
	//FIXME use same server for each connection
	
	let mut input = io::BufReader::new(stream.try_clone().expect("Failed to clone stream"));
	
	mock_ls::run_lsp_server(&mut input, || {
		stream
	});
}
