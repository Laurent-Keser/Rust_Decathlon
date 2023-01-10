//importing the io lib
//use std::io;

fn main() {
    //creating a vector (list) of somme cities. If no datas a providen you can : let v: Vec<i32> = Vec::new();
    let list_cities = vec![
        "Brussels",
        "Mons",
        "Liège",
        "Eupen",
        "Namur",
        "Braine-le-Comte",
        "Soignies",
        "Charleroi",
        "Dinant",
        "Nivelles",
    ];

    println!("My favourite cities are : ");

    //for each element in the vector, print the value
    for i in &list_cities {
        println!("{i}");
    }
}
