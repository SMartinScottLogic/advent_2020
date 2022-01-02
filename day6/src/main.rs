use anyhow::Result;
use day6::load;
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input.day6.full")?;
    info!(
        "{} {}: {:#?}",
        emojis::lookup("christmas_tree")
            .map(|emoji| emoji.as_str())
            .unwrap_or(""),
        "solution",
        solution
    );
    solution.analyse();
    info!(
        "{} part1 answer is {:?}",
        emojis::lookup("santa")
            .map(|emoji| emoji.as_str())
            .unwrap_or(""),
        solution.answer_part1()
    );
    info!(
        "{} part2 answer is {:?}",
        emojis::lookup("santa")
            .map(|emoji| emoji.as_str())
            .unwrap_or(""),
        solution.answer_part2()
    );

    Ok(())
}
