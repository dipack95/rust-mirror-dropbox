struct Person {
    age: u32
}

impl std::convert::From<u32> for Person {
    fn from(age: u32) -> Self {
        Person { age: age }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This person's age is: {0}", self.age)
    }
}

fn main() {
    let age: u32 = 50;
    let person_one: Person = Person::from(32);
    let person_two: Person = age.into();
    println!("{}", person_one);
    println!("{}", person_two);
}