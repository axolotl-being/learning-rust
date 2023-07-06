//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
   let christmas =[
    ["first","A partridge in a pear tree"]
   ,["second","Two turtle doves, and"]
   ,["third","Three french hens"]
   ,["fourth","Four calling birds"]
   ,["fifth","Five golden rings"]
   ,["sixth","Six geese a-laying"]
   ,["seventh","Seven swans a-swimming"]
   ,["eighth","Eight maids a-milking"]
   ,["ninth","Nine ladies dancing"]
   ,["tenth","Ten lords a-leaping"]
   ,["eleventh","Eleven pipers piping"]
   ,["twelth","Twelve drummers drumming"]
];

    for number  in 0..christmas.len() {

        println!("On the {} day of Christmas,", christmas[number][0]);
        println!("my true love sent to me");

        for gifts in (0..number+1).rev() {
            println!("{}", christmas[gifts][1]);
        };
        println!("");
    }
}
