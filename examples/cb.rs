extern crate circuitbreaker_rs as cb;
extern crate hyper;

use cb::{CircuitBreaker, Command, CommandResult};
use cb::strategy::count::CountStrategy;
use hyper::client::Client;

use std::io::Read;
use std::{thread, time};

pub struct NetworkRequestCommand;

impl Command<String> for NetworkRequestCommand {
    fn execute(&self) -> CommandResult<String> {
        let client = Client::new();
        let mut response = try!(client.get("").send());
        let mut buffer = String::new();
        try!(response.read_to_string(&mut buffer));

        Ok(buffer)
    }
}

fn main() {
    let command = Box::new(NetworkRequestCommand);
    let strategy = Box::new(CountStrategy::new(5, 5000));
    let mut breaker = CircuitBreaker::new(command, strategy);
    for _ in 0..10 {
        println!("{:?}", breaker.execute());
    }

    thread::sleep(time::Duration::from_millis(7500));

    println!("{:?}", breaker.execute());
}
