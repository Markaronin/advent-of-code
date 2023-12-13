use itertools::{Itertools, PeekingNext};
use std::collections::HashMap;

use advent_of_code_util::parse::read_lines;

#[derive(Debug)]
struct Packet {
    version: usize,
    data: PacketData,
}
impl Packet {
    fn read_next_packet<I>(chars: &mut I) -> Self
    where
        I: Iterator<Item = char> + PeekingNext,
    {
        let version = usize::from_str_radix(&chars.take(3).collect::<String>(), 2).unwrap();
        let type_id = usize::from_str_radix(&chars.take(3).collect::<String>(), 2).unwrap();
        let data = PacketData::from_chars(type_id, chars);

        Packet { version, data }
    }

    fn sum_versions(&self) -> usize {
        self.version
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::Operator(_, packets) => {
                    packets.iter().map(|packet| packet.sum_versions()).sum()
                }
            }
    }

    fn get_val(&self) -> usize {
        self.data.get_val()
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}
impl PacketData {
    fn from_chars<I>(type_id: usize, chars: &mut I) -> Self
    where
        I: Iterator<Item = char> + PeekingNext,
    {
        match type_id {
            4 => PacketData::literal_from_chars(chars),
            other => PacketData::operator_from_chars(other, chars),
        }
    }

    fn get_val(&self) -> usize {
        match self {
            PacketData::Literal(val) => *val,
            PacketData::Operator(operator_type, packets) => match operator_type {
                0 => packets.iter().map(|packet| packet.get_val()).sum(),
                1 => packets.iter().map(|packet| packet.get_val()).product(),
                2 => packets.iter().map(|packet| packet.get_val()).min().unwrap(),
                3 => packets.iter().map(|packet| packet.get_val()).max().unwrap(),
                5 => {
                    if packets[0].get_val() > packets[1].get_val() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].get_val() < packets[1].get_val() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].get_val() == packets[1].get_val() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        }
    }

    fn literal_from_chars<I>(chars: &mut I) -> Self
    where
        I: Iterator<Item = char> + PeekingNext,
    {
        let mut i = 0;
        let mut done = false;
        PacketData::Literal(
            usize::from_str_radix(
                &chars
                    .peeking_take_while(|ch| {
                        let ret_val = if i % 5 == 0 {
                            if done {
                                false
                            } else if *ch == '0' {
                                done = true;
                                true
                            } else {
                                true
                            }
                        } else {
                            true
                        };
                        i += 1;
                        ret_val
                    })
                    .chunks(5)
                    .into_iter()
                    .map(|mut ch| {
                        ch.next(); // Skip the first bit
                        ch.take(4)
                    })
                    .flatten()
                    .collect::<String>(),
                2,
            )
            .unwrap(),
        )
    }

    fn operator_from_chars<I>(type_id: usize, chars: &mut I) -> Self
    where
        I: Iterator<Item = char> + PeekingNext,
    {
        let length_type_id = chars.next().unwrap();
        PacketData::Operator(
            type_id,
            match length_type_id {
                '0' => {
                    let total_subpacket_len =
                        usize::from_str_radix(&chars.take(15).collect::<String>(), 2).unwrap();
                    let mut subpacket_bytes = chars.take(total_subpacket_len).collect::<String>();
                    let mut sub_packets = vec![];
                    while !subpacket_bytes.is_empty() {
                        let mut sb_iter = subpacket_bytes.chars();
                        sub_packets.push(Packet::read_next_packet(&mut sb_iter));
                        subpacket_bytes = sb_iter.collect();
                    }
                    sub_packets
                }
                '1' => {
                    let num_sub_packets =
                        usize::from_str_radix(&chars.take(11).collect::<String>(), 2).unwrap();
                    let mut sub_packets = vec![];
                    for _ in 0..num_sub_packets {
                        sub_packets.push(Packet::read_next_packet(chars));
                    }
                    sub_packets
                }
                _ => panic!(),
            },
        )
    }
}

fn convert_to_binary(hex_string: &str) -> String {
    let hex_to_bin: HashMap<char, &str> = [
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]
    .iter()
    .cloned()
    .collect();

    #[allow(unstable_name_collisions)]
    hex_string
        .chars()
        .map(|ch| hex_to_bin.get(&ch).unwrap())
        .cloned()
        .intersperse("")
        .collect()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)[0].clone();
    let bin = convert_to_binary(&input);

    let packet = Packet::read_next_packet(&mut bin.chars());

    (packet.sum_versions(), packet.get_val())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 23);
        assert_eq!(part_2_output, 46);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
