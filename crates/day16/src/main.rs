use std::usize;

use utils::InputType;

fn main() {
    let packet = read_input(InputType::Input);
    println!("Day16 part a = {}", part_a(&packet)); // 879
    println!("Day16 part b = {}", part_b(&packet)); // 539051801941
}

fn part_a(packet: &Packet) -> u32 {
    packet.sum_versions()
}

fn part_b(packet: &Packet) -> usize {
    packet.eval()
}

#[derive(Debug, PartialEq)]
pub enum TypeId {
    Literal(usize),
    Operator((Op, Vec<Packet>)),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    GT = 5,
    LT = 6,
    EQ = 7,
}

impl Op {
    pub fn new(v: u32) -> Op {
        match v {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Min,
            3 => Op::Max,
            5 => Op::GT,
            6 => Op::LT,
            7 => Op::EQ,
            _ => panic!("not a valid op code"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: u32,
    type_id: TypeId,
}

impl Packet {
    fn new(bits: Vec<char>) -> Packet {
        let (_rest, packet) = Packet::parse(bits);
        packet
    }

    fn parse(bits: Vec<char>) -> (Vec<char>, Packet) {
        let (vs, bits) = bits.split_at(3);
        let version = chars_to_u32(vs);

        let (ts, bits) = bits.split_at(3);
        let type_id_num = chars_to_u32(ts);

        match type_id_num {
            4 => {
                let mut vs: Vec<String> = vec![];
                let mut bs = bits.clone();
                loop {
                    let (ds, temp) = bs.split_at(5);
                    let (last, cs) = ds.split_at(1);
                    let cs: String = cs.iter().collect();
                    vs.push(cs);
                    if last[0] == '0' {
                        break;
                    }
                    bs = temp;
                }
                let s: String = vs.iter().map(|s| s.clone()).collect();
                let v: usize = usize::from_str_radix(&s, 2).unwrap();

                let (_, bits) = bits.split_at(vs.len() * 5);
                let type_id = TypeId::Literal(v);
                let packet = Packet { version, type_id };
                (bits.to_owned(), packet)
            }
            _ => {
                let (ls, bits) = bits.split_at(1);
                let length_type_id = chars_to_u32(ls);
                match length_type_id {
                    0 => {
                        let (ls, bits) = bits.split_at(15);
                        let total_length = chars_to_u32(ls);
                        let (to_parse, bits) = bits.split_at(total_length as usize);
                        let mut cs: Vec<char> = to_parse.iter().map(|c| *c).collect();
                        let mut ps: Vec<Packet> = vec![];
                        loop {
                            let (temp, p) = Packet::parse(cs.clone());
                            ps.push(p);
                            cs = temp;
                            if cs.len() == 0 {
                                break;
                            }
                        }
                        let type_id = TypeId::Operator((Op::new(type_id_num), ps));
                        let packet = Packet { version, type_id };
                        (bits.to_owned(), packet)
                    }
                    1 => {
                        let (ls, bits) = bits.split_at(11);
                        let sub_packets = chars_to_u32(ls) as usize;
                        let mut cs: Vec<char> = bits.iter().map(|c| *c).collect();
                        let mut ps: Vec<Packet> = vec![];
                        for _ in 0..sub_packets {
                            let (temp, p) = Packet::parse(cs.clone());
                            ps.push(p);
                            cs = temp;
                        }

                        let type_id = TypeId::Operator((Op::new(type_id_num), ps));
                        let packet = Packet { version, type_id };
                        (cs.to_owned(), packet)
                    }
                    _ => panic!("parse failure, invalid length type id"),
                }
            }
        }
    }

    fn sum_versions(&self) -> u32 {
        if let TypeId::Operator((_, ps)) = &self.type_id {
            let cnt: u32 = ps.iter().map(Packet::sum_versions).sum();
            cnt + self.version
        } else {
            self.version
        }
    }

    fn eval(&self) -> usize {
        match &self.type_id {
            TypeId::Literal(v) => *v,
            TypeId::Operator((op, ps)) => {
                let vs: Vec<usize> = ps.iter().map(Packet::eval).collect();
                match &op {
                    Op::Sum => vs.into_iter().sum(),
                    Op::Product => vs.into_iter().fold(1, |acc, v| acc * v),
                    Op::Min => vs
                        .into_iter()
                        .fold(usize::MAX, |acc, v| if v < acc { v } else { acc }),
                    Op::Max => vs
                        .into_iter()
                        .fold(usize::MIN, |acc, v| if v > acc { v } else { acc }),
                    Op::GT => vs[0].gt(&vs[1]) as usize,
                    Op::LT => vs[0].lt(&vs[1]) as usize,
                    Op::EQ => vs[0].eq(&vs[1]) as usize,
                }
            }
        }
    }
}

fn read_input(input_type: InputType) -> Packet {
    let data = {
        match input_type {
            InputType::Sample => include_str!("sample.txt"),
            InputType::Input => include_str!("input.txt"),
        }
    };
    Packet::new(decode_hex(data))
}

pub fn decode_hex_char(c: &char) -> Option<Vec<char>> {
    match c {
        '0' => Some(vec!['0', '0', '0', '0']),
        '1' => Some(vec!['0', '0', '0', '1']),
        '2' => Some(vec!['0', '0', '1', '0']),
        '3' => Some(vec!['0', '0', '1', '1']),
        '4' => Some(vec!['0', '1', '0', '0']),
        '5' => Some(vec!['0', '1', '0', '1']),
        '6' => Some(vec!['0', '1', '1', '0']),
        '7' => Some(vec!['0', '1', '1', '1']),
        '8' => Some(vec!['1', '0', '0', '0']),
        '9' => Some(vec!['1', '0', '0', '1']),
        'A' => Some(vec!['1', '0', '1', '0']),
        'B' => Some(vec!['1', '0', '1', '1']),
        'C' => Some(vec!['1', '1', '0', '0']),
        'D' => Some(vec!['1', '1', '0', '1']),
        'E' => Some(vec!['1', '1', '1', '0']),
        'F' => Some(vec!['1', '1', '1', '1']),
        _ => None,
    }
}

pub fn decode_hex(s: &str) -> Vec<char> {
    s.chars()
        .map(|c| decode_hex_char(&c).unwrap())
        .flat_map(|cs| cs)
        .collect()
}

pub fn chars_to_u32(cs: &[char]) -> u32 {
    let s: String = cs.to_owned().iter().collect();
    u32::from_str_radix(&s, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{decode_hex, Op, Packet, TypeId, TypeId::Literal, TypeId::Operator};

    #[test]
    fn test_parse1() {
        let s = "D2FE28";
        let bs = decode_hex(s);
        let bs_should_be: Vec<char> = "110100101111111000101000".chars().collect();
        assert_eq!(bs_should_be, bs);

        let packet = Packet::new(bs);
        println!("{:?}", packet);
        if let TypeId::Literal(v) = packet.type_id {
            assert_eq!(2021, v)
        } else {
            assert!(false, "not a literal")
        }
    }

    #[test]
    fn test_parse2() {
        let s = "38006F45291200";
        let bs = decode_hex(s);

        let packet = Packet::new(bs);
        let s = format!("{:?}", packet);
        println!("{}", s);
        let should_be = Packet {
            version: 1,
            type_id: Operator((
                Op::LT,
                vec![
                    Packet {
                        version: 6,
                        type_id: Literal(10),
                    },
                    Packet {
                        version: 2,
                        type_id: Literal(20),
                    },
                ],
            )),
        };
        assert_eq!(should_be, packet)
    }

    #[test]
    fn test_parse3() {
        let s = "EE00D40C823060";
        let bs = decode_hex(s);

        let packet = Packet::new(bs);
        let s = format!("{:?}", packet);
        println!("{}", s);
        let should_be = Packet {
            version: 7,
            type_id: Operator((
                Op::Max,
                vec![
                    Packet {
                        version: 2,
                        type_id: Literal(1),
                    },
                    Packet {
                        version: 4,
                        type_id: Literal(2),
                    },
                    Packet {
                        version: 1,
                        type_id: Literal(3),
                    },
                ],
            )),
        };
        assert_eq!(should_be, packet)
    }

    #[test]
    fn test_parse4() {
        let s = "8A004A801A8002F478";
        let bs = decode_hex(s);
        let packet = Packet::new(bs);
        let version_sum = packet.sum_versions();
        assert_eq!(16, version_sum);

        let s = "620080001611562C8802118E34";
        let bs = decode_hex(s);
        let packet = Packet::new(bs);
        let version_sum = packet.sum_versions();
        assert_eq!(12, version_sum);

        let s = "C0015000016115A2E0802F182340";
        let bs = decode_hex(s);
        let packet = Packet::new(bs);
        let version_sum = packet.sum_versions();
        assert_eq!(23, version_sum);

        let s = "A0016C880162017C3686B18A3D4780";
        let bs = decode_hex(s);
        let packet = Packet::new(bs);
        let version_sum = packet.sum_versions();
        assert_eq!(31, version_sum);
    }

    #[test]
    fn test_eval() {
        let s = "C200B40A82";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(3, packet.eval());

        let s = "04005AC33890";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(54, packet.eval());

        let s = "880086C3E88112";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(7, packet.eval());

        let s = "CE00C43D881120";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(9, packet.eval());

        let s = "D8005AC2A8F0";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(1, packet.eval());

        let s = "F600BC2D8F";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(0, packet.eval());

        let s = "9C005AC2F8F0";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(0, packet.eval());

        let s = "9C0141080250320F1802104A08";
        let packet = Packet::new(decode_hex(s));
        assert_eq!(1, packet.eval());
    }
}
