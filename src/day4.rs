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

*/
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum PassportAttribute {
    BirthYear(u16),
    IssueYear(u16),
    ExpirationYear(u16),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}
impl PassportAttribute {
    fn from_str(attribute: &str) -> Result<PassportAttribute, ParseIntError> {
        let attribute_with_value: Vec<&str> = attribute.split(':').collect();
        match attribute_with_value {
            byr if byr[0] == "byr" => {
                let value = (*byr[1]).parse::<u16>();
                match value {
                    Ok(n) => Ok(PassportAttribute::BirthYear(n)),
                    Err(err) => Err(err),
                }
            }
            iyr if iyr[0] == "iyr" => {
                let value = (*iyr[1]).parse::<u16>();
                match value {
                    Ok(n) => Ok(PassportAttribute::IssueYear(n)),
                    Err(err) => Err(err),
                }
            }
            eyr if eyr[0] == "eyr" => {
                let value = (*eyr[1]).parse::<u16>();
                match value {
                    Ok(n) => Ok(PassportAttribute::ExpirationYear(n)),
                    Err(err) => Err(err),
                }
            }
            hgt if hgt[0] == "hgt" => Ok(PassportAttribute::Height(String::from(hgt[1]))),
            hcl if hcl[0] == "hcl" => Ok(PassportAttribute::HairColor(String::from(hcl[1]))),
            ecl if ecl[0] == "ecl" => Ok(PassportAttribute::EyeColor(String::from(ecl[1]))),
            pid if pid[0] == "pid" => Ok(PassportAttribute::PassportID(String::from(pid[1]))),
            cid if cid[0] == "cid" => Ok(PassportAttribute::CountryID(String::from(cid[1]))),
            _ => Err("create ParseIntError".parse::<u16>().unwrap_err()),
        }
    }
}

#[derive(Debug)]
enum Passport {
    Valid(Vec<PassportAttribute>),
    // Invalid(&'a [Result<PassportAttribute<'a>, ParseIntError>]),
    Invalid,
}
impl Passport {
    fn from_attributes(attributes: Vec<Result<PassportAttribute, ParseIntError>>) -> Passport {
        let attributes = attributes
            .into_iter()
            .collect::<Result<Vec<PassportAttribute>, ParseIntError>>();

        match attributes {
            Err(_) => Passport::Invalid,
            Ok(attr) if Passport::attributes_are_valid(&attr) => Passport::Valid(attr),
            _ => Passport::Invalid,
        }
    }

    fn attributes_are_valid(attributes: &[PassportAttribute]) -> bool {
        match attributes.len() {
            7 => Passport::contains_seven_attributes(&attributes),
            8 => {
                attributes
                    .iter()
                    .any(|a| matches!(a, PassportAttribute::CountryID(_)))
                    && Passport::contains_seven_attributes(&attributes)
            }
            _ => false,
        }
    }

    fn contains_seven_attributes(attributes: &[PassportAttribute]) -> bool {
        attributes
            .iter()
            .any(|a| matches!(a, PassportAttribute::BirthYear(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttribute::IssueYear(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttribute::ExpirationYear(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttribute::Height(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttribute::HairColor(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttribute::EyeColor(_)))
            && attributes
                .iter()
                .any(|a| matches!(a, PassportAttribute::PassportID(_)))
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    let passport_entries = input.split("\n    \n");
    println!("Split entries {:#?}", passport_entries);
    let result = passport_entries
        .map(|entry| {
            println!("entry {:#?}", entry);
            let attributes = entry
                .split_whitespace()
                .map(|a| PassportAttribute::from_str(a))
                .collect();
            let pass = Passport::from_attributes(attributes);
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
        .filter(|pass| matches!(pass, Passport::Valid(_)))
        .count()
}

// #[aoc(day4, part2)]
// fn part2(forest_map: &[String]) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
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
    fn parse_input_test() -> Vec<Passport> {
        parse_input(sample_input())
    }
    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&parse_input_test()))
        // assert_eq!(2, part1(sample_input()))
    }
    // #[test]
    // fn test_ride_part2() {
    //   assert_eq!(336, part2(&parse_input()))
    // }
}
