use hashmap_practice::Person;
use std::collections::HashMap;

fn main() {
    println!("\n");

    let raphael_one: Person = Person::default();
    let raphael_two: Person = Person::default();

    let mut map: HashMap<u64, Person> = HashMap::new();

    let result_one = map.insert(raphael_one.person_id(), raphael_one);
    match result_one {
        Some(prn) => println!("This Person is duplicate: {prn}"),
        None => println!("Insert to the HashMap successfully ..."),
    }

    let result_two = map.insert(raphael_two.person_id(), raphael_two);
    match result_two {
        Some(prn) => println!("This Person is duplicate: {prn}"),
        None => println!("Insert to the HashMap successfully ..."),
    }

    println!("\nThe End ...\n");
}
