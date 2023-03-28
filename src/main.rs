#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Error, Write};
use std::{str, thread};

use std::fmt;

struct SliceDisplay<'a, T: 'a>(&'a [T]);


impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {

            if !first {
                write!(f, ",{}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageSerialized {
    value: f64,
    best_vector: Vec<f64>
}

static mut BEST_GLOBAL_VALUE: f64= 10000000000.0 ;
static mut BEST_GLOBAL_VECTOR: Vec<f64> = vec![];

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    //println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);
 
    loop {
        data.clear();

        let bytes_read = stream.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let input: MessageSerialized = serde_json::from_slice(&data)?;

        let value = input.value;
        
        if unsafe { value < BEST_GLOBAL_VALUE}{
            println!("Actualizamos mejor valor");
            unsafe { BEST_GLOBAL_VALUE = value};
            unsafe {BEST_GLOBAL_VECTOR = input.best_vector};
            println!("Mejor valor al momento {}", unsafe { BEST_GLOBAL_VALUE});
            println!("Mejor vector {}", SliceDisplay(unsafe {&BEST_GLOBAL_VECTOR}));

        }

        //write!(stream.get_mut(), "{}", f64::from(value))?;
        unsafe { write!(stream.get_mut(), "{}", SliceDisplay(&BEST_GLOBAL_VECTOR))?};
        write!(stream.get_mut(), "{}", "\n")?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }

}