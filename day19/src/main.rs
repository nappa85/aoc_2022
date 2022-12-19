use std::{fmt::Display, ops::AddAssign};

#[derive(Copy, Clone, Debug)]
enum Material {
    Ore(u8),
    Clay(u8),
    Obsidian(u8),
}

impl<'a> From<(&'a str, u8)> for Material {
    fn from(value: (&'a str, u8)) -> Self {
        match value.0 {
            "ore" => Material::Ore(value.1),
            "clay" => Material::Clay(value.1),
            "obsidian" => Material::Obsidian(value.1),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_bot: Vec<Material>,
    clay_bot: Vec<Material>,
    obsidian_bot: Vec<Material>,
    geode_bot: Vec<Material>,
}

impl Blueprint {
    fn get_cost(&self, bot: Bot, material: Material) -> u8 {
        match bot {
            Bot::Ore => self
                .ore_bot
                .iter()
                .filter_map(|m| match (m, material) {
                    (Material::Ore(i), Material::Ore(_)) => Some(*i),
                    (Material::Clay(i), Material::Clay(_)) => Some(*i),
                    (Material::Obsidian(i), Material::Obsidian(_)) => Some(*i),
                    _ => None,
                })
                .next()
                .unwrap(),
            Bot::Clay => self
                .clay_bot
                .iter()
                .filter_map(|m| match (m, material) {
                    (Material::Ore(i), Material::Ore(_)) => Some(*i),
                    (Material::Clay(i), Material::Clay(_)) => Some(*i),
                    (Material::Obsidian(i), Material::Obsidian(_)) => Some(*i),
                    _ => None,
                })
                .next()
                .unwrap(),
            Bot::Obsidian => self
                .obsidian_bot
                .iter()
                .filter_map(|m| match (m, material) {
                    (Material::Ore(i), Material::Ore(_)) => Some(*i),
                    (Material::Clay(i), Material::Clay(_)) => Some(*i),
                    (Material::Obsidian(i), Material::Obsidian(_)) => Some(*i),
                    _ => None,
                })
                .next()
                .unwrap(),
            Bot::Geode => self
                .geode_bot
                .iter()
                .filter_map(|m| match (m, material) {
                    (Material::Ore(i), Material::Ore(_)) => Some(*i),
                    (Material::Clay(i), Material::Clay(_)) => Some(*i),
                    (Material::Obsidian(i), Material::Obsidian(_)) => Some(*i),
                    _ => None,
                })
                .next()
                .unwrap(),
        }
    }
}

enum Bot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Display for Bot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bot::Ore => write!(f, "ore-collecting robot"),
            Bot::Clay => write!(f, "clay-collecting robot"),
            Bot::Obsidian => write!(f, "obsidian-collecting robot"),
            Bot::Geode => write!(f, "geode-cracking robot"),
        }
    }
}

#[derive(Debug, Default)]
struct Bots {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Bots {
    fn produce(&self) -> Materials {
        let mut materials = Materials::default();
        if self.ore > 0 {
            materials.ore += self.ore;
            println!(
                "{} ore-collecting robot collects {} ore; you now have {} ore.",
                self.ore, self.ore, materials.ore
            )
        }
        if self.clay > 0 {
            materials.clay += self.clay;
            println!(
                "{} clay-collecting robot collects {} clay; you now have {} clay.",
                self.clay, self.clay, materials.clay
            )
        }
        if self.obsidian > 0 {
            materials.obsidian += self.obsidian;
            println!(
                "{} obsidian-collecting robot collects {} obsidian; you now have {} obsidian.",
                self.obsidian, self.obsidian, materials.obsidian
            )
        }
        if self.geode > 0 {
            materials.geode += self.geode;
            println!(
                "{} geode-cracking robot crack {} geode; you now have {} geode.",
                self.geode, self.geode, materials.geode
            )
        }
        materials
    }
}

#[derive(Debug, Default)]
struct Materials {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Materials {
    fn build(&mut self, bot: Bot, materials: &[Material]) -> bool {
        for material in materials {
            match material {
                Material::Ore(n) if self.ore < *n => return false,
                Material::Clay(n) if self.clay < *n => return false,
                Material::Obsidian(n) if self.obsidian < *n => return false,
                _ => {}
            }
        }

        print!("Spend");
        let mut first = true;
        for material in materials {
            if first {
                first = false;
            } else {
                print!(" and")
            }
            match material {
                Material::Ore(n) => {
                    print!(" {n} ore");
                    self.ore -= *n;
                }
                Material::Clay(n) => {
                    print!(" {n} clay");
                    self.clay -= *n;
                }
                Material::Obsidian(n) => {
                    print!(" {n} obsidian");
                    self.obsidian -= *n;
                }
            }
        }
        println!(" to start building a {}.", bot);

        true
    }
}

impl AddAssign for Materials {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

fn get_materials<'a, I: Iterator<Item = &'a str>>(iter: I) -> Vec<Material> {
    // Each obsidian robot costs 3 ore and 14 clay
    let mut iter = iter.skip(5);
    let mut res = vec![];
    loop {
        let n = iter.next().unwrap().parse().unwrap();
        res.push(Material::from((iter.next().unwrap(), n)));
        if iter.next().is_none() {
            break;
        }
    }
    res
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let blueprints = input
        .lines()
        .map(|s| {
            let mut temp = s.split(':');
            let t1 = temp.next().unwrap();
            let mut t2 = t1.split(' ');
            let id = t2.nth(1).unwrap().parse().unwrap();
            let t1 = temp.next().unwrap();
            let mut t2 = t1.split('.');

            Blueprint {
                id,
                ore_bot: get_materials({
                    let t = t2.next().unwrap();
                    t.split(' ')
                }),
                clay_bot: get_materials({
                    let t = t2.next().unwrap();
                    t.split(' ')
                }),
                obsidian_bot: get_materials({
                    let t = t2.next().unwrap();
                    t.split(' ')
                }),
                geode_bot: get_materials({
                    let t = t2.next().unwrap();
                    t.split(' ')
                }),
            }
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for blueprint in blueprints {
        let mut materials = Materials::default();
        let mut bots = Bots::default();
        bots.ore += 1;
        for i in 0..24 {
            println!("== Minute {} == ", i + 1);
            let production = bots.produce();
            if materials.build(Bot::Geode, &blueprint.geode_bot) {
                bots.geode += 1;
                println!(
                    "The new geode-cracking robot is ready; you now have {} of them.",
                    bots.geode
                );
            } else if blueprint.get_cost(Bot::Geode, Material::Obsidian(0)) > materials.obsidian
                && materials.build(Bot::Obsidian, &blueprint.obsidian_bot)
            {
                bots.obsidian += 1;
                println!(
                    "The new obsidian-collecting robot is ready; you now have {} of them.",
                    bots.obsidian
                );
            } else if blueprint.get_cost(Bot::Obsidian, Material::Clay(0)) > materials.clay
                && materials.build(Bot::Clay, &blueprint.clay_bot)
            {
                bots.clay += 1;
                println!(
                    "The new clay-collecting robot is ready; you now have {} of them.",
                    bots.clay
                );
            } else if blueprint.get_cost(Bot::Clay, Material::Ore(0)) > materials.ore
                && blueprint.get_cost(Bot::Obsidian, Material::Ore(0)) > materials.ore
                && blueprint.get_cost(Bot::Geode, Material::Ore(0)) > materials.ore
                && materials.build(Bot::Ore, &blueprint.ore_bot)
            {
                bots.ore += 1;
                println!(
                    "The new ore-collecting robot is ready; you now have {} of them.",
                    bots.ore
                );
            }
            materials += production;
            println!("");
        }
        sum += materials.geode as u16 * blueprint.id as u16;
    }
    println!("{sum}");
}
