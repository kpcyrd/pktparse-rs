//! Handles parsing of Ethernet headers

use nom::IResult;
use nom::Endianness::Big;

#[derive(Debug, PartialEq, Eq)]
pub struct MacAddress(pub [u8; 6]);
#[derive(Debug, PartialEq, Eq)]
pub enum EtherType {
    LANMIN,
    LANMAX,
    IPv4,
    ARP,
    WOL,
    TRILL,
    DECnet,
    RARP,
    AppleTalk,
    AARP,
    VLAN,
    IPX,
    Qnet,
    IPv6,
    FlowControl,
    CobraNet,
    MPLSuni,
    MPLSmulti,
    PPPoEdiscovery,
    PPPoEsession,
    HomePlug,
    EAPOL,
    PROFINET,
    HyperSCSI,
    ATAOE,
    EtherCAT,
    QinQ,
    Powerlink,
    GOOSE,
    GSE,
    LLDP,
    SERCOS,
    HomePlugAV,
    MRP,
    MACsec,
    PBB,
    PTP,
    PRP,
    CFM,
    FCoE,
    FCoEi,
    RoCE,
    TTE,
    HSR,
    CTP,
    VLANdouble,
    Other(u16),
}
#[derive(Debug, PartialEq, Eq)]
pub struct EthernetFrame {
    pub source_mac: MacAddress,
    pub dest_mac: MacAddress,
    pub ethertype: EtherType,
}

fn to_ethertype(i: u16) -> Option<EtherType> {
    match i {
        0x002E => Some(EtherType::LANMIN),    // 802.3 Min data length
        0x05DC => Some(EtherType::LANMAX),    // 802.3 Max data length
        0x0800 => Some(EtherType::IPv4),    // Internet Protocol version 4 (IPv4)
        0x0806 => Some(EtherType::ARP),    // Address Resolution Protocol (ARP)
        0x0842 => Some(EtherType::WOL),    // Wake-on-LAN[4]
        0x22F3 => Some(EtherType::TRILL),    // IETF TRILL Protocol
        0x6003 => Some(EtherType::DECnet),    // DECnet Phase IV
        0x8035 => Some(EtherType::RARP),    // Reverse Address Resolution Protocol
        0x809B => Some(EtherType::AppleTalk),    // AppleTalk (Ethertalk)
        0x80F3 => Some(EtherType::AARP),    // AppleTalk Address Resolution Protocol (AARP)
        0x8100 => Some(EtherType::VLAN),    // VLAN-tagged frame (IEEE 802.1Q) and Shortest Path Bridging IEEE 802.1aq[5]
        0x8137 => Some(EtherType::IPX),    // IPX
        0x8204 => Some(EtherType::Qnet),    // QNX Qnet
        0x86DD => Some(EtherType::IPv6),    // Internet Protocol Version 6 (IPv6)
        0x8808 => Some(EtherType::FlowControl),    // Ethernet flow control
        0x8819 => Some(EtherType::CobraNet),    // CobraNet
        0x8847 => Some(EtherType::MPLSuni),    // MPLS unicast
        0x8848 => Some(EtherType::MPLSmulti),    // MPLS multicast
        0x8863 => Some(EtherType::PPPoEdiscovery),    // PPPoE Discovery Stage
        0x8864 => Some(EtherType::PPPoEsession),    // PPPoE Session Stage
        0x887B => Some(EtherType::HomePlug),    // HomePlug 1.0 MME
        0x888E => Some(EtherType::EAPOL),    // EAP over LAN (IEEE 802.1X)
        0x8892 => Some(EtherType::PROFINET),    // PROFINET Protocol
        0x889A => Some(EtherType::HyperSCSI),    // HyperSCSI (SCSI over Ethernet)
        0x88A2 => Some(EtherType::ATAOE),    // ATA over Ethernet
        0x88A4 => Some(EtherType::EtherCAT),    // EtherCAT Protocol
        0x88A8 => Some(EtherType::QinQ),    // Provider Bridging (IEEE 802.1ad) & Shortest Path Bridging IEEE 802.1aq[5]
        0x88AB => Some(EtherType::Powerlink),    // Ethernet Powerlink[citation needed]
        0x88B8 => Some(EtherType::GOOSE),    // GOOSE (Generic Object Oriented Substation event)
        0x88B9 => Some(EtherType::GSE),    // GSE (Generic Substation Events) Management Services
        0x88CC => Some(EtherType::LLDP),    // Link Layer Discovery Protocol (LLDP)
        0x88CD => Some(EtherType::SERCOS),    // SERCOS III
        0x88E1 => Some(EtherType::HomePlugAV),    // HomePlug AV MME[citation needed]
        0x88E3 => Some(EtherType::MRP),    // Media Redundancy Protocol (IEC62439-2)
        0x88E5 => Some(EtherType::MACsec),    // MAC security (IEEE 802.1AE)
        0x88E7 => Some(EtherType::PBB),    // Provider Backbone Bridges (PBB) (IEEE 802.1ah)
        0x88F7 => Some(EtherType::PTP),    // Precision Time Protocol (PTP) over Ethernet (IEEE 1588)
        0x88FB => Some(EtherType::PRP),    // Parallel Redundancy Protocol (PRP)
        0x8902 => Some(EtherType::CFM),    // IEEE 802.1ag Connectivity Fault Management (CFM) Protocol / ITU-T Recommendation Y.1731 (OAM)
        0x8906 => Some(EtherType::FCoE),    // Fibre Channel over Ethernet (FCoE)
        0x8914 => Some(EtherType::FCoEi),    // FCoE Initialization Protocol
        0x8915 => Some(EtherType::RoCE),    // RDMA over Converged Ethernet (RoCE)
        0x891D => Some(EtherType::TTE),    // TTEthernet Protocol Control Frame (TTE)
        0x892F => Some(EtherType::HSR),    // High-availability Seamless Redundancy (HSR)
        0x9000 => Some(EtherType::CTP),    // Ethernet Configuration Testing Protocol[6]
        0x9100 => Some(EtherType::VLANdouble),    // VLAN-tagged (IEEE 802.1Q) frame with double tagging
        other => Some(EtherType::Other(other)),
    }
}

fn to_mac_address(i: &[u8]) -> MacAddress {
    MacAddress(array_ref![i, 0, 6].clone())
}

named!(mac_address<&[u8], MacAddress>, map!(take!(6), to_mac_address));
named!(ethertype<&[u8], EtherType>, map_opt!(u16!(Big), to_ethertype));
named!(ethernet_frame<&[u8], EthernetFrame>, chain!(
    dest_mac: mac_address ~
    src_mac: mac_address ~
    et: ethertype,
    || EthernetFrame{source_mac: src_mac, dest_mac: dest_mac, ethertype: et}
));

pub fn parse_ethernet_frame(i: &[u8]) -> IResult<&[u8], EthernetFrame> {
    ethernet_frame(i)
}

#[cfg(test)]
mod tests {
    use super::{mac_address, ethertype, ethernet_frame, MacAddress, EtherType, EthernetFrame};
    use nom::IResult;
    const EMPTY_SLICE: &'static [u8] = &[];
    #[test]
    fn mac_address_works() {
        let bytes = [0x9c, 0x5c, 0x8e, 0x90, 0xca, 0xfc];
        assert_eq!(mac_address(&bytes), IResult::Done(EMPTY_SLICE, MacAddress(bytes)));
    }

    macro_rules! mk_ethertype_test {
        ($func_name:ident, $bytes:expr, $correct_ethertype:expr) => (
            #[test]
            fn $func_name() {
                let bytes = $bytes;
                assert_eq!(ethertype(&bytes), IResult::Done(EMPTY_SLICE, $correct_ethertype));
            }
        )
    }

    mk_ethertype_test!(ethertype_gets_ipv4_correct, [0x08, 0x00], EtherType::IPv4);
    mk_ethertype_test!(ethertype_gets_arp_correct, [0x08, 0x06], EtherType::ARP);
    mk_ethertype_test!(ethertype_gets_ipv6_correct, [0x86, 0xDD], EtherType::IPv6);
    mk_ethertype_test!(ethertype_gets_vlan_correct, [0x81, 0x00], EtherType::VLAN);

    #[test]
    fn ethernet_frame_works() {
        let bytes = [0x00, 0x23, 0x54, 0x07, 0x93, 0x6c, /* dest MAC */
                     0x00, 0x1b, 0x21, 0x0f, 0x91, 0x9b, /* src MAC */ 
                     0x08, 0x00 // Ethertype
                    ];
        let expectation = EthernetFrame {
            source_mac: MacAddress([0x00, 0x1b, 0x21, 0x0f, 0x91, 0x9b]),
            dest_mac: MacAddress([0x00, 0x23, 0x54, 0x07, 0x93, 0x6c]),
            ethertype: EtherType::IPv4,
        };
        assert_eq!(ethernet_frame(&bytes), IResult::Done(EMPTY_SLICE, expectation));
    }
}
