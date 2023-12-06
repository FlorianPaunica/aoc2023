use std::time::Instant;

use crate::util;

pub fn main() {
    let data = util::read_file("five");
    let start = Instant::now();
    let one_result = part_one(&data);
    let duration = start.elapsed();
    println!("Input one result: {}", one_result);
    println!("Execution time: {:?}", duration);
    let start = Instant::now();
    let two_result = part_two(&data);
    let duration = start.elapsed();
    println!("Input two result: {}", two_result);
    println!("Execution time: {:?}", duration);
}

#[derive(Debug, Clone)]
struct ScDest {
    min_dest: usize,
    min_source: usize,
    range: usize,
}

type FarmMap = Vec<ScDest>;
trait ConvertMap {
    fn convert(&self, v: Vec<(usize, usize)>) -> Vec<(usize, usize)>;
}

impl ConvertMap for FarmMap {
    fn convert(&self, mut v: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];
    while let Some(s) = v.pop() {
        let (r, skipped) = transform_seeds(&self, s);
        res.push(r);
        if skipped.len() > 0 {
            skipped.iter().for_each(|s| {
                let (r_s, _) = transform_seeds(&self, *s);
                res.push(r_s);
            });
        }

    }
    res
    }
}

fn transform_seeds(sd_vec: &Vec<ScDest>, s: (usize, usize)) -> ((usize, usize), Vec<(usize, usize)>) {
        let mut res: (usize, usize) = s;
        let mut skipped: Vec<(usize, usize)> = vec![];

        for sd in sd_vec.iter() {
            if s.0 + s.1 <= sd.min_source || s.0 >= sd.min_source + sd.range {
                continue;
            } else if s.0 >= sd.min_source && s.0 + s.1 <= sd.min_source + sd.range {
                res = (s.0 - sd.min_source + sd.min_dest, s.1);
            } else if s.0 > sd.min_source && s.0 + s.1 > sd.min_source + sd.range {
                let to_transform = sd.min_source + sd.range - s.0;
                let skip = (s.0, s.1 - to_transform);
                let transf = (s.0 + sd.min_dest - sd.min_source, to_transform);
                res = transf;
                skipped.push(skip);
            } else if s.0 < sd.min_source && s.0 + s.1 >= sd.min_source {
                let to_keep = sd.min_source - s.0;
                let skip = (s.0, to_keep);
                let transf = (s.0 + to_keep - sd.min_source + sd.min_dest, s.1 - to_keep);
                skipped.push(skip);
                res = transf;
            } else if s.0 < sd.min_source && s.0 + s.1 >= sd.min_source + sd.range {
                let keep_l = sd.min_source - s.0;
                let keep_r = s.0 + s.1 - sd.min_source + sd.range;
                let skip_l = (s.0, keep_l);
                let skip_r = (sd.min_source + sd.range, keep_r);
                let transf = (s.0 + keep_l - sd.min_source + sd.min_dest, s.1 - keep_r);
                skipped.push(skip_l);
                skipped.push(skip_r);
                res = transf;
            }
        }
        (res, skipped)
}

#[derive(Debug)]
struct Farm {
    seeds: Vec<(usize, usize)>,
    sts: FarmMap,
    stf: FarmMap,
    ftw: FarmMap,
    wtl: FarmMap,
    ltt: FarmMap,
    tth: FarmMap,
    htl: FarmMap,
}

impl Farm {
    fn seed_to_loc(&mut self) -> Vec<(usize, usize)> {
        let soil = self.sts.convert(self.seeds.clone());
        let fert = self.stf.convert(soil);
        let water = self.ftw.convert(fert);
        let light = self.wtl.convert(water);
        let temp = self.ltt.convert(light);
        let hum = self.tth.convert(temp);
        let loc = self.htl.convert(hum);
        return loc;
    }
    fn spread_seeds(&mut self) {
        self.seeds = self.seeds.chunks(2).map(|sc| (sc[0].0, sc[1].0)).collect();
    }
}

fn part_one(data: &Vec<String>) -> usize {
    let mut farm = parse_data(data);
    let res = farm.seed_to_loc().iter().map(|x| x.0).min().unwrap();
    res
}

fn part_two(data: &Vec<String>) -> usize {
    let mut farm = parse_data(data);
    farm.spread_seeds();
    farm.seed_to_loc().iter().map(|x| x.0).min().unwrap()
}

fn parse_data(data: &Vec<String>) -> Farm {
    let seed_line = 0;
    let seed_to_soil_line = data
        .iter()
        .position(|l| l.trim() == String::from("seed-to-soil map:"))
        .expect("Could not parse seed-to-soil line");
    let soil_to_fertilizer_line = data
        .iter()
        .position(|l| l.trim() == String::from("soil-to-fertilizer map:"))
        .expect("Could not parse soil-to-fertilizer line");
    let fertilizer_to_water_line = data
        .iter()
        .position(|l| l.trim() == String::from("fertilizer-to-water map:"))
        .expect("Could not parse fertilizer-to-water line");
    let water_to_light_line = data
        .iter()
        .position(|l| l.trim() == String::from("water-to-light map:"))
        .expect("Could not parse water-to-light line");
    let light_to_temp_line = data
        .iter()
        .position(|l| l.trim() == String::from("light-to-temperature map:"))
        .expect("Could not parse light-to-temperature line");
    let temp_to_humid_line = data
        .iter()
        .position(|l| l.trim() == String::from("temperature-to-humidity map:"))
        .expect("Could not parse temperature-to-humidity line");
    let humid_to_location_line = data
        .iter()
        .position(|l| l.trim() == String::from("humidity-to-location map:"))
        .expect("Could not parse humidity-to-location line");

    let (_, seeds_str) = data[seed_line].split_once(":").unwrap();
    let seeds: Vec<(usize, usize)> = seeds_str
        .trim()
        .split(" ")
        .map(|s| (s.trim().parse::<usize>().unwrap(), 1))
        .collect();
    Farm {
        seeds,
        sts: extract_maps(&data[seed_to_soil_line + 1..soil_to_fertilizer_line - 1].to_vec()),
        stf: extract_maps(
            &data[soil_to_fertilizer_line + 1..fertilizer_to_water_line - 1].to_vec(),
        ),
        ftw: extract_maps(&data[fertilizer_to_water_line + 1..water_to_light_line - 1].to_vec()),
        wtl: extract_maps(&data[water_to_light_line + 1..light_to_temp_line - 1].to_vec()),
        ltt: extract_maps(&data[light_to_temp_line + 1..temp_to_humid_line - 1].to_vec()),
        tth: extract_maps(&data[temp_to_humid_line + 1..humid_to_location_line - 1].to_vec()),
        htl: extract_maps(&data[humid_to_location_line + 1..].to_vec()),
    }
}

fn extract_maps(lines: &Vec<String>) -> FarmMap {
    let mut fm: FarmMap = lines
        .iter()
        .map(|l| {
            let numbers = l
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            return ScDest {
                min_dest: numbers[0],
                min_source: numbers[1],
                range: numbers[2],
            };
        })
        .collect();
    fm.sort_by(|a, b| a.min_source.cmp(&b.min_source));
    return fm;
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_one(&input);
        assert_eq!(result, 35);
    }
    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 46);
    }
}
