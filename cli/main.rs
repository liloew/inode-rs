use pnet::datalink;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let interfaces = datalink::interfaces();
    for interface in interfaces.into_iter() {
        println!("网络 - name: {} MAC: {}", interface.description, interface.mac.unwrap_or_default());
    }
    Ok(())
}
