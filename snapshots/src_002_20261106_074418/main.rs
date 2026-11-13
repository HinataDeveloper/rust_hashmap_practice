use hashmap_practice::Person;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    println!("\n");

    let raphael: Person = Person::default();
    let samuel: Person = Person::default();

    let mut group: HashMap<u64, Person> = HashMap::new();

    match group.entry(raphael.person_id()) {
        Entry::Vacant(v) => {
            v.insert(raphael);
            println!("Saved ...");
        }
        Entry::Occupied(_) => {
            println!("This is duplicate");
        }
    }

    match group.entry(samuel.person_id()) {
        Entry::Vacant(v) => {
            v.insert(samuel);
            println!("Saved ...");
        }
        Entry::Occupied(_) => {
            println!("This is duplicate");
        }
    }

    println!("\nThe End ...\n");
}
