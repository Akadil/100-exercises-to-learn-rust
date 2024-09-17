use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

/*
    trait IntoIterator {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;
 
        fn into_iter(self) -> Self::IntoIter;
    }
*/
impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Self::Item>; // builtin struct for IntoIter

    fn into_iter(self) -> Self::IntoIter {
        /*
            self.tickets.into_iter() returns an IntoIter struct
        
            pub struct IntoIter<T> {
                buf: ManuallyDrop<Vec<T>>,  // Holds the vector's memory buffer
                iter: *const T,             // Pointer to the current element
                end: *const T,              // Pointer to one past the last element
            }

            impl<T> IntoIterator for Vec<T> {
                type Item = T;
                type IntoIter = std::vec::IntoIter<T>;

                fn into_iter(self) -> Self::IntoIter {
                    // Consumes `self` and returns an iterator
                    IntoIter {
                        buf: ManuallyDrop::new(self),
                        iter: self.as_ptr(),
                        end: unsafe { self.as_ptr().add(self.len()) },
                    }
                }
            }
        */
        self.tickets.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        /*
            collect() transforms an iterator into a collection,
        */
        let tickets: Vec<_> = store.clone().into_iter().collect(); 
        assert_eq!(tickets, store.tickets);
    }
}
