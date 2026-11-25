//! Experimental, visible optical transport primitives. Frame modulation and RaptorQ
//! integration remain roadmap items; this crate currently offers bounded framing
//! and shard reconstruction helpers only.
use wasm_bindgen::prelude::*;
const MAGIC: &[u8;4]=b"PVOT";
const MAX_PAYLOAD: usize=64*1024*1024;
#[derive(Debug)] pub enum Error { Header, Length, Limit, Integrity }
fn frame(payload:&[u8])->Result<Vec<u8>,Error>{if payload.len()>MAX_PAYLOAD{return Err(Error::Limit)}let digest=blake3::hash(payload);let mut out=Vec::with_capacity(44+payload.len());out.extend_from_slice(MAGIC);out.extend_from_slice(&(payload.len() as u64).to_be_bytes());out.extend_from_slice(digest.as_bytes());out.extend_from_slice(payload);Ok(out)}
fn unframe(bytes:&[u8])->Result<Vec<u8>,Error>{if bytes.len()<44||&bytes[..4]!=MAGIC{return Err(Error::Header)}let n=u64::from_be_bytes(bytes[4..12].try_into().map_err(|_|Error::Header)?) as usize;if n>MAX_PAYLOAD{return Err(Error::Limit)}if bytes.len()!=44+n{return Err(Error::Length)}let payload=&bytes[44..];if blake3::hash(payload).as_bytes()!=&bytes[12..44]{return Err(Error::Integrity)}Ok(payload.to_vec())}
#[wasm_bindgen] pub fn frame_payload(data:&[u8])->Result<Vec<u8>,JsValue>{frame(data).map_err(|_|JsValue::from_str("framing failed"))}
#[wasm_bindgen] pub fn verify_frame(data:&[u8])->Result<Vec<u8>,JsValue>{unframe(data).map_err(|_|JsValue::from_str("frame invalid"))}
/// XOR reconstruction helper for a pair of equal-length shards.
#[wasm_bindgen] pub fn xor_reconstruct(a:&[u8],b:&[u8])->Result<Vec<u8>,JsValue>{if a.len()!=b.len()||a.len()>MAX_PAYLOAD{return Err(JsValue::from_str("shard size mismatch or limit"))}Ok(a.iter().zip(b.iter()).map(|(x,y)|x^y).collect())}
#[cfg(test)] mod tests{use super::*;#[test]fn frame_roundtrip(){let x=b"payload";assert_eq!(unframe(&frame(x).unwrap()).unwrap(),x)}#[test]fn xor(){assert_eq!(xor_reconstruct(&[1,2],&[3,4]).unwrap(),[2,6])}#[test]fn corrupt_rejected(){let mut f=frame(b"x").unwrap();f[44]^=1;assert!(matches!(unframe(&f),Err(Error::Integrity)))}}
