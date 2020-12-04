use regex::Regex;
use std::collections::HashMap;

pub enum Field {
    Year(u16),
    Unit(u16),
    Color(String),
    Id(u64),
    ParseError(String),
}

pub struct Passport {
    fields: HashMap<String, Field>,
}

impl Passport {
    const MANDANTORY_FIELDS: &'static [&'static str] =
        &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn is_valid(&self) -> bool {
        Passport::MANDANTORY_FIELDS
            .iter()
            .all(|field| self.fields.contains_key(&field.to_string()))
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Passport> {
    let re = Regex::new(r" |\n").unwrap();

    input
        .split("\n\n")
        .map(|block| {
            let mut fields = HashMap::<String, Field>::new();

            for field in re.split(block) {
                let key_value = field.split(":").collect::<Vec<_>>();
                let key = key_value[0];
                let value = key_value[1];
                let parsed_value = match key {
                    "byr" | "iyr" | "eyr" => value.parse::<u16>().map(|year| Field::Year(year)),
                    "hgt" => value.parse::<u16>().map(|year| Field::Year(year)),
                    "hcl" | "ecl" => value.parse::<u16>().map(|year| Field::Year(year)),
                    "pid" | "cid" => value.parse::<u64>().map(|year| Field::Id(year)),
                    _ => panic!("Invalid Field"),
                }
                .unwrap_or(Field::ParseError(value.to_string()));

                fields.insert(key.to_string(), parsed_value);
            }
            Passport { fields }
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}
