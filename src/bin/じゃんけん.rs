use rand::Rng;
use std::{collections::HashSet,cmp::max};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Role {
    Rock,
    Scissors,
    Paper,
}

impl Role {
    fn random() -> Self {
        let roles = [Role::Rock, Role::Scissors, Role::Paper];
        let index = rand::thread_rng().gen_range(0,roles.len());
        roles[index]
    }
}

#[derive(Debug)]
struct Janken {
    player_amount: usize,
    result: Vec<Role>,
}

impl Janken {
    fn new(player_amount: usize) -> Self {
        Janken {
            player_amount,
            result: Vec::new(),
        }
    }

    fn recalc(&mut self) {
        self.result = (0..self.player_amount)
            .map(|_| Role::random())
            .collect();
    }

    fn is_draw(&self) -> bool {
        let unique_roles: HashSet<_> = self.result.iter().collect();
        let draw_helper: bool = unique_roles.contains(&Role::Rock)
            && unique_roles.contains(&Role::Scissors)
            && unique_roles.contains(&Role::Paper);
        let draw_helper2: bool = self.result.iter().all_equal();
        draw_helper || draw_helper2
    }
}

fn play(player_amount: usize, debug: bool) -> usize {
    let mut obj = Janken::new(player_amount);
    let mut did_turn = 0;
    let max_iterations = 114514; // Prevent infinite loop with a cap

    loop {
        if did_turn >= max_iterations {
            eprintln!("Exceeded maximum iteration limit: {}", max_iterations);
            break;
        }

        obj.recalc();
        if obj.is_draw() {
            did_turn += 1;
            if debug {
                println!("{:?}", obj.result);
            }
        } else {
            if debug {
                println!("{:?}", obj.result);
                println!("{}", did_turn);
            }
            break;
        }
    }

    did_turn
}

fn run_simulation(n: usize, player_count: usize) {
    if n == 0 {
        println!("No games to play.");
        return;
    }

    let mut total_turns: usize = 0;
    let mut max_turns: usize = 0;

    for _ in 0..n {
        let rec_turns = play(player_count, false);
        total_turns += rec_turns;
        max_turns = max(max_turns, rec_turns);
    }

    let average_turns: f64 = total_turns as f64 / n as f64;
    println!("Average turns: {:.2}", average_turns);
    println!("Maximum turns: {}", max_turns);
}

fn main() {
    let n: usize = 100;
    let player_count: usize = 10;
    run_simulation(n, player_count);
}
