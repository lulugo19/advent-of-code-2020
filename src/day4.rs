use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<String, String>,
}

lazy_static! {
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}

impl Passport {
    const MANDANTORY_FIELDS: &'static [&'static str] =
        &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn has_all_mandatory_fields(&self) -> bool {
        Passport::MANDANTORY_FIELDS
            .iter()
            .all(|field| self.fields.contains_key(&field.to_string()))
    }

    fn is_valid(&self) -> bool {
        Passport::MANDANTORY_FIELDS.iter().all(|key| {
            self.fields
                .get(&key.to_string())
                .map(|field| self.validate_field(key, field))
                .unwrap_or(false)
        })
    }

    fn validate_field(&self, key: &str, value: &str) -> bool {
        match key {
            "byr" => self.validate_year(value, 1920, 2002),
            "iyr" => self.validate_year(value, 2010, 2020),
            "eyr" => self.validate_year(value, 2020, 2030),
            "hgt" => self.validate_height(value),
            "hcl" => self.validate_hair_color(value),
            "ecl" => self.validate_eye_color(value),
            "pid" => self.validate_passport_id(value),
            _ => panic!("Invalid key: `{}`", key),
        }
    }

    fn validate_year(&self, value: &str, min_year: u16, max_year: u16) -> bool {
        if value.len() != 4 {
            false
        } else {
            value
                .parse::<u16>()
                .map(|year| year >= min_year && year <= max_year)
                .unwrap_or(false)
        }
    }

    fn validate_height(&self, value: &str) -> bool {
        if let Ok(height) = value[..value.len() - 2].parse::<u32>() {
            match &value[value.len() - 2..] {
                "cm" => height >= 150 && height <= 193,
                "in" => height >= 59 && height <= 76,
                _ => false,
            }
        } else {
            false
        }
    }

    fn validate_hair_color(&self, value: &str) -> bool {
        HAIR_COLOR_REGEX.is_match(value)
    }

    fn validate_eye_color(&self, value: &str) -> bool {
        match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    fn validate_passport_id(&self, value: &str) -> bool {
        value.len() == 9 && value.chars().all(|c| c.is_numeric())
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Passport> {
    let re = Regex::new(r" |\n").unwrap();

    input
        .split("\n\n")
        .map(|block| {
            let mut fields = HashMap::<String, String>::new();

            for field in re.split(block) {
                let key_value = field.split(":").collect::<Vec<_>>();
                let key = key_value[0];
                let value = key_value[1];
                fields.insert(key.to_string(), value.to_string());
            }
            Passport { fields }
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(passports: &[Passport]) -> usize {
    count_passports(passports, |p| p.has_all_mandatory_fields())
}

#[aoc(day4, part2)]
fn solve_part2(passports: &[Passport]) -> usize {
    count_passports(passports, |p| p.is_valid())
}

fn count_passports<F>(passports: &[Passport], predicate: F) -> usize
where
    F: Fn(&&Passport) -> bool,
{
    passports.iter().filter(&predicate).count()
}
