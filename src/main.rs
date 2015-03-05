extern crate num;

use num::bigint::BigInt;
use num::rational::Ratio;

type ORatio = Option<Ratio<BigInt>>;

struct TipResult {
    amt: Ratio<BigInt>,
    tip: Ratio<BigInt>,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let tip = match &args[..] {
        [_, ref amt] => get_tip(ratio(&amt), Ratio::from_float(0.15f32)),
        [_, ref amt, ref pct] => get_tip(ratio(&amt), ratio(&pct)),
        _ => None,
    };

    match tip {
        Some(tip) => println!("${:.2} on ${:.2}", tip.tip, tip.amt.numer() / tip.amt.denom()),
        None => println!("USAGE: {} AMOUNT <decimal> [TIP PERCENT] <decimal>", args[0]),
    }
}

fn get_tip(amt: ORatio, pct: ORatio) -> Option<TipResult> {
    if amt.is_none() || pct.is_none() {
        return None;
    }

    let amt = amt.unwrap();
    let pct = pct.unwrap();

    Some(TipResult {
        amt: amt.clone(),
        tip: &amt * &pct,
    })
}

fn ratio(input: &str) -> ORatio {
    match input.parse::<f32>() {
        Ok(f) => Ratio::from_float(f),
        _ => None,
    }
}
