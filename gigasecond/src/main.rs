use time::{date, time, Duration, PrimitiveDateTime as DateTime};

fn main() {
    let birthdate = DateTime::new(date!(2015 - 01 - 24), time!(22:00:00));
    let date_with_gigasecond_added = after(birthdate);
    println!("{}", date_with_gigasecond_added);
}

fn after(start: DateTime) -> DateTime {
    // A gigasecond is one thousand million seconds.
    // That is a one with nine zeros after it.
    let gigasecond = Duration::seconds(1_000_000_000);

    let result_datetime = start + gigasecond;

    result_datetime
}
