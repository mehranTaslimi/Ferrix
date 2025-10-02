use bendy::decoding::{Decoder, Error as BendyError, FromBencode, Object};
use sha1::{Digest, Sha1};

mod announce;
mod decode;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub length: i64,
    pub path: Vec<String>,
}

impl FromBencode for FileInfo {
    fn decode_bencode_object(object: Object) -> Result<Self, BendyError>
    where
        Self: Sized,
    {
        let mut dict = object.try_into_dictionary()?;

        let mut length = None;
        let mut path = None;

        while let Some((key, object)) = dict.next_pair()? {
            match key {
                b"length" => {
                    length = i64::decode_bencode_object(object).map(Some)?;
                }
                b"path" => {
                    let mut list = object.try_into_list()?;
                    let mut out = vec![];
                    while let Some(l) = list.next_object()? {
                        out.push(String::decode_bencode_object(l)?);
                    }

                    path = Some(out);
                }
                _ => {}
            }
        }

        Ok(FileInfo {
            length: length.ok_or_else(|| BendyError::missing_field("length"))?,
            path: path.ok_or_else(|| BendyError::missing_field("path"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    pub name: Option<String>,
    pub piece_length: Option<i64>,
    pub pieces: Option<Vec<u8>>,
    pub length: Option<i64>,
    pub files: Option<Vec<FileInfo>>,
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: Option<String>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub created_by: Option<String>,
    pub info: Info,
    pub comment: Option<String>,
    pub info_hash: [u8; 20],
}

impl Torrent {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut dec = Decoder::new(&bytes);

        let info_hash = Self::get_info_hash(&bytes)?;

        let mut dict = dec
            .next_object()?
            .ok_or_else(|| anyhow::anyhow!("empty torrent"))?
            .try_into_dictionary()?;

        let mut s = Self {
            announce: None,
            announce_list: None,
            created_by: None,
            creation_date: None,
            comment: None,
            info_hash,
            info: Info {
                name: None,
                piece_length: None,
                pieces: None,
                length: None,
                files: None,
            },
        };

        while let Some((key, object)) = dict.next_pair()? {
            match key {
                b"announce" => {
                    s.announce = String::decode_bencode_object(object).map(Some)?;
                }
                b"comment" => {
                    s.comment = String::decode_bencode_object(object).map(Some)?;
                }
                b"created by" => {
                    s.created_by = String::decode_bencode_object(object).map(Some)?;
                }
                b"announce-list" => {
                    let mut tiers = vec![];
                    if let Ok(mut list) = object.try_into_list() {
                        while let Ok(Some(next)) = list.next_object() {
                            if let Ok(mut inner_list) = next.try_into_list() {
                                let mut tire = vec![];
                                while let Ok(Some(inner_next)) = inner_list.next_object() {
                                    if let Ok(byte) = inner_next.try_into_bytes() {
                                        let announce = String::from_utf8(byte.to_vec())?;
                                        tire.push(announce);
                                    }
                                }
                                if !tire.is_empty() {
                                    tiers.push(tire);
                                }
                            }
                        }
                    }
                    s.announce_list = Some(tiers)
                }
                b"creation date" => {
                    s.creation_date = i64::decode_bencode_object(object).map(Some)?
                }
                b"info" => {
                    let mut dict = object.try_into_dictionary()?;

                    while let Some((key, object)) = dict.next_pair()? {
                        match key {
                            b"length" => {
                                s.info.length = i64::decode_bencode_object(object).map(Some)?;
                            }
                            b"name" => {
                                s.info.name = String::decode_bencode_object(object).map(Some)?;
                            }
                            b"piece length" => {
                                s.info.piece_length =
                                    i64::decode_bencode_object(object).map(Some)?;
                            }
                            b"files" => {
                                if let Ok(mut list) = object.try_into_list() {
                                    let mut out = vec![];
                                    while let Ok(Some(next)) = list.next_object() {
                                        out.push(FileInfo::decode_bencode_object(next)?)
                                    }
                                    s.info.files = Some(out);
                                }
                            }
                            b"pieces" => {
                                let bytes = object.try_into_bytes()?;
                                s.info.pieces = Some(bytes.to_vec())
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(s)
    }

    pub fn from_db(torrent_id: i64) {}

    fn get_info_hash(bytes: &[u8]) -> anyhow::Result<[u8; 20]> {
        let mut dec = Decoder::new(bytes);

        let mut dict = dec
            .next_object()?
            .ok_or_else(|| anyhow::anyhow!("empty torrent"))?
            .try_into_dictionary()?;

        while let Some((key, object)) = dict.next_pair()? {
            if key == b"info" {
                let info_dict = object.try_into_dictionary()?;
                let info_raw = info_dict.into_raw()?;

                let digest = Sha1::digest(info_raw);

                let mut out = [0u8; 20];

                out.copy_from_slice(&digest);

                return Ok(out);
            }
        }

        Err(anyhow::anyhow!("info dict not found"))
    }
}
