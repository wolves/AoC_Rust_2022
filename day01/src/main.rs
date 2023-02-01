use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let max = include_str!("input.txt")
    //     .lines()
    //     .map(|v| v.parse::<u64>().ok())
    //     .batching(|it| {
    //         let mut sum = None;
    //         while let Some(Some(v)) = it.next() {
    //             sum = Some(sum.unwrap_or(0) + v);
    //         }
    //         sum
    //     })
    //     .max();
    //
    // println!("{max:?}");

    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .flatten()
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .sum::<u64>();

    println!("{answer:?}");

    Ok(())
}
