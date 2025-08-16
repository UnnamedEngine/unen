pub mod server;
pub mod client;

static MESSAGE_MAX_SIZE: usize = 128;

#[cfg(test)]
mod tests {
    use super::server::*;
    use super::client::*;

    use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr};
    
    const LOCAL_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

    #[test]
    fn send_to_server() {
        let server = create_server()
            .listen(LOCAL_ADDR)
            .expect("should be able to listen");
        let client = create_client()
            .connect(LOCAL_ADDR, server.addr())
            .expect("should be able to connect to server");

        let sent_bytes = client
            .send(b"hello!")
            .expect("should be able to send");

        assert_eq!(sent_bytes, 6, "should have sent 6 bytes");

        let mut received_data = server.poll();

        assert_eq!(received_data.len(), 1, "should have received 1 datagram");

        let (addres, bytes) = received_data.pop().unwrap();

        assert_eq!(addres, client.addr(), "should have been received from client");
        assert_eq!(bytes.as_slice(), b"hello!", "should have received 'hello!'");
    }

    #[test]
    fn send_to_client() {
        let server = create_server()
            .listen(LOCAL_ADDR)
            .expect("should be able to listen");
        let client = create_client()
            .connect(LOCAL_ADDR, server.addr())
            .expect("should be able to connect to server");

        let client_addr = client.addr();

        let sent_bytes = server
            .send_to(b"hello :D", client_addr)
            .expect("should be able to send");

        assert_eq!(sent_bytes, 8, "should have sent 8 bytes");

        let mut received_data = client.poll();

        assert_eq!(received_data.len(), 1, "should have received 1 datagram");

        let bytes = received_data.pop().unwrap();

        assert_eq!(bytes.as_slice(), b"hello :D", "should have received 'hello :D'");
    }
}