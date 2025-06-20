use rand::Rng;

fn main() {
    let mut all_roll: u64 = 0;
    let n = 100;
    let amount: u64 = 4;
    let side_amount: u64 = 6;

    for _ in 0..n {
        all_roll += dice_roll(amount, side_amount);
    }

    let average_roll: f64 = all_roll as f64 / n as f64;
    println!("{}d{} Stats:\n average: {}", amount, side_amount, average_roll)
}

fn dice_roll(amount: u64, hedron_amount: u64) -> u64 {
    let mut all_roll: u64 = 0;
    for _ in 0..amount {
        all_roll += rand::thread_rng().gen_range(1, hedron_amount + 1);
    }
    all_roll
}