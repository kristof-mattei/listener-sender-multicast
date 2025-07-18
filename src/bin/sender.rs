//
// Simple sender.c program for UDP
//
// Adapted from:
// http://ntrg.cs.tcd.ie/undergrad/4ba2/multicast/antony/example.html
//
// Changes:
// * Compiles for Windows as well as Linux
// * Takes the port and group on the command line
//
// Note that what this program does should be equivalent to NETCAT:
//
//     echo "Hello World" | nc -u 239.255.255.250 1900

use std::ffi::CString;
use std::net::SocketAddrV4;
use std::thread;
use std::time::{Duration, SystemTime};

use clap::Parser as _ ;
use clap::error::ContextKind;
use color_eyre::{Section as _ , eyre};
use listener_sender_multicast::Cli;
use socket2::{Domain, Socket, Type};

fn main() -> Result<(), eyre::Error> {
    let cli = Cli::try_parse();

    let cli = match cli {
        Ok(cli) => cli,
        Err(mut error) => {
            error.insert(
                ContextKind::Suggested,
                clap::error::ContextValue::StyledStrs(vec![
                    "Command line args should be multicast group and port, (e.g. for SSDP, `sender 239.255.255.250 1900`)".into(),
                ]),
            );

            error.exit();
        },
    };

    // !!! If test requires, make these configurable via args
    let delay_secs = 1;

    let fd = Socket::new(Domain::IPV4, Type::DGRAM, None).with_note(|| "socket")?;

    // set up destination address
    let addr = SocketAddrV4::new(cli.group, cli.port.into());

    // now just sendto() our destination!
    loop {
        let message = CString::new(format!(
            "Hello, World!, this message was sent at {}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Before epoch?")
                .as_secs()
        ))
        .unwrap();

        fd.send_to(message.to_bytes(), &addr.into())
            .with_note(|| "sendto")?;

        thread::sleep(Duration::from_secs(delay_secs));
    }
}
