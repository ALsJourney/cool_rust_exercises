use time::{Date, Time};
use time::{Duration, PrimitiveDateTime as DateTime};
fn main() {
    let test = DateTime::new(
        Date::from_calendar_date(2000, 12.try_into().unwrap(), 28).unwrap(),
        Time::from_hms(9, 55, 14).unwrap(),
    );

    println!("{}", after(test));
}

fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::seconds(1_000_000_000);
    //println!("{}", start + gigasecond);
    start + gigasecond
}
