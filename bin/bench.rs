use {
    std::{
        env::args,
        process::exit,
        time::{ Duration, Instant },
    },
    uuidv4_rs::{ UUID, uuidv4, },
};
fn bench<Type>(max: usize, durs: &mut Vec<Duration>)
where
    Type: UUID,
{
    for _ in 0..max {
        let this_start = Instant::now();
        uuidv4::<String>();
        let this_end = Instant::now();
        durs.push(this_end - this_start);
    }
}
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
    let typ = {
        let typ_str = match args().nth(2) {
            Some(arg) => arg,
            None => "string".to_owned(),
        };
        match typ_str.as_str() {
            "string" => "string",
            "u8" => "u8",
            _ => {
                println!("Argument for type must be in [string, u8]");
                exit(2);
            },
        }
    };
    if max.eq(&0) {
        println!("Argument must be <0");
        exit(3);
    }
    let mut each_time = Vec::new();
    match typ {
        "string" => {
            bench::<String>(max, &mut each_time);
        },
        "u8" => {
            bench::<[u8; 16]>(max, &mut each_time);
        },
        t => {
            println!("{} is not a type implementing UUID", t);
            exit(4);
        },
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
