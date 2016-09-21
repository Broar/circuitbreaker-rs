extern crate circuitbreaker_rs as cb;
extern crate hyper;

use cb::{CircuitBreaker, DefaultCircuitBreaker, Command, CommandResult};
use cb::strategy::Strategy;
use cb::strategy::count::CountStrategy;
use hyper::client::Client;
use hyper::status::StatusClass;
use hyper::error::Error;

use std::io::Read;
use std::{thread, time};
use std::sync::{Arc, Mutex};

pub struct NetworkRequestCommand;

impl Command<String> for NetworkRequestCommand {
    fn execute(&self) -> CommandResult<String> {
        let client = Client::new();
        let mut response = try!(client.get("http://httpstat.us/500").send());
        let mut buffer = String::new();
        try!(response.read_to_string(&mut buffer));

        match response.status.class() {
            StatusClass::ClientError | StatusClass::ServerError => Err(Box::new(Error::Status)),
            _ => Ok(buffer)
        }
    }
}

pub struct NetworkRequestFallback;

impl Command<String> for NetworkRequestFallback {
    fn execute(&self) -> CommandResult<String> {
        Ok("Hello, fallback!".to_string())
    }
}

fn main() {
    let command = NetworkRequestCommand;
    let fallback = NetworkRequestFallback;
    let strategy = CountStrategy::new(5, 5000);
    let mut breaker = Arc::new(Mutex::new(DefaultCircuitBreaker::new(command, Some(fallback), strategy)));

    let mut threads = vec![];
    for _ in 0..10 {
        let mut breaker = breaker.clone();
        let t = thread::spawn(move || {
            let mut breaker = breaker.lock().unwrap();
            println!("{:?}", breaker.execute());
        });

        threads.push(t);
    }

    thread::sleep(time::Duration::from_millis(7500));

    for t in threads {
        let _ = t.join();
    }

    println!("{:?}", breaker.lock().unwrap().execute());
}
