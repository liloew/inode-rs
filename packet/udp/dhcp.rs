use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MessageType {
    DHCPDiscover,
    DHCPOffer,
    DHCPRequest,
    DHCPDecline,
    DHCPAck,
    DHCPNak,
    DHCPRelease,
    DHCPInform,
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

#[derive(Debug, Clone, Copy)]
pub struct DHCPDiscover {
    pub op: DHCPOP,
    pub htype: DHCPHType,
    pub hlen: DHCPHLen,
    pub hops: u8,
    // TODO: 重新定义 xid
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: u32,
    pub yiaddr: u32,
    pub siaddr: u32,
    pub giaddr: u32,
    // pub mac: String,
    pub chaddr: [u8; 16],
    // mac -> hex format!("{:0<8}", "110").parse::<u32>().unwrap()
    // FIXME: chaddr: 6*8 + 10*8
    // pub chaddr: u128,
    pub sname: [u8; 64],
    pub file: [u8; 128],
    pub options: [u8; 312],
    // TODO: ff ending
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
            // chaddr: 0x00,
            chaddr: [0x00; 16],
            sname: [0; 64],
            file: [0; 128],
            options: [0; 312],
        }
    }
}

impl DHCPDiscover {
    pub fn with_mac(mac: &str) -> Self {
        // TODO: Vec padding zero
        /*
        let mut mac_addr: Vec<_> = mac.to_owned().split(":")
                    .flat_map(|pair| hex::decode(pair).expect("MAC address contains unexpected characters"))
                    .collect();
        mac_addr.resize(16 - mac_addr.len(), 0);
        let mut dhcp_discover = DHCPDiscover::default();
        // dhcp_discover.chaddr = mac_addr.try_into().unwrap_or_else(|v| v);
        dhcp_discover.chaddr = mac_addr.try_into().unwrap();
        dhcp_discover
        */
        /*
        let mut mac_addr: [u8; 6];
        hex::decode_to_slice(mac, &mut mac_addr);
        */

        /*
        let mac_addr = if mac.contains("-") {
            str::split(mac, "-").collect::<String>()
        } else {
            str::split(mac, ":").collect::<String>()
        };
        println!("mac:{:?} hex: {:?}", &mac_addr, hex::decode(&mac_addr).unwrap());
        */
        let mut mac_addr: Vec<u8> = if mac.contains("-") {
            str::split(mac, "-").map(|u| u8::from_str_radix(u, 16).unwrap()).collect()
        } else {
            str::split(mac, ":").map(|u| u8::from_str_radix(u, 16).unwrap()).collect()
        };
        mac_addr.resize(16, 0);
        let mut discover = DHCPDiscover::default();
        discover.chaddr = mac_addr.try_into().unwrap();
        discover
    }

    // FIXME: Vec<Options>
    pub fn insert_options(&mut self, options: Vec<&str>) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPOffer {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPRequest {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPDecline {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPAck {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPNak {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPRelease {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DHCPInform {
}


impl DHCPBytes for DHCPDiscover {
    fn to_bytes(&self) -> BytesMut {
    /*
    pub op: DHCPOP,
    pub htype: DHCPHType,
    pub hlen: DHCPHLen,
    pub hops: u8,
    // TODO: 重新定义 xid
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: u32,
    pub yiaddr: u32,
    pub siaddr: u32,
    pub giaddr: u32,
    // pub mac: String,
    pub chaddr: [u8; 16],
    // mac -> hex format!("{:0<8}", "110").parse::<u32>().unwrap()
    // FIXME: chaddr: 6*8 + 10*8
    // pub chaddr: u128,
    pub sname: [u8; 64],
    pub file: [u8; 128],
    pub options: [u8; 312],
    // TODO: ff ending
    */
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
        for ele in self.options.iter() {
            discover_bytes.put_u8(*ele);
        }
        discover_bytes
        /*
        let mac: Vec<_> = self.mac.split(":")
                    .flat_map(|pair| hex::decode(pair).expect("MAC address contains unexpected characters"))
                    .collect();
        let mut dicover_bytes = BytesMut::new();
        dicover_bytes.extend(&[0x01, 0x01]);
        dicover_bytes.put_u8(mac.len() as u8);
        dicover_bytes.put_u8(0x00);
        // 0x99, 0xc0, 0xcd, 0x35 - xid
        dicover_bytes.extend(self.xid);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        // 0x08, 0x00, 0x27, 0x0f, 0xbe, 0x55
        dicover_bytes.extend(mac.as_slice());
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x82, 0x53, 0x63]);
        // dicover_bytes.extend(&[0x35, 0x01, 0x01, 0x3d, 0x07, 0x01, 0x08, 0x00, 0x27, 0x0f, 0xbe, 0x55, 0x32, 0x04, 0xc0, 0xa8]);
        dicover_bytes.extend(&[0x35, 0x01, 0x01, 0x3d, 0x07, 0x01]);
        dicover_bytes.put_slice(mac.as_slice());
        // 0x32, 0x04, 0xc0, 0xa8, 0x38, 0x66
        dicover_bytes.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);    // requested IP address
        dicover_bytes.extend(&[0x0c, 0x0f, 0x44, 0x45, 0x53, 0x4b, 0x54, 0x4f, 0x50, 0x2d, 0x52, 0x41, 0x44, 0x44]);
        dicover_bytes.extend(&[0x4c, 0x4a, 0x45, 0x3c, 0x08, 0x4d, 0x53, 0x46, 0x54, 0x20, 0x35, 0x2e, 0x30, 0x37, 0x0e, 0x01]);
        dicover_bytes.extend(&[0x03, 0x06, 0x0f, 0x1f, 0x21, 0x2b, 0x2c, 0x2e, 0x2f, 0x77, 0x79, 0xf9, 0xfc, 0xff]);
        dicover_bytes
        */
    }
}
