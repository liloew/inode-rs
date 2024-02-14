use std::convert::TryInto;

use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DHCPOPTION {
    PadOption = 0x00,
    EndOption = 0xff,
    SubnetMask = 0x01,
    TimeOffset = 0x02,
    RouterOption = 0x03,
    TimeServerOption = 0x04,
    NameServerOption = 0x05,
    DomainNameServerOption = 0x06,
    LogServerOption = 0x07,
    CookieServerOption = 0x08,
    LPRServerOption = 0x09,
    ImpressServerOption = 0x0A,
    ResourceLocationServerOption = 0x0B,
    HostNameOption = 0x0C,
    BootFileSizeOption = 0x0D,
    MeritDumpFile = 0x0E,
    DomainName = 0x0F,
    SwapServer = 0x10,
    RootPath = 0x11,
    ExtensionsPath = 0x12,
    IPForwardingOption = 0x13,
    NonLocalSourceRoutingOption = 0x14,
    PolicyFilterOption = 0x15,
    MaximumDatagramReassemblySize = 0x16,
    DefaultIPTimeTolive = 0x17,
    PathMTUAgingTimeoutOption = 0x18,
    PathMTUPlateauTableOption = 0x19,
    InterfaceMTUOption = 0x1A,
    AllSubnetsAreLocalOption = 0x1B,
    BroadcastAddressOption = 0x1C,
    PerformMaskDiscoveryOption = 0x1D,
    MaskSupplierOption = 0x1E,
    PerformRouterDiscoveryOption = 0x1F,
    RouterSolicitationAddressOption = 0x20,
    StaticRouteOption = 0x21,
    TrailerEncapsulationOption = 0x22,
    ARPCacheTimeoutOption = 0x23,
    EthernetEncapsulationOption = 0x24,
    TCPDefaultTTLOption = 0x25,
    TCPKeepaliveIntervalOption = 0x26,
    TCPKeepaliveGarbageOption = 0x27,
    NetworkInformationServiceDomainOption = 0x28,
    NetworkInformationServersOption = 0x29,
    NetworkTimeProtocolServersOption = 0x2A,
    VendorSpecificInformation = 0x2B,
    NetBIOSOverTCPIPNameServerOption = 0x2C,
    NetBIOSOverTCPIPDatagramDistributionServerOption = 0x2D,
    NetBIOSOverTCPIPNodeTypeOption = 0x2E,
    NetBIOSOverTCPIPScopeOption = 0x2F,
    XWindowSystemFontServerOption = 0x30,
    XWindowSystemDisplayManagerOption = 0x31,
    RequestedIPAddress = 0x32,
    IPAddressLeaseTime = 0x33,
    OptionOverload = 0x34,
    DHCPMessageType = 0x35,
    ServerIdentifier = 0x36,
    ParameterRequestList = 0x37,
    Message = 0x38,
    MaximumDHCPMessageSize = 0x39,
    RenewalTimeValue = 0x3A,
    RebindingTimeValue = 0x3B,
    VendorClassIdentifier = 0x3C,
    ClientIdentifier = 0x3D,
    NetworkInformationServicePlusDomainOption = 0x40,
    NetworkInformationServicePlusServersOption = 0x41,
    TFTPServerName = 0x42,
    BootfileName = 0x43,
    MobileIPHomeAgentOption = 0x44,
    SMTPServerOption = 0x45,
    POP3ServerOption = 0x46,
    NNTPServerOption = 0x47,
    DefaultWWWServerOption = 0x48,
    DefaultFingerServerOption = 0x49,
    DefaultIRCServerOption = 0x4A,
    StreetTalkServerOption = 0x4B,
    StreetTalkDirectoryAssistanceServerOption = 0x4C,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHCPOPTIONS {
    pub tp: DHCPOPTION,
    pub len: u8,
    pub va: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DHCPMessageType {
    DHCPDISCOVER = 0x01,
    DHCPOFFER = 0x02,
    DHCPREQUEST = 0x03,
    DHCPDECLINE = 0x04,
    DHCPACK = 0x05,
    DHCPNAK = 0x06,
    DHCPRELEASE = 0x07,
    DHCPINFORM = 0x08,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum DHCPOP {
    #[default]
    BOOTREQUEST = 0x01,
    BOOTREPLY = 0x02,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum DHCPHType {
    #[default]
    ETHERNET = 0x01,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum DHCPHLen {
    #[default]
    ETHERNET = 0x06,
}

pub trait DHCPBytes {
    fn to_bytes(&self) -> BytesMut;
}

#[derive(Debug, Clone)]
pub struct DHCPDiscover {
    pub op: DHCPOP,
    pub htype: DHCPHType,
    pub hlen: DHCPHLen,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: u32,
    pub yiaddr: u32,
    pub siaddr: u32,
    pub giaddr: u32,
    pub chaddr: [u8; 16],
    pub sname: [u8; 64],
    pub file: [u8; 128],
    pub options: Vec<u8>,
}

impl Default for DHCPDiscover {
    fn default() -> Self {
        DHCPDiscover {
            op: DHCPOP::BOOTREQUEST,
            htype: DHCPHType::ETHERNET,
            hlen: DHCPHLen::ETHERNET,
            hops: 0x00,
            xid: 0x00000000,
            secs: 0x0000,
            flags: 0x0000,
            ciaddr: 0x00000000,
            yiaddr: 0x00000000,
            siaddr: 0x00000000,
            giaddr: 0x00000000,
            chaddr: [0x00; 16],
            sname: [0; 64],
            file: [0; 128],
            options: vec![99, 130, 83, 99],
        }
    }
}

impl DHCPDiscover {
    pub fn with_mac(mac: &str) -> Self {
        let mut mac_addr: Vec<u8> = if mac.contains("-") {
            str::split(mac, "-")
                .map(|u| u8::from_str_radix(u, 16).unwrap())
                .collect()
        } else {
            str::split(mac, ":")
                .map(|u| u8::from_str_radix(u, 16).unwrap())
                .collect()
        };
        mac_addr.resize(16, 0);
        let mut discover = DHCPDiscover::default();
        discover.chaddr = mac_addr.try_into().unwrap();
        discover
    }

    pub fn insert_options(&mut self, options: Vec<DHCPOPTIONS>) {
        for ele in options.iter() {
            println!("{:?}", *ele);
            self.options.push(ele.tp as u8);
            self.options.push(ele.len);
            self.options.extend(ele.va.clone());
        }
        self.options.push(0xff);
        if self.options.len() < 312 {
            self.options.resize(312, 0);
        } else if self.options.len() > 576 {
            panic!("DHCP Message must less then 512.")
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPOffer {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPRequest {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPDecline {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPAck {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPNak {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPRelease {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPInform {}

impl DHCPBytes for DHCPDiscover {
    fn to_bytes(&self) -> BytesMut {
        let mut discover_bytes = BytesMut::new();
        discover_bytes.put_u8(self.op as u8);
        discover_bytes.put_u8(self.htype as u8);
        discover_bytes.put_u8(self.hlen as u8);
        discover_bytes.put_u8(self.hops);
        discover_bytes.put_u32(self.xid);
        discover_bytes.put_u16(self.secs);
        discover_bytes.put_u16(self.flags);
        discover_bytes.put_u32(self.ciaddr);
        discover_bytes.put_u32(self.yiaddr);
        discover_bytes.put_u32(self.siaddr);
        discover_bytes.put_u32(self.giaddr);
        for ele in self.chaddr.iter() {
            discover_bytes.put_u8(*ele);
        }
        for ele in self.sname.iter() {
            discover_bytes.put_u8(*ele);
        }
        for ele in self.file.iter() {
            discover_bytes.put_u8(*ele);
        }
        for ele in self.options.iter() {
            discover_bytes.put_u8(*ele);
        }
        discover_bytes
    }
}
