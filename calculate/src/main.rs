use smolcalc::*;

fn evaluate<T: core::fmt::Display + traits::ComputableNumeral, F: Fn(T) -> String>(s: &str, f: F) {
    match to_nodes::<T>(s) {
        Ok(n) => match n.evaluate() {
            Ok(v) => {
                show_int(&n, s);
                println!("= {}", f(v));
            },
            Err(e) => {
                show_int(&n, s);
                report(s, e);
            },
        },
        Err(e) => {
            report(s, e);
        }
    }
}

fn report(src: &str, err: Error) {
    println!(
        "\
\x1b[1;31mError:\x1b[0m {}
  {src}
  \x1b[1;33m{:<2$}{1:^<3$}\x1b[0m",
        err.message,
        "",
        err.location.start,
        err.location.end - err.location.start
    )
}

pub struct Eval {
    pub output: String,
    pub latex: String,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next();
    let expr = args.collect::<Vec<String>>().join(" ");

    match mode.as_deref() {
        Some("f32") => evaluate::<f32, _>(&expr, |a| trunc(&format!("{a:.5}")).to_string()),
        Some("f64") => evaluate::<f64, _>(&expr, |a| trunc(&format!("{a:.13}")).to_string()),
        Some("rat") => evaluate::<rational::Rational<num_bigint::BigInt>, _>(&expr, |a| format!("{a:#}")),
        Some("c32") => evaluate::<num_complex::Complex<f32>, _>(&expr, |a| pretty_cmplx(a, |a| trunc(&format!("{a:.5}")).to_string())),
        Some("c64") => evaluate::<num_complex::Complex<f64>, _>(&expr, |a| pretty_cmplx(a, |a| trunc(&format!("{a:.13}")).to_string())),
        Some("crat") => evaluate::<rational::complex::ComplexRational<num_bigint::BigInt>, _>(&expr, |a| format!("{a:#}")),
        Some(m) => {
            println!("\x1b[1;31mError:\x1b[0m mode `{m}` not supported!");
            std::process::exit(1);
        },
        None => {
            println!("\x1b[1;31mError:\x1b[0m specify a mode!");
            std::process::exit(1);
        },
    };
}

fn trunc(s: &str) -> &str {
    match s.as_bytes().last() {
        Some(b'0') => trunc(&s[..s.len() - 1]),
        Some(b'.') => &s[..s.len() - 1],
        _ => s,
    }
}

fn show_int<T: core::fmt::Display>(i: &Node<T>, src: &str) {
    println!("\x1b[1mInput interpretation:\x1b[0m ${}$", latex::LatexDisplay {
        node: i,
        src,
    });
}

fn pretty_cmplx<T: num_traits::Float + num_traits::Signed, F: Fn(T) -> String>(
    c: num_complex::Complex<T>,
    f: F,
) -> String {
    if c.im.is_zero() {
        f(c.re)
    } else if c.re.is_zero() {
        f(c.im) + "i"
    } else if !c.im.is_negative() {
        format!("{}+{}i", f(c.re), f(c.im))
    } else {
        format!("{}-{}i", f(c.re), f(c.im.abs()))
    }
}
