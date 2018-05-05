#[macro_use] extern crate log;
extern crate env_logger;
extern crate tokio;
extern crate tokio_io;
extern crate capnp_schema;
extern crate capnp;
extern crate capnp_futures;
extern crate bytes;
extern crate futures;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;
use capnp_schema::addressbook_capnp::{address_book, person};
use capnp_futures::serialize::Transport;
use capnp::message::ReaderOptions;
use tokio::net::TcpStream;

fn main() {
    env_logger::init();

    // Bind the server's socket.
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");

    // First prepare the echo server, taken directly from Tokio's examples.
    let echo_server = listener.incoming()
        .map_err(|e| error!("accept failed = {:?}", e))
        .for_each(|sock| {
            info!("Connection received.");
            let (reader, writer) = sock.split();

            let bytes_copied = copy(reader, writer);

            let handle_conn = bytes_copied.map(|amt| {
                info!("Closed connection, wrote {:?} bytes", amt)
            }).map_err(|err| {
                error!("IO error {:?}", err)
            });

            tokio::spawn(handle_conn)
        });

    // Then prepare the client future.
    let capnp_client = TcpStream::connect(&addr)
        .and_then(|stream| {
            info!("Connection established");

            // Set up the CapnProto transport.
            let transport = Transport::new(stream, ReaderOptions::new());
            let (t_writer, t_reader) = transport.split();

            // Now, send an addressbook.
            let mut m = capnp::message::Builder::new_default();
            populate_address_book(m.init_root());

            let sent = t_writer.send(m)
                .map(|_|{})
                .map_err(|err|{
                    format!("Sending error: {}", err).to_string()
                });

            // Wait for the addressbook to be sent back.
            let received = t_reader
                // This will make the client stop once it has received and processed its first message
                .into_future()
                .and_then(|(first, _rest)|{
                    info!("Received");

                    let first = first.unwrap();
                    let address_book = first.get_root::<address_book::Reader>().unwrap();
                    read_address_book(address_book);

                    future::ok(())
                })
                .map_err(|_err|{
                    "Reception error".to_string()
                });

            sent
                .join(received)
                .map(|_|{})
                .map_err(|err|{
                    panic!("An unexpected error occurred:  {}", err);
                })
        })
        .map_err(|err|{
            error!("Client error: {}", err)
        })
    ;

    // Run both futures, the whole runtime should stop once
    info!("Starting.");
    tokio::run(
        echo_server
            .select(capnp_client)
            .then(|_|{future::ok(())}))
}

/// Inserts sample entries into the address book.
/// Taken from the CapnProto examples.
fn populate_address_book(address_book: address_book::Builder) {
    let mut people = address_book.init_people(2);
    {
        let mut alice = people.reborrow().get(0);
        alice.set_id(123);
        alice.set_name("Alice");
        alice.set_email("alice@example.com");
        {
            let mut alice_phones = alice.reborrow().init_phones(1);
            alice_phones.reborrow().get(0).set_number("555-1212");
            alice_phones.reborrow().get(0).set_type(person::phone_number::Type::Mobile);
        }
        alice.get_employment().set_school("MIT");
    }

    {
        let mut bob = people.get(1);
        bob.set_id(456);
        bob.set_name("Bob");
        bob.set_email("bob@example.com");
        {
            let mut bob_phones = bob.reborrow().init_phones(2);
            bob_phones.reborrow().get(0).set_number("555-4567");
            bob_phones.reborrow().get(0).set_type(person::phone_number::Type::Home);
            bob_phones.reborrow().get(1).set_number("555-7654");
            bob_phones.reborrow().get(1).set_type(person::phone_number::Type::Work);
        }
        bob.get_employment().set_unemployed(());
    }
}

/// Read the address book and check that its entries match the sample.
/// Taken from the CapnProto examples.
fn read_address_book(address_book: address_book::Reader) {
    let people = address_book.get_people().unwrap();
    assert_eq!(people.len(), 2);
    let alice = people.get(0);
    assert_eq!(alice.get_id(), 123);
    assert_eq!(alice.get_name().unwrap(), "Alice");
    assert_eq!(alice.get_email().unwrap(), "alice@example.com");

    let bob = people.get(1);
    assert_eq!(bob.get_id(), 456);
    assert_eq!(bob.get_name().unwrap(), "Bob");
    info!("Valid");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run(){
        main();
    }
}