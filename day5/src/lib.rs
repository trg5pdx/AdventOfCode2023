use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum AlmanacState {
    SeedSoil,
    SoilFert,
    FertWater,
    WaterLight,
    LightTemp,
    TempHum,
    HumLoc,
    Parse // Base state
}

fn update_ranges(
    map: &mut HashMap<AlmanacState, Vec<Vec<u32>>>, 
    key: &AlmanacState, 
    value: &str) {
    if let Some(x) = map.get_mut(&key) {
        x.push(fetch_numbers(value));
    } else {
        map.insert(*key, vec![fetch_numbers(value)]);
    }
}

fn fetch_numbers(input: &str) -> Vec<u32> {
    input.split(' ').filter_map(|s| s.parse().ok()).collect()
}

pub fn get_lowest_seed_location(input: &String) -> u32 {
    let mut lowest: Option<u32> = None;   
    let mut seeds = Vec::new(); 
    let mut state = AlmanacState::Parse; 
    let mut ranges: HashMap<AlmanacState, Vec<Vec<u32>>> = HashMap::new();

    let lines = input.split('\n').map(|s| s.trim());
    
    for l in lines {
        println!("line: {:?}", l);
        
        if l.starts_with(|c: char| !c.is_digit(10)) {
            state = AlmanacState::Parse;
        }

        if state == AlmanacState::Parse {
            if l.contains("seeds") {
                seeds = fetch_numbers(l); 
                println!("{:?}", seeds);
            }
            if l.contains("seed-to-soil") {
                state = AlmanacState::SeedSoil;
            } else if l.contains("soil-to-fertilizer") {
                state = AlmanacState::SoilFert;
            } else if l.contains("fertilizer-to-water") {
                state = AlmanacState::FertWater;
            } else if l.contains("water-to-light") {
                state = AlmanacState::WaterLight;
            } else if l.contains("light-to-temperature") {
                state = AlmanacState::LightTemp;
            } else if l.contains("temperature-to-humidity") {
                state = AlmanacState::TempHum;
            } else if l.contains("humidity-to-location") {
                state = AlmanacState::HumLoc;
            }
        } else {
            update_ranges(&mut ranges, &state, l);
        }
    }

     
    for i in 0..seeds.len() {
        let conversions = [AlmanacState::SeedSoil, 
            AlmanacState::SoilFert, 
            AlmanacState::FertWater, 
            AlmanacState::WaterLight, 
            AlmanacState::LightTemp, 
            AlmanacState::TempHum, 
            AlmanacState::HumLoc];
        println!("before: {}", seeds[i]);
        for c in conversions {
            let mut found = false;
            if let Some(range) = ranges.get(&c) {
                for r in range {
                    if !r.is_empty() && !found {
                        let dest = r[0];
                        let source = r[1];
                        let size = r[2];
                        
                        if source <= seeds[i] && seeds[i] <= source + size {
                            seeds[i] = dest + (seeds[i] - source);
                            found = true;
                            println!("{:?} {}", c, seeds[i]);
                        }
                    }
                }
            }
        }
        println!("after: {}\n", seeds[i]);
    }

    println!("{:?}", seeds);
    for (k, v) in ranges.iter() {
        println!("K: {:?}, V: {:?}", k, v);
    }

    for i in seeds {
        if let Some(j) = lowest {
            if j > i {
                lowest = Some(i);
            }
        } else {
            lowest = Some(i);
        }
    }

    
    match lowest {
        None => 0,
        Some(n) => n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_almanac_check() {
        let input = 
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4"
        .to_string();

        let result = get_lowest_seed_location(&input);
        
        assert_eq!(result, 35);

    }
}
