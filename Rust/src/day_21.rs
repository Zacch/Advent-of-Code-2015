use std::fs;
use crate::traits::StringExtensions;

pub struct Character {
    pub hit_points: i32,
    pub damage: i32,
    pub armor: i32,
}

struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

pub fn run() {
    println!("Day 21");
    let contents = fs::read_to_string("input/day_21.txt").unwrap();

    let numbers = contents.to_int_vector();
    let boss = Character { hit_points: numbers[0], damage: numbers[1], armor: numbers[2] };

    let weapons: Vec<Item> = vec![
        Item { name: "Dagger".to_string(), cost: 8, damage: 4, armor: 0, },
        Item { name: "Shortsword".to_string(), cost: 10, damage: 5, armor: 0, },
        Item { name: "Warhammer".to_string(), cost: 25, damage: 6, armor: 0, },
        Item { name: "Longsword".to_string(), cost: 40, damage: 7, armor: 0, },
        Item { name: "Greataxe".to_string(), cost: 74, damage: 8, armor: 0, },
    ];

    let armours: Vec<Item> = vec![
        Item { name: "None".to_string(), cost: 0, damage: 0, armor: 0, },
        Item { name: "Leather".to_string(), cost: 13, damage: 0, armor: 1, },
        Item { name: "Chainmail".to_string(), cost: 31, damage: 0, armor: 2, },
        Item { name: "Splintmail".to_string(), cost: 53, damage: 0, armor: 3, },
        Item { name: "Bandedmail".to_string(), cost: 75, damage: 0, armor: 4, },
        Item { name: "Platemail".to_string(), cost: 102, damage: 0, armor: 5, },
    ];

    let rings: Vec<Item> = vec![
        Item { name: "None 1".to_string(), cost: 0, damage: 0, armor: 0, },
        Item { name: "None 2".to_string(), cost: 0, damage: 0, armor: 0, },
        Item { name: "Damage +1".to_string(), cost: 25, damage: 1, armor: 0, },
        Item { name: "Damage +2".to_string(), cost: 50, damage: 2, armor: 0, },
        Item { name: "Damage +3".to_string(), cost: 100, damage: 3, armor: 0, },
        Item { name: "Defense +1".to_string(), cost: 20, damage: 0, armor: 1, },
        Item { name: "Defense +2".to_string(), cost: 40, damage: 0, armor: 2, },
        Item { name: "Defense +3".to_string(), cost: 80, damage: 0, armor: 3, },
    ];

    let mut part1: i32 = 1234567;
    let mut part2: i32 = 0;

    for weapon in &weapons {
        for armour in &armours {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1.name == ring2.name { continue; }
                    let gear: Vec<&Item> = vec![weapon, armour, ring1, ring2];
                    let cost = gear.iter().map(|i| i.cost).sum();
                    let player = Character {
                        hit_points: 100,
                        damage: gear.iter().map(|i| i.damage).sum(),
                        armor: gear.iter().map(|i| i.armor).sum(),
                    };
                    if player_wins_fight(&player, &boss) {
                        if cost < part1 { part1 = cost; }
                    } else {
                        if cost > part2 { part2 = cost; }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn player_wins_fight(player: &Character, boss: &Character) -> bool {
    let mut player_hit_points = player.hit_points;
    let mut boss_hit_points = boss.hit_points;
    let mut players_turn = true;
    loop {
        if players_turn {
            boss_hit_points -= 1.max(player.damage - boss.armor);
            if boss_hit_points <= 0 { return true; }
        } else {
            player_hit_points -= 1.max(boss.damage - player.armor);
            if player_hit_points <= 0 { return false; }
        }
        players_turn = !players_turn;
    }
}
