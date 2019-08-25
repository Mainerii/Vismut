use std::io;
use lightheader::packet::{Packet, PacketBuilder};

#[derive(Debug)]
enum State {

    Closed,
    Listen,
    SynRcvd,
    //Estab,

}

pub struct Connection {

    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,

}

struct RecvSequenceSpace {

    /// receive next
    nxt: u32,
    /// receive window
    wnd: u16,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: u32,

}

struct SendSequenceSpace {

    /// send unacknowledged
    una: u32,
    /// send next
    nxt: u32,
    /// send window
    wnd: u16,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledgment number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: u32,

}

impl Connection {

    pub fn accept(nic: &tun_tap::Iface, packet: &Packet) -> io::Result<Option<Self>> {
        println!("Accepting TCP-packet with {} bytes ({} of TCP-data) from {}:{} to {}:{}",
                 packet.ip_header.total_length,
                 packet.get_tcp_data().len(),
                 packet.ip_header.get_source_address_str(),
                 packet.tcp_header.source_port,
                 packet.ip_header.get_destination_address_str(),
                 packet.tcp_header.destination_port,
        );


        if !packet.tcp_header.syn {
            // Only SYN accepted when listening
            return Ok(None);
        }

        let iss = 0;

        let mut connection = Connection {

            state: State::SynRcvd,

            recv: RecvSequenceSpace {
                irs: packet.tcp_header.sequence_number,
                nxt: packet.tcp_header.sequence_number + 1,
                wnd: packet.tcp_header.window,
                up: false,
            },

            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },

        };

        // Create ack packet
        let mut packet_builder = PacketBuilder::new();
        packet_builder.ip_header_builder.source_address = packet.ip_header.destination_address;
        packet_builder.ip_header_builder.destination_address = packet.ip_header.source_address;
        packet_builder.ip_header_builder.ttl = 64;
        packet_builder.tcp_header_builder.source_port = packet.tcp_header.destination_port;
        packet_builder.tcp_header_builder.destination_port = packet.tcp_header.source_port;
        packet_builder.tcp_header_builder.sequence_number = connection.send.iss;
        packet_builder.tcp_header_builder.window = connection.send.wnd;
        packet_builder.tcp_header_builder.acknowledgement_number = connection.recv.nxt;
        packet_builder.tcp_header_builder.syn = true;
        packet_builder.tcp_header_builder.ack = true;


        // Send ack to establish connection
        let bytes_sent = nic.send(packet_builder.build().unwrap().get_bytes())?;

        Ok(Some(connection))

    }

    pub fn on_packet(&self, nic: &tun_tap::Iface, packet: &Packet) { //-> io::Result<()>

        println!("Accepting TCP-packet with {} bytes ({} of TCP-data) from {}:{} to {}:{}",
                 packet.ip_header.total_length,
                 packet.get_tcp_data().len(),
                 packet.ip_header.get_source_address_str(),
                 packet.tcp_header.source_port,
                 packet.ip_header.get_destination_address_str(),
                 packet.tcp_header.destination_port,
        );

    }


}
