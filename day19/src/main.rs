use std::{fmt::Display, ops::Add};

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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
struct Bots {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Default for Bots {
    fn default() -> Self {
        Bots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

impl Bots {
    fn produce(&self, _materials: &Materials) -> Materials {
        let mut production = Materials::default();
        if self.ore > 0 {
            production.ore += self.ore;
            #[cfg(debug_assertions)]
            println!(
                "{} ore-collecting robot collects {} ore; you now have {} ore.",
                self.ore,
                self.ore,
                _materials.ore + production.ore
            )
        }
        if self.clay > 0 {
            production.clay += self.clay;
            #[cfg(debug_assertions)]
            println!(
                "{} clay-collecting robot collects {} clay; you now have {} clay.",
                self.clay,
                self.clay,
                _materials.clay + production.clay
            )
        }
        if self.obsidian > 0 {
            production.obsidian += self.obsidian;
            #[cfg(debug_assertions)]
            println!(
                "{} obsidian-collecting robot collects {} obsidian; you now have {} obsidian.",
                self.obsidian,
                self.obsidian,
                _materials.obsidian + production.obsidian
            )
        }
        if self.geode > 0 {
            production.geode += self.geode;
            #[cfg(debug_assertions)]
            println!(
                "{} geode-cracking robot crack {} geode; you now have {} geode.",
                self.geode,
                self.geode,
                _materials.geode + production.geode
            )
        }
        production
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Materials {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Materials {
    fn build(&mut self, _bot: Bot, materials: &[Material]) -> bool {
        for material in materials {
            match material {
                Material::Ore(n) if self.ore < *n => return false,
                Material::Clay(n) if self.clay < *n => return false,
                Material::Obsidian(n) if self.obsidian < *n => return false,
                _ => {}
            }
        }

        #[cfg(debug_assertions)]
        print!("Spend");
        let mut first = true;
        for material in materials {
            if first {
                first = false;
            } else {
                #[cfg(debug_assertions)]
                print!(" and")
            }
            match material {
                Material::Ore(n) => {
                    #[cfg(debug_assertions)]
                    print!(" {n} ore");
                    self.ore -= *n;
                }
                Material::Clay(n) => {
                    #[cfg(debug_assertions)]
                    print!(" {n} clay");
                    self.clay -= *n;
                }
                Material::Obsidian(n) => {
                    #[cfg(debug_assertions)]
                    print!(" {n} obsidian");
                    self.obsidian -= *n;
                }
            }
        }
        #[cfg(debug_assertions)]
        println!(" to start building a {}.", _bot);

        true
    }
}

impl Add for Materials {
    type Output = Materials;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
        self
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

fn get_best(blueprint: &Blueprint, turns: u8) -> u8 {
    let mut queue = vec![(Materials::default(), Bots::default(), 0)];
    let mut max = 0;
    let mut res = 0;
    for _ in 0..turns {
        let mut next_max = max;
        for (materials, bots, skipped) in std::mem::take(&mut queue) {
            if bots.geode < max {
                continue;
            }

            let production = bots.produce(&materials);
            res = res.max(materials.geode + production.geode);

            let mut skippable = false;
            {
                let mut materials = materials;
                if materials.build(Bot::Geode, &blueprint.geode_bot) {
                    let mut bots = bots;
                    bots.geode += 1;
                    #[cfg(debug_assertions)]
                    println!(
                        "The new geode-cracking robot is ready; you now have {} of them.",
                        bots.geode
                    );

                    queue.push((materials + production, bots, 0));
                    next_max = bots.geode;
                    continue; // skip other bots
                }
            }
            {
                let mut materials = materials;
                if blueprint.get_cost(Bot::Geode, Material::Obsidian(0)) > bots.obsidian
                    && materials.build(Bot::Obsidian, &blueprint.obsidian_bot)
                {
                    let mut bots = bots;
                    bots.obsidian += 1;
                    #[cfg(debug_assertions)]
                    println!(
                        "The new obsidian-collecting robot is ready; you now have {} of them.",
                        bots.obsidian
                    );

                    queue.push((materials + production, bots, 0));
                    skippable = true;
                }
            }
            {
                let mut materials = materials;
                if blueprint.get_cost(Bot::Obsidian, Material::Clay(0)) > bots.clay
                    && materials.build(Bot::Clay, &blueprint.clay_bot)
                {
                    let mut bots = bots;
                    bots.clay += 1;
                    #[cfg(debug_assertions)]
                    println!(
                        "The new clay-collecting robot is ready; you now have {} of them.",
                        bots.clay
                    );

                    queue.push((materials + production, bots, 0));
                    skippable = true;
                }
            }
            {
                let mut materials = materials;
                if blueprint
                    .get_cost(Bot::Geode, Material::Ore(0))
                    .max(blueprint.get_cost(Bot::Obsidian, Material::Ore(0)))
                    .max(blueprint.get_cost(Bot::Clay, Material::Ore(0)))
                    .max(blueprint.get_cost(Bot::Ore, Material::Ore(0)))
                    > bots.ore
                    && materials.build(Bot::Ore, &blueprint.ore_bot)
                {
                    let mut bots = bots;
                    bots.ore += 1;
                    #[cfg(debug_assertions)]
                    println!(
                        "The new ore-collecting robot is ready; you now have {} of them.",
                        bots.ore
                    );

                    queue.push((materials + production, bots, 0));
                    skippable = true;
                }
            }

            if skipped < 3 || !skippable {
                queue.push((
                    materials + production,
                    bots,
                    if skippable { skipped + 1 } else { 0 },
                ));
            }
        }
        max = next_max;
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
    for blueprint in &blueprints {
        sum += get_best(blueprint, 24) as u16 * blueprint.id as u16;
    }
    println!("{sum}"); // 1528

    let mut mul = 1;
    for blueprint in blueprints.into_iter().take(3) {
        mul *= get_best(&blueprint, 32) as u16;
    }
    println!("{mul}"); // 16926
}
