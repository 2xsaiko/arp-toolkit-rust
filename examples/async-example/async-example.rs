extern crate libarp;

use futures::executor::block_on;
use libarp::{arp::ArpMessage, client::ArpClient, interfaces::Interface, interfaces::MacAddr};
use std::net::Ipv4Addr;

fn main() {
    block_on(start());
}

async fn start() {
    let mac_addr = MacAddr::new(0xdc, 0xa6, 0x32, 0x27, 0x5b, 0xd8);
    let ip_addr = Ipv4Addr::new(10, 0, 0, 2);

    futures::join!(
        resolve_simple(mac_addr, ip_addr),
        resolve_advanced(mac_addr, ip_addr)
    );
}

async fn resolve_simple(mac_addr: MacAddr, ip_addr: Ipv4Addr) {
    let mut client = ArpClient::new();

    let result = client.mac_to_ip(mac_addr, None);
    println!(
        "Simple: IP for MAC {} is {}",
        mac_addr,
        result.await.unwrap()
    );

    let result = client.ip_to_mac(ip_addr, None);
    println!(
        "Simple: MAC for IP {} is {}",
        ip_addr,
        result.await.unwrap()
    );
}

async fn resolve_advanced(mac_addr: MacAddr, ip_addr: Ipv4Addr) {
    let iface = Interface::new_by_name("enp4s0").unwrap();
    let mut client = ArpClient::new_with_iface(&iface);

    /*
    This is just for demonstrating the API.
    The following code may not lead to the same result as the previous code,
    as checking if the ARP response is related to us (or if it even is a response) is omitted.
    One would have to implement these checks manually, similar to how it is done in the
    client's mac_to_ip and ip_to_mac methods.
    */

    let arp_request =
        ArpMessage::new_arp_request(iface.get_mac().into(), iface.get_ip().unwrap(), ip_addr);
    let result = client.send_message(None, arp_request).await.unwrap();
    println!(
        "Advanced: IP for MAC {} is {}",
        mac_addr, result.target_protocol_address
    );

    let rarp_request = ArpMessage::new_rarp_request(iface.get_mac().into(), mac_addr);
    let result = client.send_message(None, rarp_request).await.unwrap();
    println!(
        "Advanced: MAC for IP {} is {}",
        ip_addr, result.target_hardware_address
    );
}
