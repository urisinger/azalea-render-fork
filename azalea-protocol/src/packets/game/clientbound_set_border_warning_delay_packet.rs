use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetBorderWarningDelayPacket {
    #[var]
    pub warning_delay: u32,
}