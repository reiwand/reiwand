use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

#[derive(Debug, thiserror::Error)]
#[error("{0} has only private, local or reserved addresses: a plugin reaches only public hosts")]
pub struct Private(pub String);

pub struct Public;

impl Resolve for Public {
    fn resolve(&self, name: Name) -> Resolving {
        let host = name.as_str().to_owned();
        Box::pin(async move {
            let found = tokio::net::lookup_host((host.as_str(), 0)).await?;
            let kept: Vec<SocketAddr> = found.filter(|a| public(a.ip())).collect();
            if kept.is_empty() {
                return Err(Private(host).into());
            }
            Ok(Box::new(kept.into_iter()) as Addrs)
        })
    }
}

fn public_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    let reserved = match a {
        0 | 10 | 127 => true,
        100 => (64..128).contains(&b),
        169 => b == 254,
        172 => (16..32).contains(&b),
        192 => b == 168 || b == 0 && (c == 0 || c == 2),
        198 => b == 18 || b == 19 || b == 51 && c == 100,
        203 => b == 0 && c == 113,
        _ => a >= 224,
    };
    !reserved
}

pub fn public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => public_v4(ip),
        IpAddr::V6(ip) => match ip.to_ipv4_mapped() {
            Some(v4) => public_v4(v4),
            None => public_v6(ip),
        },
    }
}

fn public_v6(ip: Ipv6Addr) -> bool {
    let first = ip.segments()[0];
    let translated = ip.segments()[..6] == [0x64, 0xff9b, 0, 0, 0, 0];
    let local = ip.is_loopback() || ip.is_unspecified() || ip.is_multicast();
    let ranges = (first & 0xfe00) == 0xfc00
        || (first & 0xffc0) == 0xfe80
        || first == 0x2001 && ip.segments()[1] == 0xdb8;
    !local && !ranges && !translated && (first & 0xe000) == 0x2000
}
