use log::*;

use dns::{Request, Response};
use super::{Transport, Error, UdpTransport, TcpTransport};


/// The **automatic transport**, which sends DNS wire data using the UDP
/// transport, then tries using the TCP transport if the first one fails
/// because the response wouldn't fit in a single UDP packet.
///
/// This is the default behaviour for many DNS clients.
///
/// # Examples
///
/// ```no_run
/// use dns_transport::{Transport, AutoTransport};
/// use dns::{Request, Flags, Query, Labels, QClass, qtype, record::NS};
///
/// let query = Query {
///     qname: Labels::encode("dns.lookup.dog").unwrap(),
///     qclass: QClass::IN,
///     qtype: qtype!(NS),
/// };
///
/// let request = Request {
///     transaction_id: 0xABCD,
///     flags: Flags::query(),
///     query: query,
///     additional: None,
/// };
///
/// let transport = AutoTransport::new("8.8.8.8");
/// transport.send(&request);
/// ```
pub struct AutoTransport {
    addr: String,
}

impl AutoTransport {

    /// Creates a new automatic transport that connects to the given host.
    pub fn new(sa: impl Into<String>) -> Self {
        let addr = sa.into();
        Self { addr }
    }
}


impl Transport for AutoTransport {
    fn send(&self, request: &Request) -> Result<Response, Error> {
        let udp_transport = UdpTransport::new(&self.addr);
        let udp_response = udp_transport.send(&request)?;

        if ! udp_response.flags.truncated {
            return Ok(udp_response);
        }

        debug!("Truncated flag set, so switching to TCP");

        let tcp_transport = TcpTransport::new(&self.addr);
        let tcp_response = tcp_transport.send(&request)?;
        Ok(tcp_response)
    }
}
