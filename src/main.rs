fn main() {
    let arg = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    #[cfg(feature = "any_num")]
    let n = calculate::to_nodes::<f64>(&arg).and_then(|n| n.evaluate());

    #[cfg(not(feature = "any_num"))]
    let n = calculate::to_nodes::<f32>(&arg);

    n.map_or_else(
        |s| {
            println!("\x1b[1;31mError:\x1b[0m");
            println!("  {arg}");
            println!(
                "  \x1b[33m{:<1$}{2:^<3$}\x1b[0m",
                "",
                s.start,
                "",
                s.end - s.start
            );
        },
        |n| {
            println!("\x1b[1m=\x1b[0m {n}");
        },
    );
}
