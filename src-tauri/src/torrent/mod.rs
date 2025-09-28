use bendy::decoding::{Decoder, FromBencode};
use sha1::{Digest, Sha1};

mod announce;

// #[derive(Debug)]
// pub struct FileInfo {
//     pub length: i64,
//     pub path: Vec<String>,
// }

#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub length: Option<i64>,
    pub files: Vec<String>,
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: Option<String>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub created_by: Option<String>,
    pub info: Option<Info>,
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
            info: None,
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

                    let mut name = None;
                    let mut piece_length = None;
                    let mut pieces = None;
                    let mut length = None;
                    let mut files = None;

                    while let Some((key, object)) = dict.next_pair()? {
                        match key {
                            b"length" => {
                                length = i64::decode_bencode_object(object).map(Some)?;
                            }
                            b"name" => {
                                name = String::decode_bencode_object(object).map(Some)?;
                            }
                            b"piece length" => {
                                piece_length = i64::decode_bencode_object(object).map(Some)?;
                            }
                            b"files" => {
                                files = Vec::decode_bencode_object(object).map(Some)?;
                            }
                            b"pieces" => {
                                let bytes = object.try_into_bytes()?;
                                pieces = Some(bytes.to_vec())
                            }
                            _ => {}
                        }
                    }

                    s.info = Some(Info {
                        name: name.ok_or_else(|| anyhow::anyhow!("missing field 'name'"))?,
                        piece_length: piece_length
                            .ok_or_else(|| anyhow::anyhow!("missing field 'piece length'"))?,
                        pieces: pieces.ok_or_else(|| anyhow::anyhow!("missing field 'pieces'"))?,
                        files: files.unwrap_or_default(),
                        length,
                    });
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
