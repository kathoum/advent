use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

struct Blueprint {
    id: i32,
    cost_ore: i32,
    cost_clay: i32,
    cost_obsidian: (i32,i32),
    cost_geode: (i32,i32),
}

fn main() {
    let reader = BufReader::new(File::open("input/day19.txt").unwrap());
    let blueprints = reader.lines().map(|l| l.unwrap().parse().unwrap()).collect::<Vec<Blueprint>>();

    let quality: i32 = blueprints.iter().map(|bp| bp.id * bp.maximum_geodes(24)).sum();
    println!("The total quality level is {quality}");

    let bp1 = Blueprint { id: -1, cost_ore: 4, cost_clay: 2, cost_obsidian: (3,14), cost_geode: (2,7) };
    bp1.maximum_geodes(32);
    let bp2 = Blueprint { id: -2, cost_ore: 2, cost_clay: 3, cost_obsidian: (3,8), cost_geode: (3,12) };
    bp2.maximum_geodes(32);

    let prod1 = blueprints[0].maximum_geodes(32);
    let prod2 = blueprints[1].maximum_geodes(32);
    let prod3 = blueprints[2].maximum_geodes(32);
    println!("The geode production of the first three blueprints is {}", prod1 * prod2 * prod3);
}

#[derive(Default, Clone, Copy)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    skip_ore: bool,
    skip_clay: bool,
    skip_obsidian: bool,
}

enum Robot { Ore, Clay, Obsidian, Geode }

struct Cache(HashMap<[i32;5], Vec<Resources>>);

impl Blueprint {
    pub fn maximum_geodes(&self, time: i32) -> i32 {
        let res = Resources { ore_robots: 1, ..Default::default() };
        let mut cache = Cache(HashMap::new());
        let max = self.find_maximum_geodes(&mut cache, res, time);
        println!("Blueprint {} time {} geode {}", self.id, time, max.geode);
        max.geode
    }

    fn find_maximum_geodes(&self, cache: &mut Cache, res: Resources, time: i32) -> Resources {
        // let key = [time, res.ore_robots, res.clay_robots, res.obsidian_robots, res.geode_robots];
        // match cache.0.get_mut(&key) {
        //     None => { cache.0.insert(key, vec![res]); }
        //     Some(v) => {
        //         if v.iter().any(|r| r.ore >= res.ore && r.clay >= res.clay && r.obsidian >= res.obsidian) {
        //             return res;
        //         } else {
        //             v.push(res);
        //         }
        //     }
        // }

        if time == 0 {
            res
        } else if self.can_produce_geode_robot(&res) {
            self.find_maximum_with_production(cache, res, time, Some(Robot::Geode))
        } else {
            let mut best = self.find_maximum_with_production(cache, res, time, None);
            if !res.skip_obsidian && self.can_produce_obsidian_robot(&res) {
                let t = self.find_maximum_with_production(cache, res, time, Some(Robot::Obsidian));
                if t.geode > best.geode { best = t; }
            }
            if !res.skip_clay && self.can_produce_clay_robot(&res) {
                let t = self.find_maximum_with_production(cache, res, time, Some(Robot::Clay));
                if t.geode > best.geode { best = t; }
            }
            if !res.skip_ore && self.can_produce_ore_robot(&res) {
                let t = self.find_maximum_with_production(cache, res, time, Some(Robot::Ore));
                if t.geode > best.geode { best = t; }
            }
            best
        }
    }

    fn find_maximum_with_production(&self, cache: &mut Cache, mut resources: Resources, time: i32, production: Option<Robot>) -> Resources {
        if production.is_none() {
            if self.can_produce_ore_robot(&resources) { resources.skip_ore = true; }
            if self.can_produce_clay_robot(&resources) { resources.skip_clay = true; }
            if self.can_produce_obsidian_robot(&resources) { resources.skip_obsidian = true; }
        } else {
            resources.skip_ore = false;
            resources.skip_clay = false;
            resources.skip_obsidian = false;
        }
        self.gather(&mut resources);
        self.produce(&mut resources, production);
        self.find_maximum_geodes(cache, resources, time - 1)
    }

    fn gather(&self, resources: &mut Resources) {
        resources.ore += resources.ore_robots;
        resources.clay += resources.clay_robots;
        resources.obsidian += resources.obsidian_robots;
        resources.geode += resources.geode_robots;
    }

    fn produce(&self, resources: &mut Resources, production: Option<Robot>) {
        match production {
            None => (),
            Some(Robot::Ore) => {
                resources.ore -= self.cost_ore;
                resources.ore_robots += 1;
            }
            Some(Robot::Clay) => {
                resources.ore -= self.cost_clay;
                resources.clay_robots += 1;
            }
            Some(Robot::Obsidian) => {
                resources.ore -= self.cost_obsidian.0;
                resources.clay -= self.cost_obsidian.1;
                resources.obsidian_robots += 1;
            }
            Some(Robot::Geode) => {
                resources.ore -= self.cost_geode.0;
                resources.obsidian -= self.cost_geode.1;
                resources.geode_robots += 1;
            }
        }
        assert!(resources.ore >= 0);
        assert!(resources.clay >= 0);
        assert!(resources.obsidian >= 0);
        assert!(resources.geode >= 0);
    }

    fn can_produce_ore_robot(&self, resources: &Resources) -> bool {
        resources.ore >= self.cost_ore && (
            resources.ore_robots < self.cost_ore ||
            resources.ore_robots < self.cost_clay ||
            resources.ore_robots < self.cost_obsidian.0 ||
            resources.ore_robots < self.cost_geode.0
        )
    }

    fn can_produce_clay_robot(&self, resources: &Resources) -> bool {
        resources.ore >= self.cost_clay &&
        resources.clay_robots < self.cost_obsidian.1
    }

    fn can_produce_obsidian_robot(&self, resources: &Resources) -> bool {
        resources.ore >= self.cost_obsidian.0 &&
        resources.clay >= self.cost_obsidian.1 &&
        resources.obsidian_robots < self.cost_geode.1
    }

    fn can_produce_geode_robot(&self, resources: &Resources) -> bool {
        resources.ore >= self.cost_geode.0 &&
        resources.obsidian >= self.cost_geode.1
    }
}

impl std::str::FromStr for Blueprint {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Blueprint ").unwrap();
        let (id, s) = s.split_once(": Each ore robot costs ").unwrap();
        let (ore, s) = s.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay, s) = s.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_ore, s) = s.split_once(" ore and ").unwrap();
        let (obsidian_clay, s) = s.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_ore, s) = s.split_once(" ore and ").unwrap();
        let (geode_obsidian, _) = s.split_once(" obsidian.").unwrap();
        Ok(Blueprint {
            id: id.parse()?,
            cost_ore: ore.parse()?,
            cost_clay: clay.parse()?,
            cost_obsidian: (obsidian_ore.parse()?, obsidian_clay.parse()?),
            cost_geode: (geode_ore.parse()?, geode_obsidian.parse()?),
        })
    }
}
