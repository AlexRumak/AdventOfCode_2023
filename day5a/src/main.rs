use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read input.txt");

    // Initialization
    let (seeds, almanac) = parse_input(&contents);

    let lowest_location = find_lowest_location(seeds, almanac);

    println!("Lowest location: {}", lowest_location);
}

fn find_lowest_location(seeds: Seeds, almanac: Almanac) -> u32 {

    let mut lowest_location = u32::MAX;
    for seed in seeds.seeds {
        let soil_value = almanac.seed_to_soil.convert_value(seed);
        let fertilizer_value = almanac.soil_to_fertilizer.convert_value(soil_value);
        let water_value = almanac.fertilizer_to_water.convert_value(fertilizer_value);
        let light_value = almanac.water_to_light.convert_value(water_value);
        let temperature_value = almanac.light_to_temperature.convert_value(light_value);
        let humidity_value = almanac.temperature_to_humidity.convert_value(temperature_value);
        let location_value = almanac.humidity_to_location.convert_value(humidity_value);

        if (location_value as u32) < lowest_location {
            lowest_location = location_value as u32;
        }
    }

    return lowest_location;
}

struct Seeds {
    seeds: Vec<i64>,
}

struct Range {
    min: i64,
    max: i64,
    conversion: i64
}

struct Mappings {
    mappings: Vec<Range>
}

impl Mappings {
    fn convert_value(&self, input: i64) -> i64 {
        for range in &self.mappings {
            if range.min <= input && input <= range.max {
                return input - range.conversion;
            }
        }

        // If there is no mapping, return the value
        return input
    }
}

struct Almanac {
    seed_to_soil: Mappings,
    soil_to_fertilizer: Mappings,
    fertilizer_to_water: Mappings,
    water_to_light: Mappings,
    light_to_temperature: Mappings,
    temperature_to_humidity: Mappings,
    humidity_to_location: Mappings,
}

fn parse_input(input: &str) -> (Seeds, Almanac) {
    let mut iter = input.split("\n\n");
    let seeds = parse_seeds(iter.next().expect("Should be able to get seeds"));
    let almanac = parse_almanac(iter);

    (seeds, almanac)
}

fn parse_seeds(input: &str) -> Seeds {
    let mut seeds = Vec::new();
    let mut iter = input.split(' ');
    iter.next(); // skip annotation of 'seeds'

    for seed in iter {
        seeds.push(seed.parse::<i64>().expect("Should have been able to parse seed number"));
    }

    Seeds { seeds: seeds }
}

fn parse_almanac<'a, T>(mut iter: T) -> Almanac
where 
    T: Iterator<Item = &'a str>
{
    Almanac {
        seed_to_soil: parse_mappings(iter.next()),
        soil_to_fertilizer: parse_mappings(iter.next()),
        fertilizer_to_water: parse_mappings(iter.next()),
        water_to_light: parse_mappings(iter.next()),
        light_to_temperature: parse_mappings(iter.next()),
        temperature_to_humidity: parse_mappings(iter.next()),
        humidity_to_location: parse_mappings(iter.next()),
    }
}

fn parse_mappings(map: Option<&str>) -> Mappings
{
    let mut mappings : Vec<Range> = Vec::new();

    let map_str = map.expect("Should have been able to get map");
    let mut iter = map_str.split('\n');
    iter.next(); // skip map name
    for line in iter {
        let mut range_info = line.split(' ');

        let source_range_start = parse_int_or_panic(range_info.next());
        let destination_range_start = parse_int_or_panic(range_info.next());
        let range = parse_int_or_panic(range_info.next());

        mappings.push(
            Range { 
                min: destination_range_start,
                max: destination_range_start + range - 1,
                conversion: destination_range_start - source_range_start,
            }
        )
    }

    Mappings {
        mappings: mappings
    }
}

fn parse_int_or_panic(int : Option<&str>) -> i64 {
    int
        .expect("Should have been able to get value")
        .parse::<i64>()
        .expect("Value should be a valid i64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_almanac() {

    }

    #[test]
    fn test_parse_seeds() {
        let input = String::from("seeds: 1 2 3 4 5 6 7 8 9 10");
        let seeds = parse_seeds(&input);
        assert_eq!(seeds.seeds, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_parse_mappings() {
        let input = Some("seed-to-soil map:\n0 2 2");
        let parse_mappings = parse_mappings(input);

        let get = parse_mappings.mappings.get(0).expect("Test - come on");
        assert_eq!(get.min, 2);
        assert_eq!(get.max, 3);
        assert_eq!(get.conversion, 2);
    }
}