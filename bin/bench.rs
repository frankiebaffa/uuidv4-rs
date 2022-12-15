use {
    std::{
        env::args,
        process::exit,
        time::Instant,
    },
    uuidv4_rs::uuidv4,
};
fn main() {
    let max = {
        let max_str = match args().nth(1) {
            Some(arg) => arg,
            None => String::from("10000"),
        };
        match max_str.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument must be a number");
                exit(1);
            },
        }
    };
    if max.eq(&0) {
        println!("Argument must be <0");
        exit(2);
    }
    let mut each_time = Vec::new();
    for _ in 0..max {
        let this_start = Instant::now();
        uuidv4();
        let this_end = Instant::now();
        each_time.push(this_end - this_start);
    }
    let tot_ns = each_time.iter().map(|e| e.as_nanos())
        .sum::<u128>();
    let tot_ms = tot_ns / 1000000;
    let tot_ms_str = match tot_ms {
        0 => ">1".to_owned(),
        n => n.to_string(),
    };
    let avg_ns = tot_ns / max as u128;
    let avg_ms = avg_ns / 1000000;
    let avg_ms_str = match avg_ms {
        0 => ">1".to_owned(),
        n => n.to_string(),
    };
    println!(
        "{} uuid(s) generated in {}ms ({}ns)",
        max, tot_ms_str, tot_ns
    );
    println!(
        "Average of {}ms ({}ns) per uuid",
        avg_ms_str, avg_ns
    );
}
