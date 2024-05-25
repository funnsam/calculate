use smolcalc::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next();
    let expr = args.collect::<Vec<String>>().join(" ");

    let n = match mode.as_deref() {
        Some("cmplx") => to_nodes::<rational::complex::ComplexRational<num_bigint::BigInt>>(&expr)
            .map(|i| show_int(i, &expr))
            .and_then(|n| n.evaluate())
            .map(|v| v.limit_denom(1_000_000_000_000_000_u64.into()))
            .map(|v| format!("{v:#}")),
        Some("rat") => to_nodes::<rational::Rational<num_bigint::BigInt>>(&expr)
            .map(|i| show_int(i, &expr))
            .and_then(|n| n.evaluate())
            .map(|v| v.limit_denom(1_000_000_000_000_000_u64.into()))
            .map(|v| format!("{v:#}")),
        Some("f32") => to_nodes::<f32>(&expr)
            .map(|i| show_int(i, &expr))
            .and_then(|n| n.evaluate())
            .map(|v| trunc(&format!("{v:.5}")).to_string()),
        Some("f64") => to_nodes::<f64>(&expr)
            .map(|i| show_int(i, &expr))
            .and_then(|n| n.evaluate())
            .map(|v| trunc(&format!("{v:.13}")).to_string()),
        Some(m) => {
            println!("\x1b[1;31mError:\x1b[0m mode `{m}` not supported!");
            std::process::exit(1);
        },
        None => {
            println!("\x1b[1;31mError:\x1b[0m specify a mode!");
            std::process::exit(1);
        },
    };

    n.map_or_else(
        |s| {
            println!("\x1b[1;31mError:\x1b[0m {}", s.message);
            println!("  {expr}");
            println!(
                "  \x1b[33m{:<1$}{0:^<2$}\x1b[0m",
                "",
                s.location.start,
                s.location.end - s.location.start
            );
        },
        |n| {
            println!("= {n:#}");
        },
    );
}

fn trunc(s: &str) -> &str {
    match s.as_bytes().last() {
        Some(b'0') => trunc(&s[..s.len() - 1]),
        Some(b'.') => &s[..s.len() - 1],
        _ => s,
    }
}

fn show_int<T: core::fmt::Display>(i: Node<T>, src: &str) -> Node<T> {
    println!("\x1b[1mInput interpretation:\x1b[0m ${}$", latex::LatexDisplay {
        node: &i,
        src,
    });
    i
}
