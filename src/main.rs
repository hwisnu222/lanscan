use std::{net::{IpAddr, Ipv4Addr}, thread, time::Duration};

use dns_lookup::lookup_addr;
use netdev::ipnet::Ipv4Net;
use netscan::{host::Host, scan::{scanner::HostScanner, setting::{HostScanSetting, HostScanType}}};

fn main() {
    let interface = netdev::get_default_interface().unwrap();
    let mut scan_setting = HostScanSetting::default()
        .with_if_index(interface.index)
        .with_scan_type(HostScanType::IcmpPingScan)
        .with_timeout(Duration::from_millis(10000))
        .with_wait_time(Duration::from_millis(500));

    let src_ip: Ipv4Addr = interface.ipv4[0].addr();
    let net: Ipv4Net = Ipv4Net::new(src_ip, 24).unwrap();
    let nw_addr = Ipv4Net::new(net.network(), 24).unwrap();

    let mut handles = vec![];

    for host in nw_addr.hosts(){
        let ipv4 = IpAddr::V4(host);

        let handle = thread::spawn(move ||{
            let hostname = match lookup_addr(&ipv4) {
                Ok(name)=> name,
                Err(_) => format!("unknow-host-{}", host)
            };

            (ipv4, hostname)
        });

        handles.push(handle);
    };

    for handle in handles{
        if let Ok((ipv4, hostname)) = handle.join(){
            scan_setting.add_target(Host::new(ipv4, hostname));
        }
    }

    let result = HostScanner::new(scan_setting).scan();

    println!("Status: {:?}", result.scan_status);
    println!("Up hosts:");
    for host in result.hosts{
        println!("{:?}", host);
    }
}
