use std::{io, collections::HashMap};
use lightheader::packet::Packet;

mod tcp;

fn main() -> io::Result<()> {

    let mut connections: HashMap<vismut::Connection, tcp::Connection> = Default::default();

    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buffer = [0u8; 1504];

    loop {

        let bytes_read = nic.recv(&mut buffer[..])?;

        let packet = Packet::parse(buffer, bytes_read);

        if packet.is_none() {
            println!("Packet ignored!");
            continue;
        }

        let packet = packet.unwrap();

        use std::collections::hash_map::Entry;

        match connections.entry(vismut::Connection {
            src: (packet.ip_header.source_address, packet.tcp_header.source_port),
            dst: (packet.ip_header.destination_address, packet.tcp_header.destination_port),
        }) {
            Entry::Occupied(connection) => {
                connection.get().on_packet(&nic, &packet);
            }
            Entry::Vacant(entry) => {
                if let Some(connection) = tcp::Connection::accept(&nic, &packet)? {
                    entry.insert(connection);
                }
            }
        }

        //println!("Connections: {:#?}", connections);

    }

    Ok(())

}
