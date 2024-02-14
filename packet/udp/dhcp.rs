use std::convert::TryInto;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
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

impl Into<DHCPOPTION> for u8 {
    fn into(self) -> DHCPOPTION {
        match self {
            0x00 => DHCPOPTION::PadOption,
            0xff => DHCPOPTION::EndOption,
            0x01 => DHCPOPTION::SubnetMask,
            0x02 => DHCPOPTION::TimeOffset,
            0x03 => DHCPOPTION::RouterOption,
            0x04 => DHCPOPTION::TimeServerOption,
            0x05 => DHCPOPTION::NameServerOption,
            0x06 => DHCPOPTION::DomainNameServerOption,
            0x07 => DHCPOPTION::LogServerOption,
            0x08 => DHCPOPTION::CookieServerOption,
            0x09 => DHCPOPTION::LPRServerOption,
            0x0A => DHCPOPTION::ImpressServerOption,
            0x0B => DHCPOPTION::ResourceLocationServerOption,
            0x0C => DHCPOPTION::HostNameOption,
            0x0D => DHCPOPTION::BootFileSizeOption,
            0x0E => DHCPOPTION::MeritDumpFile,
            0x0F => DHCPOPTION::DomainName,
            0x10 => DHCPOPTION::SwapServer,
            0x11 => DHCPOPTION::RootPath,
            0x12 => DHCPOPTION::ExtensionsPath,
            0x13 => DHCPOPTION::IPForwardingOption,
            0x14 => DHCPOPTION::NonLocalSourceRoutingOption,
            0x15 => DHCPOPTION::PolicyFilterOption,
            0x16 => DHCPOPTION::MaximumDatagramReassemblySize,
            0x17 => DHCPOPTION::DefaultIPTimeTolive,
            0x18 => DHCPOPTION::PathMTUAgingTimeoutOption,
            0x19 => DHCPOPTION::PathMTUPlateauTableOption,
            0x1A => DHCPOPTION::InterfaceMTUOption,
            0x1B => DHCPOPTION::AllSubnetsAreLocalOption,
            0x1C => DHCPOPTION::BroadcastAddressOption,
            0x1D => DHCPOPTION::PerformMaskDiscoveryOption,
            0x1E => DHCPOPTION::MaskSupplierOption,
            0x1F => DHCPOPTION::PerformRouterDiscoveryOption,
            0x20 => DHCPOPTION::RouterSolicitationAddressOption,
            0x21 => DHCPOPTION::StaticRouteOption,
            0x22 => DHCPOPTION::TrailerEncapsulationOption,
            0x23 => DHCPOPTION::ARPCacheTimeoutOption,
            0x24 => DHCPOPTION::EthernetEncapsulationOption,
            0x25 => DHCPOPTION::TCPDefaultTTLOption,
            0x26 => DHCPOPTION::TCPKeepaliveIntervalOption,
            0x27 => DHCPOPTION::TCPKeepaliveGarbageOption,
            0x28 => DHCPOPTION::NetworkInformationServiceDomainOption,
            0x29 => DHCPOPTION::NetworkInformationServersOption,
            0x2A => DHCPOPTION::NetworkTimeProtocolServersOption,
            0x2B => DHCPOPTION::VendorSpecificInformation,
            0x2C => DHCPOPTION::NetBIOSOverTCPIPNameServerOption,
            0x2D => DHCPOPTION::NetBIOSOverTCPIPDatagramDistributionServerOption,
            0x2E => DHCPOPTION::NetBIOSOverTCPIPNodeTypeOption,
            0x2F => DHCPOPTION::NetBIOSOverTCPIPScopeOption,
            0x30 => DHCPOPTION::XWindowSystemFontServerOption,
            0x31 => DHCPOPTION::XWindowSystemDisplayManagerOption,
            0x32 => DHCPOPTION::RequestedIPAddress,
            0x33 => DHCPOPTION::IPAddressLeaseTime,
            0x34 => DHCPOPTION::OptionOverload,
            0x35 => DHCPOPTION::DHCPMessageType,
            0x36 => DHCPOPTION::ServerIdentifier,
            0x37 => DHCPOPTION::ParameterRequestList,
            0x38 => DHCPOPTION::Message,
            0x39 => DHCPOPTION::MaximumDHCPMessageSize,
            0x3A => DHCPOPTION::RenewalTimeValue,
            0x3B => DHCPOPTION::RebindingTimeValue,
            0x3C => DHCPOPTION::VendorClassIdentifier,
            0x3D => DHCPOPTION::ClientIdentifier,
            0x40 => DHCPOPTION::NetworkInformationServicePlusDomainOption,
            0x41 => DHCPOPTION::NetworkInformationServicePlusServersOption,
            0x42 => DHCPOPTION::TFTPServerName,
            0x43 => DHCPOPTION::BootfileName,
            0x44 => DHCPOPTION::MobileIPHomeAgentOption,
            0x45 => DHCPOPTION::SMTPServerOption,
            0x46 => DHCPOPTION::POP3ServerOption,
            0x47 => DHCPOPTION::NNTPServerOption,
            0x48 => DHCPOPTION::DefaultWWWServerOption,
            0x49 => DHCPOPTION::DefaultFingerServerOption,
            0x4A => DHCPOPTION::DefaultIRCServerOption,
            0x4B => DHCPOPTION::StreetTalkServerOption,
            0x4C => DHCPOPTION::StreetTalkDirectoryAssistanceServerOption,
            _ => DHCPOPTION::EndOption,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHCPOPTIONS {
    pub tp: DHCPOPTION,
    pub len: u8,
    pub va: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
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
#[repr(u8)]
pub enum DHCPOP {
    #[default]
    BOOTREQUEST = 0x01,
    BOOTREPLY = 0x02,
}

impl Into<DHCPOP> for u8 {
    fn into(self) -> DHCPOP {
        match self {
            0x01 => DHCPOP::BOOTREQUEST,
            0x02 => DHCPOP::BOOTREPLY,
            _ => DHCPOP::BOOTREQUEST,
        }
    }
}

impl From<DHCPOP> for u8 {
    fn from(value: DHCPOP) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum DHCPHType {
    #[default]
    ETHERNET = 0x01,
}

impl Into<DHCPHType> for u8 {
    fn into(self) -> DHCPHType {
        match self {
            0x01 => DHCPHType::ETHERNET,
            _ => DHCPHType::ETHERNET,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum DHCPHLen {
    #[default]
    ETHERNET = 0x06,
}

impl Into<DHCPHLen> for u8 {
    fn into(self) -> DHCPHLen {
        match self {
            0x01 => DHCPHLen::ETHERNET,
            _ => DHCPHLen::ETHERNET,
        }
    }
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
            self.options.push(ele.tp as u8);
            self.options.push(ele.len);
            self.options.extend(ele.va.clone());
        }
        self.options.push(0xff);
        // if self.options.len() < 312 {
        //     self.options.resize(312, 0);
        // } else if self.options.len() > 576 {
        //     panic!("DHCP Message must less then 512.")
        // }
    }
}

#[derive(Debug, Clone)]
pub struct DHCPOffer {
    pub op: DHCPOP,
    pub htype: DHCPHType,
    pub hlen: DHCPHLen,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: [u8; 4],
    pub yiaddr: [u8; 4],
    pub siaddr: [u8; 4],
    pub giaddr: [u8; 4],
    pub chaddr: [u8; 16],
    pub sname: [u8; 64],
    pub file: [u8; 128],
    pub options: Vec<DHCPOPTIONS>,
}

impl DHCPOffer {
    pub fn default() -> Self {
        todo!()
    }

    pub fn from_bytes<T: Buf>(buf: &mut T) -> Self {
        let op: DHCPOP = buf.get_u8().into();
        let htype: DHCPHType = buf.get_u8().into();
        let hlen = buf.get_u8();
        let hops = buf.get_u8();
        let xid = buf.get_u32();
        let secs = buf.get_u16();
        let flags = buf.get_u16();
        let mut ciaddr = [0u8; 4];
        let mut yiaddr = [0u8; 4];
        let mut siaddr = [0u8; 4];
        let mut giaddr = [0u8; 4];
        for i in 0..4 {
            ciaddr[i] = buf.get_u8();
        }
        for i in 0..4 {
            yiaddr[i] = buf.get_u8();
        }
        for i in 0..4 {
            siaddr[i] = buf.get_u8();
        }
        for i in 0..4 {
            giaddr[i] = buf.get_u8();
        }
        let mut chaddr = [0u8; 16];
        for i in 0..16 {
            chaddr[i] = buf.get_u8();
        }
        let mut sname = [0u8; 64];
        for i in 0..64 {
            sname[i] = buf.get_u8();
        }
        let mut file = [0u8; 128];
        for i in 0..128 {
            file[i] = buf.get_u8();
        }
        // magic cookie dhcp read as u32 - [63, 82, 53, 63]
        let magic_cookie_dhcp = buf.get_u32();
        if magic_cookie_dhcp != 1669485411u32 {
            println!("Error: {}", magic_cookie_dhcp);
            // panic
        }
        let mut options: Vec<DHCPOPTIONS> = Vec::new();
        loop {
            let tp = buf.get_u8();
            if tp == DHCPOPTION::EndOption as u8 {
                break;
            }
            let len = buf.get_u8();
            let mut va = Vec::with_capacity(len as usize);
            for i in 0..len as usize {
                va.push(buf.get_u8());
            }
            options.push(DHCPOPTIONS {
                tp: tp.into(),
                len: len,
                va: va,
            })
        }
        DHCPOffer {
            op: op,
            htype: htype,
            hlen: hlen.into(),
            hops: hops,
            xid: xid,
            secs: secs,
            flags: flags,
            ciaddr: ciaddr,
            yiaddr: yiaddr,
            siaddr: siaddr,
            giaddr: giaddr,
            chaddr: chaddr,
            sname: sname,
            file: file,
            options: options,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DHCPRequest {
    pub op: DHCPOP,
    pub htype: DHCPHType,
    pub hlen: DHCPHLen,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: [u8; 4],
    pub yiaddr: [u8; 4],
    pub siaddr: [u8; 4],
    pub giaddr: [u8; 4],
    pub chaddr: [u8; 16],
    pub sname: [u8; 64],
    pub file: [u8; 128],
    // pub options: Vec<DHCPOPTIONS>,
    pub options: Vec<u8>,
}

impl Default for DHCPRequest {
    fn default() -> Self {
        DHCPRequest {
            op: DHCPOP::BOOTREQUEST,
            htype: DHCPHType::ETHERNET,
            hlen: DHCPHLen::ETHERNET,
            hops: 0x00,
            xid: 0x00000000,
            secs: 0x0000,
            flags: 0x0000,
            ciaddr: [0; 4],
            yiaddr: [0; 4],
            siaddr: [0; 4],
            giaddr: [0; 4],
            chaddr: [0x00; 16],
            sname: [0; 64],
            file: [0; 128],
            // options: Vec::new(),
            options: vec![99, 130, 83, 99],
        }
    }
}

impl DHCPRequest {
    //    fn with_mac_ip_options(mac: &str, ip: [u8; 4], options: Vec<DHCPOPTIONS>) -> Self {
    pub fn with_mac_ip_options(mac: &str, options: Vec<DHCPOPTIONS>) -> Self {
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
        let mut dhcp_request = DHCPRequest::default();
        dhcp_request.chaddr = mac_addr.try_into().unwrap();
        // TODO: get_hostname()
        // dhcp_request.sname =
        for ele in options.into_iter() {
            dhcp_request.options.push(ele.tp as u8);
            dhcp_request.options.push(ele.len);
            dhcp_request.options.extend(ele.va);
        }
        dhcp_request.options.push(0xff);
        dhcp_request
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPDecline {}

#[derive(Debug, Clone)]
pub struct DHCPAck {
    pub op: DHCPOP,
    pub htype: DHCPHType,
    pub hlen: DHCPHLen,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: [u8; 4],
    pub yiaddr: [u8; 4],
    pub siaddr: [u8; 4],
    pub giaddr: [u8; 4],
    pub chaddr: [u8; 16],
    pub sname: [u8; 64],
    pub file: [u8; 128],
    pub options: Vec<DHCPOPTIONS>,
}

impl DHCPAck {
    pub fn from_bytes<T: Buf>(buf: &mut T) -> Self {
        let op: DHCPOP = buf.get_u8().into();
        let htype: DHCPHType = buf.get_u8().into();
        let hlen = buf.get_u8();
        let hops = buf.get_u8();
        let xid = buf.get_u32();
        let secs = buf.get_u16();
        let flags = buf.get_u16();
        let mut ciaddr = [0u8; 4];
        let mut yiaddr = [0u8; 4];
        let mut siaddr = [0u8; 4];
        let mut giaddr = [0u8; 4];
        for i in 0..4 {
            ciaddr[i] = buf.get_u8();
        }
        for i in 0..4 {
            yiaddr[i] = buf.get_u8();
        }
        for i in 0..4 {
            siaddr[i] = buf.get_u8();
        }
        for i in 0..4 {
            giaddr[i] = buf.get_u8();
        }
        let mut chaddr = [0u8; 16];
        for i in 0..16 {
            chaddr[i] = buf.get_u8();
        }
        let mut sname = [0u8; 64];
        for i in 0..64 {
            sname[i] = buf.get_u8();
        }
        let mut file = [0u8; 128];
        for i in 0..128 {
            file[i] = buf.get_u8();
        }
        // magic cookie dhcp read as u32 - [63, 82, 53, 63]
        let magic_cookie_dhcp = buf.get_u32();
        if magic_cookie_dhcp != 1669485411u32 {
            println!("Error: {}", magic_cookie_dhcp);
            // panic
        }
        let mut options: Vec<DHCPOPTIONS> = Vec::new();
        loop {
            let tp = buf.get_u8();
            if tp == DHCPOPTION::EndOption as u8 {
                break;
            }
            let len = buf.get_u8();
            let mut va = Vec::with_capacity(len as usize);
            for i in 0..len as usize {
                va.push(buf.get_u8());
            }
            options.push(DHCPOPTIONS {
                tp: tp.into(),
                len: len,
                va: va,
            })
        }
        DHCPAck {
            op: op,
            htype: htype,
            hlen: hlen.into(),
            hops: hops,
            xid: xid,
            secs: secs,
            flags: flags,
            ciaddr: ciaddr,
            yiaddr: yiaddr,
            siaddr: siaddr,
            giaddr: giaddr,
            chaddr: chaddr,
            sname: sname,
            file: file,
            options: options,
        }
    }
}

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

impl DHCPBytes for DHCPRequest {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.put_u8(self.op as u8);
        bytes.put_u8(self.htype as u8);
        bytes.put_u8(self.hlen as u8);
        bytes.put_u8(self.hops);
        bytes.put_u32(self.xid);
        bytes.put_u16(self.secs);
        bytes.put_u16(self.flags);
        for addrs in [self.ciaddr, self.yiaddr, self.siaddr, self.giaddr] {
            for addr in addrs {
                bytes.put_u8(addr);
            }
        }
        for ele in self.chaddr.iter() {
            bytes.put_u8(*ele);
        }
        for ele in self.sname.iter() {
            bytes.put_u8(*ele);
        }
        for ele in self.file.iter() {
            bytes.put_u8(*ele);
        }
        for ele in self.options.iter() {
            bytes.put_u8(*ele);
        }
        bytes
    }
}
