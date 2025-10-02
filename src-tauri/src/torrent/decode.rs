use anyhow::anyhow;
use bendy::decoding::{Error as BendyError, Object};

use super::*;

#[derive(Debug, Clone)]
pub struct PeerDict {
    pub peer_id: Option<Vec<u8>>,
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub enum Peers {
    CompactV4(Vec<u8>),
    Dict(Vec<PeerDict>),
}

#[derive(Debug, Clone)]
pub struct Tracker {
    pub interval: Option<u64>,
    pub min_interval: Option<u64>,
    pub tracker_id: Option<String>,
    pub complete: Option<u64>,
    pub incomplete: Option<u64>,
    pub downloaded: Option<u64>,
    pub peers: Option<Peers>,
    pub peers6: Option<Vec<u8>>,
    pub failure_reason: Option<String>,
    pub warning_message: Option<String>,
}

impl FromBencode for PeerDict {
    fn decode_bencode_object(object: Object) -> Result<Self, BendyError>
    where
        Self: Sized,
    {
        let mut dict = object.try_into_dictionary()?;

        let mut peer_id = None;
        let mut ip = None;
        let mut port = None;

        while let Some((key, object)) = dict.next_pair()? {
            match key {
                b"peer id" => {
                    peer_id = object.try_into_bytes().map(|op| op.to_vec()).map(Some)?;
                }
                b"port" => {
                    port = u16::decode_bencode_object(object).map(Some)?;
                }
                b"ip" => {
                    ip = String::decode_bencode_object(object).map(Some)?;
                }
                _ => {}
            }
        }

        Ok(PeerDict {
            peer_id: peer_id,
            ip: ip.ok_or_else(|| BendyError::missing_field("ip"))?,
            port: port.ok_or_else(|| BendyError::missing_field("port"))?,
        })
    }
}

impl Torrent {
    pub(super) fn decode_tracker_response(&self, bytes: &[u8]) -> anyhow::Result<Tracker> {
        let mut dec = Decoder::new(&bytes);

        let mut dict = dec
            .next_object()?
            .ok_or_else(|| anyhow::anyhow!("empty tracker response"))?
            .try_into_dictionary()?;

        let mut tracker = Tracker {
            interval: None,
            min_interval: None,
            tracker_id: None,
            complete: None,
            incomplete: None,
            downloaded: None,
            peers: None,
            peers6: None,
            failure_reason: None,
            warning_message: None,
        };

        while let Some((key, object)) = dict.next_pair()? {
            match key {
                b"interval" => {
                    let interval = u64::decode_bencode_object(object).map(Some)?;
                    tracker.interval = interval
                }
                b"min interval" => {
                    let min_interval = u64::decode_bencode_object(object).map(Some)?;
                    tracker.min_interval = min_interval
                }
                b"tracker id" => {
                    let tracker_id = String::decode_bencode_object(object).map(Some)?;
                    tracker.tracker_id = tracker_id
                }
                b"complete" => {
                    let complete = u64::decode_bencode_object(object).map(Some)?;
                    tracker.complete = complete
                }
                b"incomplete" => {
                    let incomplete = u64::decode_bencode_object(object).map(Some)?;
                    tracker.incomplete = incomplete
                }
                b"downloaded" => {
                    let downloaded = u64::decode_bencode_object(object).map(Some)?;
                    tracker.downloaded = downloaded
                }
                b"peers" => {
                    match object {
                        Object::List(mut list_decoder) => {
                            let mut out = vec![];
                            while let Ok(Some(next)) = list_decoder.next_object() {
                                out.push(PeerDict::decode_bencode_object(next)?);
                            }
                            tracker.peers = Some(Peers::Dict(out));
                        }
                        Object::Bytes(items) => {
                            tracker.peers = Some(Peers::CompactV4(items.to_vec()))
                        }
                        _ => {}
                    };
                }
                b"peers6" => {
                    let peers = object.try_into_bytes().map(|op| op.to_vec()).map(Some)?;
                    tracker.peers6 = peers
                }
                b"failure reason" => {
                    let failure_reason = String::decode_bencode_object(object).map(Some)?;
                    tracker.failure_reason = failure_reason
                }
                b"warning message" => {
                    let warning_message = String::decode_bencode_object(object).map(Some)?;
                    tracker.warning_message = warning_message
                }
                _ => {}
            }
        }

        Ok(tracker)
    }
}
