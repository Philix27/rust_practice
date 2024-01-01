mod learning;

use learning::LearningTips;

fn main() {
    let b = String::from("Hello");

    let val = LearningTips::SetReward;

    let val = val.get_id();

    print!("Returned value: {}", val)
}
