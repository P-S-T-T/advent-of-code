/*
--- Day 4: Passport Processing ---

You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.

Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:

    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)

Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.

Here is an example batch file containing four passports:
```
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
```
The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid passports.

Count the number of valid passports - those that have all required fields. Treat cid as optional. In your batch file, how many passports are valid?

Your puzzle answer was 237.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!

You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:

byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789

Here are some invalid passports:

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

Here are some valid passports:

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

Count the number of valid passports - those that have all required fields and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid?

*/
use crate::parse_error::ParseError;

use regex::Regex;

#[derive(Debug)]
enum PassportAttributeNotValidated {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}
#[derive(Debug)]
enum PassportAttributeValidated {
    BirthYear(u16),
    IssueYear(u16),
    ExpirationYear(u16),
    Height { value: u16, unit: String },
    HairColor(String),
    EyeColor(String),
    PassportID(u32),
    CountryID(String),
}

impl PassportAttributeNotValidated {
    fn new(attribute: &str) -> Result<PassportAttributeNotValidated, ParseError> {
        let attribute_with_value: Vec<&str> = attribute.split(':').collect();
        match attribute_with_value[0] {
            "byr" => Ok(PassportAttributeNotValidated::BirthYear(String::from(
                attribute_with_value[1],
            ))),
            "iyr" => Ok(PassportAttributeNotValidated::IssueYear(String::from(
                attribute_with_value[1],
            ))),
            "eyr" => Ok(PassportAttributeNotValidated::ExpirationYear(String::from(
                attribute_with_value[1],
            ))),
            "hgt" => Ok(PassportAttributeNotValidated::Height(String::from(
                attribute_with_value[1],
            ))),

            "hcl" => Ok(PassportAttributeNotValidated::HairColor(String::from(
                attribute_with_value[1],
            ))),
            "ecl" => Ok(PassportAttributeNotValidated::EyeColor(String::from(
                attribute_with_value[1],
            ))),
            "pid" => Ok(PassportAttributeNotValidated::PassportID(String::from(
                attribute_with_value[1],
            ))),
            "cid" => Ok(PassportAttributeNotValidated::CountryID(String::from(
                attribute_with_value[1],
            ))),
            _ => Err(ParseError::NoneError),
        }
    }
    fn validate_attributes(
        attributes: &[PassportAttributeNotValidated],
    ) -> Result<Vec<PassportAttributeValidated>, ParseError> {
        attributes
            .iter()
            .map(|attribute| PassportAttributeNotValidated::validate_attribute(attribute))
            .collect()
    }
    fn validate_attribute(
        attribute: &PassportAttributeNotValidated,
    ) -> Result<PassportAttributeValidated, ParseError> {
        match attribute {
            PassportAttributeNotValidated::BirthYear(s) => {
                let value = s.parse::<u16>();
                match value {
                    Ok(n) => match n {
                        1920..=2002 => Ok(PassportAttributeValidated::BirthYear(n)),
                        _ => Err(ParseError::ValidationError),
                    },
                    Err(err) => Err(ParseError::ParseIntError(err)),
                }
            }
            PassportAttributeNotValidated::IssueYear(s) => {
                let value = s.parse::<u16>();
                match value {
                    Ok(n) => match n {
                        2010..=2020 => Ok(PassportAttributeValidated::IssueYear(n)),
                        _ => Err(ParseError::ValidationError),
                    },
                    Err(err) => Err(ParseError::ParseIntError(err)),
                }
            }
            PassportAttributeNotValidated::ExpirationYear(s) => {
                let value = s.parse::<u16>();
                match value {
                    Ok(n) => match n {
                        2020..=2030 => Ok(PassportAttributeValidated::ExpirationYear(n)),
                        _ => Err(ParseError::ValidationError),
                    },
                    Err(err) => Err(ParseError::ParseIntError(err)),
                }
            }
            PassportAttributeNotValidated::Height(s) => {
                if s.len() < 3 {
                    return Err(ParseError::ValidationError);
                }
                let (height_value_string, height_unit) = s.split_at(s.len() - 2);
                let height_value_result = height_value_string.parse::<u16>();
                match height_value_result {
                    Ok(height_value) => match height_unit {
                        "cm" => match height_value {
                            150..=193 => Ok(PassportAttributeValidated::Height {
                                value: height_value,
                                unit: height_unit.to_owned(),
                            }),
                            _ => Err(ParseError::ValidationError),
                        },
                        "in" => match height_value {
                            59..=76 => Ok(PassportAttributeValidated::Height {
                                value: height_value,
                                unit: height_unit.to_owned(),
                            }),
                            _ => Err(ParseError::ValidationError),
                        },
                        _ => Err(ParseError::ValidationError),
                    },
                    Err(err) => Err(ParseError::ParseIntError(err)),
                }
            }

            PassportAttributeNotValidated::HairColor(color) => {
                let regex = Regex::new(r"#[0-9a-f]{6}").unwrap();

                match regex.is_match(color) {
                    true => Ok(PassportAttributeValidated::HairColor(color.to_owned())),
                    false => Err(ParseError::ValidationError),
                }
            }
            PassportAttributeNotValidated::EyeColor(color) => match color.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                    Ok(PassportAttributeValidated::EyeColor(color.to_owned()))
                }
                _ => Err(ParseError::ValidationError),
            },
            PassportAttributeNotValidated::PassportID(s) => {
                let value = s.parse::<u32>();
                match value {
                    Ok(n) => match n {
                        0..=999_999_999 => Ok(PassportAttributeValidated::PassportID(n)),
                        _ => Err(ParseError::ValidationError),
                    },
                    Err(err) => Err(ParseError::ParseIntError(err)),
                }
            }
            PassportAttributeNotValidated::CountryID(s) => {
                Ok(PassportAttributeValidated::CountryID(s.to_owned()))
            }
        }
    }
}

#[derive(Debug)]
enum Passport {
    NotValidated(Vec<PassportAttributeNotValidated>),
    Valid(Vec<PassportAttributeValidated>),
    // Invalid(&'a [Result<PassportAttributeValidated<'a>, ParseError>]),
    Invalid,
}
impl Passport {
    fn new(attributes: Vec<Result<PassportAttributeNotValidated, ParseError>>) -> Passport {
        let attributes = attributes
            .into_iter()
            .collect::<Result<Vec<PassportAttributeNotValidated>, ParseError>>();

        match attributes {
            Err(_) => Passport::Invalid,
            Ok(attr) => match Passport::attribute_types_and_count_are_valid(&attr) {
                true => Passport::NotValidated(attr),
                false => Passport::Invalid,
            },
        }
    }
    fn from_not_validated(passport: &Passport) -> Result<Passport, ParseError> {
        match passport {
            Passport::NotValidated(attributes) => {
                let validated_attributes =
                    PassportAttributeNotValidated::validate_attributes(attributes);
                match validated_attributes {
                    Ok(attr) => Ok(Passport::Valid(attr)),
                    Err(_) => Err(ParseError::ValidationError),
                }
            }
            _ => Err(ParseError::ValidationError),
        }
    }

    fn attribute_types_and_count_are_valid(attributes: &[PassportAttributeNotValidated]) -> bool {
        match attributes.len() {
            7 => Passport::contains_all_seven_attributes(&attributes),
            8 => {
                attributes
                    .iter()
                    .any(|a| matches!(a, PassportAttributeNotValidated::CountryID(_)))
                    && Passport::contains_all_seven_attributes(&attributes)
            }
            _ => false,
        }
    }

    fn contains_all_seven_attributes(attributes: &[PassportAttributeNotValidated]) -> bool {
        attributes
            .iter()
            .any(|a| matches!(a, PassportAttributeNotValidated::BirthYear(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttributeNotValidated::IssueYear(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttributeNotValidated::ExpirationYear(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttributeNotValidated::Height(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttributeNotValidated::HairColor(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttributeNotValidated::EyeColor(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttributeNotValidated::PassportID(_)))
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    let re = Regex::new(r"\n\s*\n").unwrap();
    let passport_entries = re.split(input);
    let result = passport_entries
        .map(|entry| {
            println!("entry {:#?}", entry);
            let attributes = entry
                .split_whitespace()
                .map(|a| PassportAttributeNotValidated::new(a))
                .collect();
            let pass = Passport::new(attributes);
            println!("Pass {:#?}", pass);
            pass
        })
        .collect::<Vec<Passport>>();
    println!("Passports: {:#?}", result);
    result
}

#[aoc(day4, part1)]
fn part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|pass| !matches!(pass, Passport::Invalid))
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|pass| !matches!(pass, Passport::Invalid))
        .map(|p| Passport::from_not_validated(p))
        .filter(|pass| matches!(pass, Ok(Passport::Valid(_))))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input_part1() -> &'static str {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:1
    
    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:18

    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm
    
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929
    
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm
    
    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in"
    }
    fn parse_input_test_part1() -> Vec<Passport> {
        parse_input(sample_input_part1())
    }
    #[test]
    fn test_part1() {
        assert_eq!(4, part1(&parse_input_test_part1()))
    }

    fn sample_input_part2_not_valid() -> &'static str {
        "eyr:1972 cid:100
      hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
      
      iyr:2019
      hcl:#602927 eyr:1967 hgt:170cm
      ecl:grn pid:012533040 byr:1946
      
      hcl:dab227 iyr:2012
      ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
      
      hgt:59cm ecl:zzz
      eyr:2038 hcl:74454a iyr:2023
      pid:3556412378 byr:2007"
    }
    fn sample_input_part2_valid() -> &'static str {
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f
    
    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
    
    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022
    
    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
    }

    fn parse_input_test_part2_not_valid() -> Vec<Passport> {
        parse_input(sample_input_part2_not_valid())
    }
    fn parse_input_test_part2_valid() -> Vec<Passport> {
        parse_input(sample_input_part2_valid())
    }
    #[test]
    fn test_part2_not_valid_input() {
        assert_eq!(0, part2(&parse_input_test_part2_not_valid()))
    }
    #[test]
    fn test_part2_valid_input() {
        assert_eq!(4, part2(&parse_input_test_part2_valid()))
    }
}
