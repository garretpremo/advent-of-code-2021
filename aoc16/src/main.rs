use std::{fs, usize};

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket)
}

impl Packet {
    fn get_length(&self) -> usize {
        match self {
            Packet::Literal(packet) => packet.length,
            Packet::Operator(packet) => packet.length
        }
    }

    fn get_version_sum(&self) -> u32 {
        match self {
            Packet::Literal(packet) => packet.version,
            Packet::Operator(packet) => packet.version_sum
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct LiteralPacket {
    version: u32,
    length: usize,
    value: u64,
}

#[derive(Eq, PartialEq, Debug)]
struct OperatorPacket {
    version: u32,
    version_sum: u32,
    length: usize,
    sub_packets: Vec<Packet>,
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-16.txt").unwrap();

    let packet = parse(input_file_contents);

    println!("answer 16.1: {}", packet.get_version_sum());
}

fn parse(input: String) -> Packet {
    let binary_packet = convert_hex_string_to_binary_string(input);

    parse_packet(binary_packet.as_str(), false)
}

fn convert_hex_string_to_binary_string(hex_string: String) -> String {
    let hex: Vec<u32> = hex_string.trim().chars().map(|c| c.to_digit(16).unwrap()).collect();

    hex.iter()
        .map(|value| format!("{:0>4b}", value))
        .fold(String::from(""), |acc, bin| format!("{}{}", acc, bin))
}

fn parse_packet(packet: &str, sub_packet: bool) -> Packet {
    let type_id = u32::from_str_radix(&packet[3..6], 2).unwrap();

    match type_id {
        4 => parse_literal(packet, sub_packet),
        _ => parse_operator(packet, sub_packet)
    }
}

fn parse_literal(packet: &str, is_sub_packet: bool) -> Packet {
    let version = u32::from_str_radix(&packet[0..3], 2).unwrap();
    let mut literal = String::from("");
    let mut packet_index = 6;

    loop {
        let end_of_packet = packet.chars().nth(packet_index).unwrap() == '0';
        let value = &packet[packet_index + 1..packet_index + 5];
        literal = format!("{}{}", literal, value);

        packet_index += 5;

        if end_of_packet {
            break;
        }
    }

    let mut length = packet_index;

    // all packets have to have a length divisible by 4
    // a sub_packet does not need to worry about padding zeros, since the outermost packet will handle padding
    if !is_sub_packet {
        length += 4 - (length % 4);
    }

    let value = u64::from_str_radix(literal.as_str(), 2).unwrap();
    Packet::Literal(LiteralPacket { version, length, value })
}

fn parse_operator(packet: &str, is_sub_packet: bool) -> Packet {
    let version = u32::from_str_radix(&packet[0..3], 2).unwrap();
    let length_type_id = u32::from_str_radix(&packet[6..=6], 2).unwrap();

    let mut sub_packet_index = match length_type_id {
        0 => 22,
        1 => 18,
        _ => panic!("invalid length_type_id")
    };

    let mut sub_packets: Vec<Packet> = vec![];
    let mut sub_packets_total_length = 0;
    let mut version_sum = version;

    match length_type_id {
        0 => { // length_id = 0 indicates the next 15 bits give the # of bits in sub-packets
            let bits_in_sub_packets = usize::from_str_radix(&packet[7..22], 2).unwrap();

            while sub_packets_total_length < bits_in_sub_packets {
                let sub_packet = parse_packet(&packet[sub_packet_index..packet.len()], true);
                let sub_packet_length = sub_packet.get_length();

                version_sum += sub_packet.get_version_sum();
                sub_packet_index += sub_packet_length;
                sub_packets_total_length += sub_packet_length;

                sub_packets.push(sub_packet);
            }
        },
        1 => { // length_id = 1 indicates that the next 15 bits give the # of sub-packets
            let number_of_sub_packets = usize::from_str_radix(&packet[7..18], 2).unwrap();

            for _ in 0..number_of_sub_packets {
                let sub_packet = parse_packet(&packet[sub_packet_index..packet.len()], true);
                let sub_packet_length = sub_packet.get_length();

                version_sum += sub_packet.get_version_sum();
                sub_packet_index += sub_packet_length;
                sub_packets_total_length += sub_packet_length;

                sub_packets.push(sub_packet);
            }
        },
        _ => panic!("invalid length_type_id")
    }

    let mut length = sub_packet_index;

    if !is_sub_packet {
        length += 4 - (length % 4);
    }

    Packet::Operator(OperatorPacket { version, version_sum, length, sub_packets })
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("");
    parse(sample_input);
}

#[test]
fn test_literal_packet() {
    let packet_hex = String::from("D2FE28");
    parse(packet_hex);
}

#[test]
fn test_small_literal_packet_as_sub_packet() {
    let packet = parse_literal("11010001010", true);

    assert_eq!(packet, Packet::Literal(LiteralPacket { version: 6, length: 11, value: 10 }));
}

#[test]
fn test_operator_packet() {
    let packet_hex = String::from("38006F45291200");

    let packet = parse(packet_hex);
    println!("{:?}", packet);
    assert_eq!(packet.get_length(), 52);
}

#[test]
fn test_operator_packet_2() {
    let packet_hex = String::from("EE00D40C823060");

    let packet = parse(packet_hex);
    println!("{:?}", packet);
    assert_eq!(packet.get_length(), 52);
}

#[test]
fn test_operator_packet_3() {
    let packet_hex = String::from("8A004A801A8002F478");

    let packet = parse(packet_hex.clone());
    assert_eq!(packet.get_version_sum(), 16)
}

#[test]
fn test_operator_packet_4() {
    let packet_hex = String::from("620080001611562C8802118E34");

    let packet = parse(packet_hex.clone());
    assert_eq!(packet.get_version_sum(), 12)
}

#[test]
fn test_operator_packet_5() {
    let packet_hex = String::from("C0015000016115A2E0802F182340");

    let packet = parse(packet_hex.clone());
    assert_eq!(packet.get_version_sum(), 23)
}

#[test]
fn test_operator_packet_6() {
    let packet_hex = String::from("A0016C880162017C3686B18A3D4780");

    let packet = parse(packet_hex.clone());
    assert_eq!(packet.get_version_sum(), 31)
}

#[test]
fn test_convert_hex_to_binary() {
    assert_eq!(convert_hex_string_to_binary_string(String::from("1")), String::from("0001"));
    assert_eq!(convert_hex_string_to_binary_string(String::from("2")), String::from("0010"));
    assert_eq!(convert_hex_string_to_binary_string(String::from("3")), String::from("0011"));
    assert_eq!(convert_hex_string_to_binary_string(String::from("F")), String::from("1111"));
    assert_eq!(convert_hex_string_to_binary_string(String::from("1111")), String::from("0001000100010001"));
}
