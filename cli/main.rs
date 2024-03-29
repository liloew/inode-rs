use anyhow::Result;
use bytes::{Bytes, BytesMut};
use packet::udp::dhcp::*;
use pnet::datalink;
use tokio::{self, net::UdpSocket};

#[tokio::main]
async fn main() -> Result<()> {
    let interfaces = datalink::interfaces();
    println!("网卡数: {}", interfaces.len());
    for interface in interfaces.into_iter() {
        println!(
            "网络 - name: {} MAC: {}",
            interface.description,
            interface.mac.unwrap_or_default()
        );
    }
    let socket = UdpSocket::bind("0.0.0.0:68").await?;
    let broadcast = socket.set_broadcast(true);
    if let Ok(r) = broadcast {
        println!("开启广播模式");
    }
    let mut buf = [0; 1024];
    let mac = "F6-6D-3F-C0-8A-6F";
    let mut v = vec![DHCPHType::ETHERNET as u8];
    let a: Vec<u8> = str::split(mac, "-")
        .map(|u| u8::from_str_radix(u, 16).unwrap())
        .collect();
    v.extend_from_slice(&a);
    let parameter_request_list: Vec<u8> = vec![
        DHCPOPTION::SubnetMask as u8,
        DHCPOPTION::RouterOption as u8,
        DHCPOPTION::DomainNameServerOption as u8,
        DHCPOPTION::NetworkTimeProtocolServersOption as u8,
    ];
    let mut discovery_message = DHCPDiscover::with_mac(mac);
    discovery_message.insert_options(vec![
        DHCPOPTIONS {
            tp: DHCPOPTION::DHCPMessageType,
            len: 1,
            va: vec![DHCPMessageType::DHCPDISCOVER as u8],
        },
        DHCPOPTIONS {
            tp: DHCPOPTION::ClientIdentifier,
            len: v.len() as u8,
            va: v.clone(),
        },
        DHCPOPTIONS {
            tp: DHCPOPTION::ParameterRequestList,
            len: parameter_request_list.len() as u8,
            va: parameter_request_list.clone(),
        },
    ]);
    let tmp_bytes = discovery_message.to_bytes();

    let broadcast_result = socket.send_to(&tmp_bytes, "255.255.255.255:67").await;
    match broadcast_result {
        Ok(n) => println!("发送数据包: {}", n),
        Err(e) => println!("错误: {:?}", e),
    }
    loop {
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        println!("BEGIN");
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        let dhcp_offer = DHCPOffer::from_bytes(&mut Bytes::from_iter(buf));
        println!("Prefer IP Address: {:?}", dhcp_offer.yiaddr);
        let dhcp_server_identifier: Vec<&DHCPOPTIONS> = dhcp_offer
            .options
            .iter()
            .filter(|&v| v.tp == DHCPOPTION::ServerIdentifier)
            .collect();
        let mut options = vec![
            DHCPOPTIONS {
                tp: DHCPOPTION::DHCPMessageType,
                len: 1,
                va: vec![DHCPMessageType::DHCPREQUEST as u8],
            },
            DHCPOPTIONS {
                tp: DHCPOPTION::ClientIdentifier,
                len: v.len() as u8,
                va: v.clone(),
            },
            DHCPOPTIONS {
                tp: DHCPOPTION::RequestedIPAddress,
                len: 4u8,
                va: dhcp_offer.yiaddr.to_vec(),
            },
            // DHCPOPTIONS {
            //     tp: DHCPOPTION::ServerIdentifier,
            //     len: 4u8,
            //     va: dhcp_offer.chaddr.to_vec(),
            // },
            DHCPOPTIONS {
                tp: DHCPOPTION::ParameterRequestList,
                len: parameter_request_list.len() as u8,
                va: parameter_request_list.clone(),
            },
        ];
        if dhcp_server_identifier.len() > 0 {
            let option = dhcp_server_identifier.get(0).unwrap();
            println!("{:?}", *option);
            options.insert(
                0,
                DHCPOPTIONS {
                    tp: DHCPOPTION::ServerIdentifier,
                    len: 4u8,
                    va: option.va.clone(),
                },
            );
        }
        let mut v = vec![DHCPHType::ETHERNET as u8];
        let a: Vec<u8> = str::split(mac, "-")
            .map(|u| u8::from_str_radix(u, 16).unwrap())
            .collect();
        v.extend_from_slice(&a);
        let mut dhcp_request = DHCPRequest::with_mac_ip_options(mac, options);
        println!("DHCP Request: {:?}", dhcp_request);
        let result = socket
            .send_to(&dhcp_request.to_bytes(), "255.255.255.255:67")
            .await;
        println!("DHCP Request: {:?}", result);
        // TODO: DHCPACK
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        let dhcp_ack = DHCPAck::from_bytes(&mut Bytes::from_iter(buf));
        println!("DHCP ACK: {:?}", dhcp_ack);
        break;
        println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
    }
    Ok(())
}
