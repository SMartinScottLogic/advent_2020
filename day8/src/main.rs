use anyhow::Result;
use log::info;
use day8::load;

fn main() -> Result<()> {
    env_logger::init();

    for emoji in emojis::iter() {
        info!("{} {} {:?}", emoji.as_str(), emoji.name(), emoji.group());
    }
    let mut solution = load("input.day8.full")?;
    info!(
        "{} {}: {:?}",
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
