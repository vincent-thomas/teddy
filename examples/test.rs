use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::unix::SourceFd;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

const STDIN_TOKEN: Token = Token(0);

fn main() -> io::Result<()> {
    let poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    
    // Register standard input for readability
    poll.register(
        &SourceFd(&0),
        STDIN_TOKEN,
        Ready::readable(),
        PollOpt::edge(),
    )?;

    let mut buffer = [0; 1024];
    
    loop {
        // Poll for events
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                STDIN_TOKEN => {
                    if let Ok(bytes_read) = io::stdin().read(&mut buffer) {
                        if bytes_read > 0 {
                            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
                            println!("You entered: {}", input);
                        }
                    }
                },
                _ => (),
            }
        }
    }
}
