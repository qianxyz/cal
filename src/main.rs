use carender::Calendar;

fn main() {
    let cal = Calendar::new(2022, 1, 1, 12, false, 0, 3).unwrap();
    println!("{}", cal);
}
