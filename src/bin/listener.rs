//
// Simple listener.c program for UDP multicast
//
// Adapted from:
// http://ntrg.cs.tcd.ie/undergrad/4ba2/multicast/antony/example.html
//
// Changes:
// * Compiles for Windows as well as Linux
// * Takes the port and group on the command line
//

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, SocketAddrV4};

const MSGBUFSIZE: usize = 256;

use clap::Parser as _;
use clap::error::ContextKind;
use color_eyre::{Section as _, eyre};
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
                    "Command line args should be multicast group and port, (e.g. for SSDP, `listener 239.255.255.250 1900`)".into(),
                ]),
            );

            error.exit();
        },
    };

    let fd = Socket::new(Domain::IPV4, Type::DGRAM, None).with_note(|| "socket")?;

    fd.set_reuse_address(true)
        .with_note(|| "Reusing addr failed")?;

    // set up receive address
    // IP address differs from sender
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, cli.port.into());

    if let Err(err) = fd.bind(&addr.into()) {
        Err(err).with_note(|| "bind")?;
    }

    // check the readme for SSM instructions
    fd.join_multicast_v4(&cli.group, &Ipv4Addr::UNSPECIFIED)
        .with_note(|| "setsockopt")?;

    loop {
        let mut buffer = [const { MaybeUninit::uninit() }; MSGBUFSIZE];

        let (received, from) = fd.recv_from(&mut buffer).with_note(|| "recvfrom")?;

        let null_pos = if received >= MSGBUFSIZE {
            MSGBUFSIZE - 1
        } else {
            received
        };

        buffer[null_pos].write(b'\0');

        // SAFETY: `Socket::recv_from` promises not to write any uninitialized bytes.
        // And we wrote the final `\0`, meaning we can include that last byte (`=null_pos`).
        let initialized_part = unsafe { &*((&raw const buffer[..=null_pos]) as *const [u8]) };

        let message = CStr::from_bytes_until_nul(initialized_part)?.to_str()?;

        let from = from
            .as_socket()
            .expect("We're listening to AF_INET, so we can only receive from other sockets.");

        println!("{:?} -> {}", from, message);
    }
}
