use num_integer::Integer;
use once_cell::sync::Lazy;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type Sockets = (u8, u8);

pub type Combo = [usize; 6];

pub type TreasureCollection = [usize; 10];

type BonusMap = HashMap<Combo, Bonus>;

type SocketMap = HashMap<Sockets, Vec<Combo>>;

type TreasureMap = HashMap<Combo, Appraisal>;

type ComboMap = HashMap<Combo, ((Combo, Combo), usize)>;

#[derive(serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Color {
    Blue = 0,
    Green = 1,
    Purple = 2,
    Red = 3,
    Yellow = 4
}

#[derive(Clone, Copy, Debug, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Shape {
    Circular = 0,
    Rectangular = 1
}

#[derive(Clone, Copy, Debug)]
enum Bonus {
    TwoColors = 11,
    Duo = 12,
    ThreeColors = 13,
    Trio = 14,
    TwoDuos = 15,
    FourColors = 16,
    Quartet = 17,
    DuoAndTrio = 18,
    Quintet = 19,
    FiveColors = 20
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum ObjectiveFunction {
    MaximizeBonus = 0,
    MaximizeValue = 1
}

#[derive(Debug, serde::Serialize)]
pub struct Appraisal {
    pub value: usize,
    pub bonus: usize
}

#[derive(serde::Serialize)]
pub struct Gem {
    pub id: u8,
    pub name: &'static str,
    pub shape: Shape,
    pub color: Color,
    pub value: u16
}

#[derive(serde::Serialize)]
pub struct Treasure {
    pub id: u8,
    pub name: &'static str,
    pub sockets: Sockets,
    pub value: u16
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Socket {
    shape: Shape,
    gem_id: Option<u8>
}

#[derive(Debug, serde::Serialize)]
pub struct SocketedTreasure {
    pub id: u8,
    pub sockets: Vec<Socket>,
    pub appraisal: Appraisal
}

pub static GEMS: [Gem; 6] = [
    Gem {
        id: 1,
        name: "Ruby",
        shape: Shape::Circular,
        color: Color::Red,
        value: 3000
    },
    Gem {
        id: 2,
        name: "Sapphire",
        shape: Shape::Circular,
        color: Color::Blue,
        value: 4000
    },
    Gem {
        id: 3,
        name: "Yellow Diamond",
        shape: Shape::Circular,
        color: Color::Yellow,
        value: 7000
    },
    Gem {
        id: 4,
        name: "Emerald",
        shape: Shape::Rectangular,
        color: Color::Green,
        value: 5000
    },
    Gem {
        id: 5,
        name: "Alexandrite",
        shape: Shape::Rectangular,
        color: Color::Purple,
        value: 6000
    },
    Gem {
        id: 6,
        name: "Red Beryl",
        shape: Shape::Rectangular,
        color: Color::Red,
        value: 9000
    }
];

pub static TREASURES: [Treasure; 10] = [
    Treasure {
        id: 1,
        name: "Flagon",
        sockets: (2, 0),
        value: 4000
    },
    Treasure {
        id: 2,
        name: "Splendid Bangle",
        sockets: (0, 2),
        value: 4000
    },
    Treasure {
        id: 3,
        name: "Elegant Bangle",
        sockets: (2, 0),
        value: 5000
    },
    Treasure {
        id: 4,
        name: "Elegant Mask",
        sockets: (3, 0),
        value: 5000
    },
    Treasure {
        id: 5,
        name: "Butterfly Lamp",
        sockets: (3, 0),
        value: 6000
    },
    Treasure {
        id: 6,
        name: "Chalice of Atonement",
        sockets: (0, 3),
        value: 7000
    },
    Treasure {
        id: 7,
        name: "Extravagant Clock",
        sockets: (1, 1),
        value: 9000
    },
    Treasure {
        id: 8,
        name: "Ornate Necklace",
        sockets: (2, 2),
        value: 11000
    },
    Treasure {
        id: 9,
        name: "Golden Lynx",
        sockets: (2, 1),
        value: 15000
    },
    Treasure {
        id: 10,
        name: "Elegant Crown",
        sockets: (2, 3),
        value: 19000
    }
];

static BONUS_MAP: Lazy<BonusMap> = Lazy::new(create_bonus_map);

static SOCKET_MAP: Lazy<SocketMap> = Lazy::new(|| {
    let mut map: SocketMap = HashMap::new();

    for treasure in TREASURES.iter() {
        if map.contains_key(&treasure.sockets) {
            continue;
        }

        let (circ, rect) = treasure.sockets;
        map.insert(
            treasure.sockets,
            combos(circ.into(), rect.into()));
    }

    map
});

pub fn allocate_gems(
    treasure_collection: TreasureCollection,
    gems: Combo,
    obj_fn: ObjectiveFunction
) -> Option<Vec<SocketedTreasure>> {
    let treasures = treasures_from_collection(treasure_collection);

    if treasures.is_empty() {
        return Some(Vec::new());
    }

    if treasures.len() == 1 {
        let treasure = treasures[0];
        let combo: TreasureMap = get_valid_combos(treasure, &gems)?;
        let solution = combo.iter()
            .max_by(|a, b| {
                match obj_fn {
                    ObjectiveFunction::MaximizeBonus => a.1.bonus.cmp(&b.1.bonus),
                    ObjectiveFunction::MaximizeValue => a.1.value.cmp(&b.1.value)
                }
            })
            .map(|(k, _)| *k)?;

        return Some(vec![create_socketed_treasure(treasure, &solution)]);
    }

    let mut sub_solutions: Vec<ComboMap> = Vec::with_capacity(treasures.len() - 1);

    // Compute first sub-solution from first two treasures.
    let mut combo: TreasureMap = get_valid_combos(treasures[1], &gems)?;
    sub_solutions.push(compute_sub_solution(
        &get_valid_combos(treasures[0], &gems)?,
        &combo,
        &gems,
        &obj_fn
    ));

    for treasure in treasures.iter().skip(2) {
        let sub_solution = sub_solutions.last()?;
        combo = get_valid_combos(treasure, &gems)?;

        // Combine with next treasure.
        let mut map: ComboMap = HashMap::new();

        for a in sub_solution.iter() {
            for b in combo.iter() {
                let key = add_combos(a.0, b.0);
                if compare_combos(&key, &gems) == Ordering::Greater {
                    continue;
                }

                let value = map.get(&key);
                let sum = match obj_fn {
                    ObjectiveFunction::MaximizeBonus => a.1.1 + b.1.bonus,
                    ObjectiveFunction::MaximizeValue => a.1.1 + b.1.value
                };

                if value.is_some() && sum <= value.unwrap().1 {
                    continue;
                }

                // Insert or update entry.
                map.insert(key, ((*a.0, *b.0), sum));
            }
        }

        sub_solutions.push(map);
    }

    let solution = trace_optimal_solution(&sub_solutions)?;

    Some(solution.iter().enumerate()
        .fold(
            Vec::with_capacity(treasures.len()),
            |mut list, (i, gems)| {
                list.push(create_socketed_treasure(treasures[i], gems));

                list
            }
        ))
}

fn trace_optimal_solution(sub_solutions: &Vec<ComboMap>) -> Option<Vec<Combo>> {
    // Find largest starting value.
    let max = sub_solutions.last().unwrap().iter()
        .max_by(|a, b| a.1.1.cmp(&b.1.1))
        .map(|(_, v)| (*v).0)?;

    let mut solution: Vec<Combo> = Vec::with_capacity(sub_solutions.len() + 1);
    solution.insert(0, max.1);

    let mut key: &Combo = &max.0;
    let mut value: (Combo, Combo);
    for sub_solution in sub_solutions.iter().rev().skip(1) {
        value = sub_solution.get(key)?.0;
        solution.insert(0, value.1);
        key = &value.0;
    }

    // Add first one
    solution.insert(0, *key);

    Some(solution)
}

fn compute_sub_solution(
    t1: &TreasureMap,
    t2: &TreasureMap,
    gems: &Combo,
    obj_fn: &ObjectiveFunction
) -> ComboMap {
    let mut sub_solution: ComboMap = HashMap::new();

    for a in t1.iter() {
        for b in t2.iter() {
            let key = add_combos(a.0, b.0);
            if compare_combos(&key, gems) == Ordering::Greater {
                continue;
            }

            let value = sub_solution.get(&key);
            let sum = match obj_fn {
                ObjectiveFunction::MaximizeBonus => a.1.bonus + b.1.bonus,
                ObjectiveFunction::MaximizeValue => a.1.value + b.1.value
            };

            if value.is_some() && sum <= value.unwrap().1 {
                continue;
            }

            // Insert or update entry.
            sub_solution.insert(key, ((*a.0, *b.0), sum));
        }
    }

    sub_solution
}

fn get_valid_combos(treasure: &Treasure, gems: &Combo) -> Option<TreasureMap> {
    let combos = SOCKET_MAP.get(&treasure.sockets)?;
    let mut map: TreasureMap = HashMap::new();

    for combo in combos {
        // Filter out invalid combos
        if compare_combos(combo, gems) == Ordering::Greater {
            continue;
        }

        map.insert(*combo, appraise(treasure, combo));
    }

    Some(map)
}

fn combos(circ: usize, rect: usize) -> Vec<Combo> {
    let size = binomial(circ + 3, circ) * binomial(rect + 3, rect);
    let mut values: Vec<Combo> = Vec::with_capacity(size);

    let v = combinations(cmp::max(circ, rect));

    for i in 0..=circ {
        for j in 0..=rect {
            for k in 0..v[i].len() {
                for m in 0..v[j].len() {
                    values.push([
                        v[i][k][0], v[i][k][1], v[i][k][2],
                        v[j][m][0], v[j][m][1], v[j][m][2]
                    ]);
                }
            }
        }
    }

    values
}

fn combinations(sockets: usize) -> Vec<Vec<[usize; 3]>> {
    let k: usize = 2;
    let mut values: Vec<Vec<[usize; 3]>> = Vec::with_capacity(sockets + 1);

    for h in 0..=sockets {
        let n: usize = h + k;
        values.push(Vec::with_capacity(binomial(n, k)));

        for i in 0..n {
            for j in i+1..n {
                values[h].push([i, j - i - 1, h - (j - 1)]);
            }
        }
    }

    values
}

fn treasures_from_collection(
    collection: TreasureCollection
) -> Vec<&'static Treasure> {
    collection.iter()
        .enumerate()
        .fold(
            Vec::new() as Vec<&Treasure>,
            |mut list, (i, count)| {
            (0..*count).for_each(|_| list.push(&TREASURES[i]));

            list
        }
    )
}

fn create_socketed_treasure(treasure: &Treasure, gems: &Combo) -> SocketedTreasure {
    SocketedTreasure {
        id: treasure.id,
        sockets: gems.iter().enumerate().fold(
            Vec::new(),
            |mut sockets, (i, count)| {
                for _ in 0..*count {
                    sockets.push(Socket {
                        shape: GEMS[i].shape,
                        gem_id: Some(GEMS[i].id)
                    });
                }

                // Account for empty circular sockets
                if i == 2 {
                    let sum: usize = gems.iter().take(3).map(|x| *x).sum();
                    for _ in 0..(usize::from(treasure.sockets.0) - sum) {
                        sockets.push(Socket {
                            shape: Shape::Circular,
                            gem_id: None
                        });
                    }
                }

                // Account for empty rectangular sockets
                if i == 5 {
                    let sum: usize = gems.iter().skip(3).map(|x| *x).sum();
                    for _ in 0..(usize::from(treasure.sockets.1) - sum) {
                        sockets.push(Socket {
                            shape: Shape::Rectangular,
                            gem_id: None
                        });
                    }
                }

                sockets
            }),
        appraisal: appraise(treasure, gems)
    }
}

fn binomial<T: Integer + Clone>(n: T, k: T) -> T {
    match k {
        x if x == T::zero() => T::one(),
        _ => num_integer::binomial(n, k)
    }
}

fn compare_combos(a: &Combo, b: &Combo) -> Ordering {
    let mut ordering = Ordering::Equal;

    for i in 0..a.len() {
        if a[i] > b[i] {
            return Ordering::Greater;
        }

        if a[i] < b[i] {
            ordering = Ordering::Less;
        }
    }

    ordering
}

fn add_combos(a: &Combo, b: &Combo) -> Combo {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3], a[4] + b[4], a[5] + b[5]]
}

pub fn appraise(treasure: &Treasure, gems: &[usize; 6]) -> Appraisal {
    let mut appraisal = Appraisal {
        value: treasure.value.into(),
        bonus: 0
    };

    appraisal.value += gems.iter().enumerate()
        .fold(0, |sum, (i, &count)|  {
            sum + count * usize::from(GEMS[i].value)
        }
    );

    match BONUS_MAP.get(gems) {
        Some(&x) => {
            let multiplier = (x as usize) - 10;
            appraisal.bonus = (multiplier * appraisal.value) / 10;
            appraisal.value += appraisal.bonus;
        },
        None => ()
    };

    appraisal
}

fn create_bonus_map() -> BonusMap {
    let mut values: BonusMap = HashMap::new();
    let v = combinations(3);

    for i in 0..=3 {
        for j in 0..=3 {
            if (i + j < 2) || (i + j > 5) {
                continue;
            }

            for k in 0..v[i].len() {
                for m in 0..v[j].len() {
                    let gems = [
                        v[i][k][0], v[i][k][1], v[i][k][2],
                        v[j][m][0], v[j][m][1], v[j][m][2]
                    ];

                    match get_bonus(&gems) {
                        Some(x) => values.insert(gems, x),
                        None => None
                    };
                }
            }
        }
    }

    values
}

fn get_bonus(gems: &[usize; 6]) -> Option<Bonus> {
    let mut colors: [usize; 5] = [
        gems[0] + gems[5], gems[1], gems[2], gems[3], gems[4]
    ];

    colors.sort_by(|a, b| b.cmp(a));

    match colors[0] {
        5 => Some(Bonus::Quintet),
        4 => Some(Bonus::Quartet),
        3 => match colors[1] {
            2 => Some(Bonus::DuoAndTrio),
            _ => Some(Bonus::Trio)
        },
        2 => match colors[1] {
            2 => Some(Bonus::TwoDuos),
            1 => match colors[2] {
                1 => match colors[3] {
                    1 => match colors[4] {
                        1 => Some(Bonus::FourColors),
                        _ => Some(Bonus::ThreeColors)
                    },
                    _ => Some(Bonus::ThreeColors)
                },
                _ => Some(Bonus::Duo)
            },
            _ => Some(Bonus::Duo)
        },
        1 => match colors[1] {
            1 => match colors[2] {
                1 => match colors[3] {
                    1 => match colors[4] {
                        1 => Some(Bonus::FiveColors),
                        _ => Some(Bonus::FourColors)
                    },
                    _ => Some(Bonus::ThreeColors)
                },
                _ => Some(Bonus::TwoColors)
            },
            _ => None
        },
        _ => None
    }
}
