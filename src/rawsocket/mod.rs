use crate::rawsocket::ip_header::IPHeader;
use crate::rawsocket::tcp_header::TCPHeader;

mod ip_flags;
pub mod ip_header;
pub mod socket;
mod tcp_flags;
pub mod tcp_header;

// -- Public interface for rawsocket --

pub fn pack(ip: &mut IPHeader, tcp: &TCPHeader, payload: &Vec<u8>) -> Vec<u8> {
    // Calculate total length of packet [IP header + TCP header + payload]
    let tcp_header_len = tcp.data_offset as usize * 4;
    ip.tot_len = (20 + tcp_header_len + payload.len()) as u16;

    // Allocate entire vector
    let mut packet = vec![0u8; ip.tot_len as usize];

    // Copy IP header bytes into packet
    packet[0..20].copy_from_slice(&ip.to_bytes());

    // Copy TCP header bytes into packet
    let payload_idx = 20 + tcp_header_len;
    packet[20..payload_idx].copy_from_slice(&tcp.to_bytes(ip, payload));

    // Copy payload into packet
    packet[payload_idx..].copy_from_slice(payload);

    packet
}

// pub fn unpack(packet: &[u8]) -> Result<(&IPHeader, &TCPHeader, &Vec<u8>), &'static str> {
//     if packet.len() < 20 {
//         return Err("Incomplete TCP/IP packet");
//     }
//
//     let ip_header_bytes = &packet[0..20];
//     if IPHeader::checksum(&ip_header_bytes) != 0 {
//         return Err("IP checksum failed");
//     }
//
//     let ip = IPHeader::from_bytes(ip_header_bytes);
//
//     let tcp_header_bytes = &packet[20..];
//     if TCPHeader::checksum(tcp_header_bytes, &ip) != 0 {
//         return Err("TCP checksum failed");
//     }
//
//     let tcp = TCPHeader::from_bytes(&packet[20..]);
// }

// -- Unit test helpers --

pub mod test_utils {
    pub fn get_ip_hex() -> &'static str {
        "45000040000040004006d3760a6ed06acc2cc03c"
    }

    pub fn get_tcp_hex() -> &'static str {
        "c6b70050a4269c9300000000b002ffff92970000020405b4010303060101080abb6879f80000000004020000"
    }

    pub fn get_ip_hex_with_payload() -> &'static str {
        "45000592464440002a069de0cc2cc03c0a6ed06a"
    }

    pub fn get_tcp_hex_with_payload() -> &'static str {
        "0050c6b762a01b47a4269e88801000eb71aa00000101080abeb95f0abb687a45"
    }

    pub fn giant_payload() -> &'static str {
        "485454502f312e3120323030204f4b0d0a446174653a205468752c203331204d61722\
        0323032322032303a35383a303220474d540d0a5365727665723a204170616368650d0a55706772616465\
        3a2068322c6832630d0a436f6e6e656374696f6e3a20557067726164652c204b6565702d416c6976650d0\
        a566172793a204163636570742d456e636f64696e672c557365722d4167656e740d0a436f6e74656e742d\
        456e636f64696e673a20677a69700d0a4b6565702d416c6976653a2074696d656f75743d322c206d61783\
        d3130300d0a5472616e736665722d456e636f64696e673a206368756e6b65640d0a436f6e74656e742d54\
        7970653a20746578742f68746d6c3b20636861727365743d5554462d380d0a0d0a316661610d0a1f8b080\
        00000000000036c8f414bc4301085effe8a31e7b65b4151966641da154f5a582f1e43326947d3a424d3d6\
        9f6fbb2be8c1d3e3cdc0f7deabae9bd7faedbd3d42cf833b5c559b8053be9302bdd80ea8cc2a03b202dda\
        b9890a598d8e60fdb97891d1eda183e5033dceea13ec1dd7d59c2d3e48d1ad0b3720982853a0ce3c418e1\
        057909f1937cb78746cd64a0ee83b51e53066d5f3445b5bb407f32fd4a9162265cc61059800e9e57ac140\
        b19eea5c19934e66793017962522e4f5a39943745b9753c57bf600c261d69640afe0fe9390c38aa0ec186\
        f87fa70c1e530a9a1423ac632dae2eae69bfb34e9ad06bcce0f8857afa6693ec791a868130bcf32b8ec98\
        9444281818faa434519582a964e55551d5793045a3bd8e78888fe78ee1a86aaea14f93eecf779df70d359\
        9835414c9139c1e7dac273ff6ec53e4aa1e11ed06de4aaa643eae1d54561167b0019e68229a64731cbc1c\
        2c94d21cac2090926ae7d3882d0fe6521e4ca07dcb7e21adb1fbefec40e87aa8c5c007418605de1374c86\
        cf7e0fcbd5581a5a2cdb14eb6c69d612f394c827c7e60acc625adc3edc8d1e47f7c58d59e5e7a6677e878\
        d9b4b5aba40ff9996e477e71638207dbd89e71aec614004641fc9916693e5f02be7416b85a274e329e9df\
        5452b012c2cbd6ea29330398c9c75061a9d0326b4eb0cda189b177245d0ec9aa7ed08d18b494999ab98d4\
        f0626472f6d3da18a29dbe0ff98a87ade0661203ad7bfe2c44292169ca903495a295dab4eddaa0e062e0e\
        8dc321ce1565c87fef191325133c73dce97d9c3d55e4e015e642ad995d0a45c485d6c330a44b788434b74\
        4d661665ae346df541c04d032e987d33835c8cff78c2cfa990eefc74f63838437625febef0d70de995ef8\
        7e508d79d332f67e8f12565c58f3043cf971592ee4a9b63a4a926562b6408904bc23b01f1d3284d3ad6bd\
        a131c7b3cec92c85beb3a2c627e6f9a2e893c8b4d9dae986f281794408f6e97c49e47441fb237a1175552\
        3dcee675a6ae65cd334f5d01cfebee6f037a35bd8027389b1382047d5a68498e5c0d96c038371d0e660c4\
        5e17b49ded3f9ba44d2ac1405575a3d5cfbc7827984ba282553de7e39fc142e8bd8fb9fb46ad94d180682\
        67f215dbf65ad3d028290d522e88aa48edad37c4c1344e70e53ce404a8c4cf77d60e121c2a2171f57a77d\
        6bb3363248c6bb9e7dc6350495bea3aa59020a3c6efa592bfde45529a86dc2c2a617943bea8a5b5cde1a6\
        dc0c433f2b1001844207e31b130546aeac28ade2115ed7e488c92ea4d125def30d8e287b5692c69bbe46e\
        fc3bb478569649f9251457fba25acca81067e3736c56273177017ad2eb73d46501c039fe80e6641dbc090\
        a00cbe6e247bdd2c70a194c4e4d9cdce372f182815133f4f0ff1902451349f3b9476b70134d181ad1c0b7\
        c8987b9732023a32fa2f1b015509cd97c2b93f1f0ae6dea0eedff47eac0ebe7fdebf323a66eabab47f745\
        2c17899852b76bf9476262fa0bca38531a5406e1ad74"
    }
}

// -- Unit tests --

mod tests {
    use super::*;
    use crate::rawsocket::ip_flags::IPFlags;
    use crate::rawsocket::tcp_flags::TCPFlags;
    use std::net::Ipv4Addr;

    #[test]
    fn test_pack() {
        let ip_bytes = hex::decode(test_utils::get_ip_hex_with_payload()).unwrap();
        let tcp_bytes = hex::decode(test_utils::get_tcp_hex_with_payload()).unwrap();
        let payload = hex::decode(test_utils::giant_payload()).unwrap();

        let mut ip = IPHeader {
            version: 4,
            ihl: 5,
            tos: 0,
            tot_len: 1426,
            id: 17988,
            flags: IPFlags::DF,
            frag_offset: 0,
            ttl: 42,
            protocol: 6,
            checksum: 40416,
            src_ip: Ipv4Addr::new(204, 44, 192, 60),
            dst_ip: Ipv4Addr::new(10, 110, 208, 106),
        };

        let tcp = TCPHeader {
            src_port: 80,
            dst_port: 50871,
            seq_num: 1654659911,
            ack_num: 2753994376,
            data_offset: 8,
            reserved: 0,
            flags: TCPFlags::ACK,
            window: 235,
            checksum: 29098,
            urgent: 0,
            options: hex::decode("0101080abeb95f0abb687a45").unwrap(),
        };

        let packet = pack(&mut ip, &tcp, &payload);
        let expected = [ip_bytes, tcp_bytes, payload].concat();
        assert_eq!(expected, packet);
    }
}
