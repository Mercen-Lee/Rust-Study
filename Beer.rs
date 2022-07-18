fn main() {
  beersong(99);
}

fn nobeer() {
  println!("Take one down and pass it around, no more bottles of beer on the wall.\n\n\
    No more bottles of beer on the wall, no more bottles of beer.\n\
    Go to the store and buy some more, 99 bottles of beer on the wall.");
}

fn beer(a: i32, b: String) {
  println!("{0} bottle{1} of beer on the wall, {0} bottle{1} of beer.", a, b);
}

fn takebeer(a: i32, b: String) {
  println!("Take one down and pass it around, {} bottle{} of beer on the wall.\n", a, b);
}

fn beersong(beers: i32) {
  if beers > 0 {
    beer(beers, calculate(beers));
    if beers == 1 { nobeer(); } else {
      takebeer(beers-1, calculate(beers-1));
      beersong(beers-1);
    }
  }
}

fn calculate(count: i32) -> String { (if count == 1 {""} else {"s"}).to_string() }
