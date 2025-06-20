// xorshift(seed)の余りの計算が不適切
use std::{cmp::max, vec};
use chrono::Local;
use colored::Colorize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Role {
    Rock,
    Scissors,
    Paper,
}

#[derive(Clone)]
enum VOrD { // victory or defeat
    Win,
    Draw,
    Lose
}

impl Role {
    fn random() -> Self {
        let this_seed: u64 = Local::now().timestamp_millis().abs() as u64;
        let roles: [Role; 3] = [Role::Rock, Role::Scissors, Role::Paper];
        let index: u64 = xorshift(this_seed) % 3;
        roles[index as usize]
    }
}

fn xorshift(seed: u64) -> u64 {
    let mut x: u64 = seed; 
    x = x ^ (x << 13);
    x = x ^ (x >> 7);
    x = x ^ (x << 17);
    x
}

fn run() -> Vec<bool> {
    let player1: Role = Role::random();
    let player2: Role = Role::random();

    match (player1,player2){
        (Role::Rock, Role::Scissors) => vec![true,false],
        (Role::Scissors, Role::Paper) => vec![true,false],
        (Role::Paper, Role::Rock) => vec![true,false],
        (Role::Rock, Role::Rock) => vec![false,false],
        (Role::Scissors, Role::Scissors) => vec![false,false],
        (Role::Paper, Role::Paper) => vec![false,false],
        (Role::Scissors, Role::Rock) => vec![false,true],
        (Role::Paper, Role::Scissors) => vec![false,true],
        (Role::Rock, Role::Paper) => vec![false,true],
    }
}

fn is_draw(judge: Vec<bool>) -> bool {
    if judge[0] == judge[1] {
        return true
    }
    false
}

fn one_play() -> VOrD {
    let win: Vec<bool> = run();
    let draw: bool = is_draw(win.clone());
    if draw {
        VOrD::Draw
    } else {
        if win[1] == true {
            VOrD::Win
        } else {
            VOrD::Lose
        }
    }
}

fn main() {
    let mut win_cnt: usize = 0;
    let mut max_win: usize = 0;
    let mut play_cnt: usize = 0;
    let mut bin_cha: isize = 1; //binary_chance

    loop {
        let played: VOrD = one_play();
        play_cnt += 1;
        match played.clone() {
            VOrD::Win => {
                if win_cnt+1 > max_win {
                    bin_cha = bin_cha << 1;
                    println!("{}\nプレイ数: {}\n最高連勝数: {}\n確率: 1/{}({}%)\n平均との差: {} play{}",
                        "最高連勝数更新！！".red().bold(),
                        play_cnt,
                        win_cnt+1,
                        bin_cha,
                        100.0/(bin_cha as f64),
                        (bin_cha - (play_cnt as isize)).abs(),
                        if (bin_cha - (play_cnt as isize)).is_positive() {
                            "早い"
                        } else if (bin_cha - (play_cnt as isize)) == 0 {
                            "(変わらない)"
                        } else {
                            "遅い"
                        }
                    );
                }

                max_win = max(win_cnt + 1,max_win);
                win_cnt += 1;
            },
            VOrD::Draw =>{ 
                win_cnt += 0;
                max_win += 0;
            },
            VOrD::Lose => {
                win_cnt = 0
            }
        };
        if play_cnt % 1000000 == 0 {
            println!("{}{}", (play_cnt/1000000).to_string().blue().bold(), " million(100万) played.".bright_blue().bold())
        }

    }
}
