extern crate tokio;
extern crate tokio_core;
extern crate mio_uds;
extern crate capnp;
extern crate capnpc;
extern crate capnp_futures;
extern crate futures;

pub mod addressbook_capnp;

/// Generates the Rust code from the CapnProto schema.
fn main() {
    ::capnpc::CompilerCommand::new()
        .file("addressbook.capnp")
        .run()
        .unwrap();
}