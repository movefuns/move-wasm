use std::io::prelude::*;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn zip_metadata(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut e = GzEncoder::new(Vec::new(), Compression::best());
    e.write_all(data)?;
    Ok(e.finish()?)
}

pub fn zip_metadata_str(s: &str) -> anyhow::Result<Vec<u8>> {
    zip_metadata(s.as_bytes())
}

pub fn unzip_metadata(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut d = GzDecoder::new(data);
    let mut res = vec![];
    d.read_to_end(&mut res)?;
    Ok(res)
}

pub fn unzip_metadata_str(data: &[u8]) -> anyhow::Result<String> {
    let r = unzip_metadata(data)?;
    let s = String::from_utf8(r)?;
    Ok(s)
}
