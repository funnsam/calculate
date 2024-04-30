use smolcalc::*;

fn main() {
    let arg = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    //let n = to_nodes::<rational::complex::ComplexRational<num_bigint::BigInt>>(&arg).and_then(|n|
    // n.evaluate());
    println!("{:?}", to_nodes::<f64>(&arg).and_then(|n| n.evaluate()));
    let n = to_nodes::<rational::Rational<num_bigint::BigInt>>(&arg)
        .and_then(|n| n.evaluate())
        .map(|v| v.limit_denom(1_000_000_000_000_000_u64.into()));

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
            println!("= {n:#}");
        },
    );
}
