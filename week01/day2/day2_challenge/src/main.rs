const MINUTES_PER_SESSION: u32 = 60;
const SESSIONS_PER_WEEK: u32 = 6;

fn main() {
    // println!("Hello, world!");
    let mut total_minutes = 0;
    for week in 1..=22 {
        total_minutes += MINUTES_PER_SESSION * SESSIONS_PER_WEEK;

        if week % 5 == 0 {
            println!("After week {week} : {total_minutes}");
        }
    }

    let total = total_minutes / 60;
    let total = format!("{total} hours");
    println!("Total study {total}")
}
