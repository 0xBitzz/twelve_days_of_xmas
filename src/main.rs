const LYRICS_ARR: [(&str, &str); 12] = [
  ("first", "And a partridge in a pear tree."),
  ("second", "Two turtle doves,"),
  ("third", "Three French hens,"),
  ("fourth", "Four calling birds,"),
  ("fifth", "Five golden rings,"),
  ("sixth", "Six geese a-laying,"),
  ("seventh", "Seven swans a-swimming,"),
  ("eighth", "Eight maids a-milking,"),
  ("nineth", "Nine ladies dancing,"),
  ("tenth", "Ten lords a-leaping,"),
  ("eleventh", "Eleven pipers piping,"),
  ("twelfth", "Twelve drummers drumming,"),
];

fn main() {
    let mut counter = 0;

    while counter < 12 {
      let (day, mut gift) = LYRICS_ARR[counter];
      if counter == 0 {
        gift = "A patridge in a pear tree.";
      }
      println!("On the {day} day of Christmas, \nmy true love gave to me \n{gift}");

      let mut second_counter = counter;
      while second_counter > 0 {
        let mut gift = LYRICS_ARR[second_counter - 1].1;
        if counter == 11 && second_counter == 1 {
          gift = "And a patridge in a pear tree!";
        }
        println!("{gift}");
        second_counter -= 1;
      }

      counter += 1;
    }
}
