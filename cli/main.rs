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
    let mut discovery_message = DHCPDiscover::with_mac("F6-6D-3F-C0-8A-6E");
    discovery_message.insert_options(vec![
        DHCPOPTIONS {
            tp: DHCPOPTION::DHCPMessageType,
            len: 1,
            va: vec![DHCPMessageType::DHCPDISCOVER as u8],
        },
        DHCPOPTIONS {
            tp: DHCPOPTION::DomainNameServerOption,
            len: 1,
            va: vec![DHCPMessageType::DHCPDISCOVER as u8],
        },
    ]);
    let tmp_bytes = discovery_message.to_bytes();

    let broadcast_result = socket.send_to(&tmp_bytes, "255.255.255.255:67").await;
    match broadcast_result {
        Ok(n) => println!("发送数据包: {}", n),
        Err(e) => println!("错误: {:?}", e),
    }
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
    }
}
