/**
 * @analyze     Understand the nature of iterators
 * 
 * 
 * @Results:
 *      1. There are Iterator struct (std::slice::Iter<'a, T>), Iterator trait, 
 *          and iter() method.
 * 
 *          The iter() method is mostly declared in popular structs who wants to
 *          to make the container iterable. As I understood, this method creates
 *          an Iterator struct and returns it (but we don't see, because it is 
 *          mostly given to the for loop directly).
 * 
 *          So the purpose of all 3 of them is to create an iterable object, just
 *          with different approaches.
 * 
 *      2. The same goes with IntoIterator. The difference is that simple
 *          iterators works with references and don't consume the original container. 
 *          While IntoIterator has its own struct in std::vec::IntoIter<T> and
 *          basically consumes the original container. As I understood it copies 
 *          the original container and works with it. It implements the Iterator
 *          trait internally.
 * 
 *      3. For and while loops are using iterators under the hood. In order to be
 *          able to use for loop, the container must implement the Iterator trait. 
 * 
 *      4. We are covering the lifetime section very weirdly here because for 
 *          iterators use lots of references, so we need to practice it
 * 
 * @Questions:
 *      1. If 
 */
pub fn main() {
    println!("Run the basic test");
    test_basic();

    println!("\nRun the test iter");
    test_iter();

    println!("\nRun the test into_iter");
    test_into_iter();
}

/* ************************** Test 1 - basic ******************************** */
fn test_basic() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut iter = IntoIterator::into_iter(v);

    loop {
        match iter.next() {
            Some(n) => {
                println!("{}", n);
            }
            None => {
                println!("done");
                break;
            }
        }
    }
}

/* ************************** Test 2 - iter ******************************** */
/*
    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
*/
fn test_iter() {
    let mut store = TicketStore::new();

    store.add_ticket(1);
    store.add_ticket(2);
    store.add_ticket(3);

    loop {
        match store.next() {
            Some(n) => {
                println!("{}", n);
            }
            None => {
                println!("done");
                break;
            }
        }
    }
}

/**     My local vector(or container)    */
pub struct TicketStore {
    tickets: Vec<i32>,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: i32) {
        self.tickets.push(ticket);
    }
}

impl Iterator for TicketStore {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.tickets.pop()
    }
}

/* ************************** Test - intoIterator ************************** */
/*
    trait IntoIterator {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;

        fn into_iter(self) -> Self::IntoIter;
    }

    The purpose of this function is to create an iterator. It is not a type
    of iterator, it is just a function that creates an iterator.
*/
fn test_into_iter() {
    let mut store = TicketStore::new();

    store.add_ticket(1);
    store.add_ticket(2);
    store.add_ticket(3);

    let mut iter = IntoIterator::into_iter(store);

    loop {
        match iter.next() {
            Some(n) => {
                println!("{}", n);
            }
            None => {
                println!("done");
                break;
            }
        }
    }
}