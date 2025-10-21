use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use super::*;

impl Torrent {
    pub(super) fn bytes_to_ipv4(&self, peer_bytes: &[u8]) -> Option<SocketAddrV4> {
        if peer_bytes.len() != 6 {
            return None;
        }

        let ip = Ipv4Addr::new(peer_bytes[0], peer_bytes[1], peer_bytes[2], peer_bytes[3]);
        let port = u16::from_be_bytes([peer_bytes[4], peer_bytes[5]]);

        Some(SocketAddrV4::new(ip, port))
    }

    pub(super) fn bytes_to_ipv6(&self, peer_bytes: &[u8]) -> Option<SocketAddrV6> {
        if peer_bytes.len() != 18 {
            return None;
        }

        let ip_bytes: [u8; 16] = peer_bytes[0..16].try_into().ok()?;

        let ip = Ipv6Addr::from(ip_bytes);
        let port = u16::from_be_bytes([peer_bytes[16], peer_bytes[17]]);

        Some(SocketAddrV6::new(ip, port, 0, 0))
    }
}
