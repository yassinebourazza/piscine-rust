use edit_distance::*;

fn main() {
    let source = "rosettacode";
    let target = "raisethysword";

    println!(
        "It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance(source, target),
        source,
        target
    );
}