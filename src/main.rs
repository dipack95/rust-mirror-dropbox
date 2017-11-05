use List::*;

enum List {
    Node(u32, Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, element: u32) -> List {
        Node(element, Box::new(self))
    }
    fn length(&self) -> u32 {
        match *self {
            Node(_, ref tail) => tail.length() + 1,
            Nil => 0
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);

    println!("{:?}", list.length());
}