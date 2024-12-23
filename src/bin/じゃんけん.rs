use rand::Rng;
use std::collections::HashSet;

fn main() {
    let n: u16 = 100;

    if n == 0 {
        println!("No games to play.");
        return;
    }

    let mut total_turns: usize = 0;

    for _ in 0..n {
        total_turns += play(5, false);
    }

    let average_turns: f64 = total_turns as f64 / n as f64;
    println!("Average turns: {:.2}", average_turns);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Role {
    Rock,
    Scissors,
    Paper,
}

impl Role {
    fn random() -> Self {
        let roles = [Role::Rock, Role::Scissors, Role::Paper];
        let index = rand::thread_rng().gen_range(0, roles.len());
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
        unique_roles.contains(&Role::Rock)
            && unique_roles.contains(&Role::Scissors)
            && unique_roles.contains(&Role::Paper)
    }
}

fn play(player_amount: usize, debug: bool) -> usize {
    let mut obj = Janken::new(player_amount);
    let mut did_turn = 0;
    let max_iterations = 114514; // Prevent infinite loop with a cap. 114514

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
                println!("turn end");
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
