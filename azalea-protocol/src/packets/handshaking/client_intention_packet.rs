use crate::packets::ClientIntention;
use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundHandshakePacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, ServerboundHandshakePacket)]
pub struct ClientIntentionPacket {
    #[var]
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    pub intention: ClientIntention,
}
