struct RangeCheck {
    destination_start: usize,
    source_start: usize,
    length: usize
}

struct MultiRangeChecker {
    ranges: Vec<RangeCheck>
}

impl From<&str> for RangeCheck {
    fn from(value: &str) -> Self {
        let parts = value.split(" ")
            .map(|part| part.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            destination_start: parts[0],
            source_start: parts[1],
            length: parts[2]
        }
    }
}

impl RangeCheck {
    fn is_contained(&self, s: usize) -> bool {
        s >= self.source_start && s <= (self.source_start + self.length)
    }

    fn destination_from_source(&self, s: usize) -> usize {
        self.destination_start + (s - self.source_start)
    }
}

impl From<&&str> for MultiRangeChecker {
    fn from(value: &&str) -> Self {
        let ranges = value.split(":")
            .last()
            .unwrap()
            .split("\n")
            .filter(|val| *val != "")
            .map(RangeCheck::from)
            .collect();
        Self { ranges }
    }
}

impl MultiRangeChecker {
    fn resolve(&self, src: usize) -> usize {
        for range in self.ranges.iter() {
            if range.is_contained(src) {
                return range.destination_from_source(src);
            }
        }
        src
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt")
        .expect("Cannot read file!");
    let result = content.split("\n\n")
        .collect::<Vec<_>>();

    let seeds = result.get(0)
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter(|val| *val != "")
        .map(|ns| ns.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let seed_to_soil = MultiRangeChecker::from(result.get(1).unwrap());
    let soil_to_fert = MultiRangeChecker::from(result.get(2).unwrap());
    let fert_to_watr = MultiRangeChecker::from(result.get(3).unwrap());
    let watr_to_ligh = MultiRangeChecker::from(result.get(4).unwrap());
    let ligh_to_temp = MultiRangeChecker::from(result.get(5).unwrap());
    let temp_to_humi = MultiRangeChecker::from(result.get(6).unwrap());
    let humi_to_loca = MultiRangeChecker::from(result.get(7).unwrap());

    let min_location = seeds.iter()
        .map(|seed| {
            let soil = seed_to_soil.resolve(*seed);
            let fert = soil_to_fert.resolve(soil);
            let watr = fert_to_watr.resolve(fert);
            let ligh = watr_to_ligh.resolve(watr);
            let temp = ligh_to_temp.resolve(ligh);
            let humi = temp_to_humi.resolve(temp);
            humi_to_loca.resolve(humi)
        })
        .min()
        .unwrap();

    println!("ANS: {min_location:?}");
}
