/**
 * a struct to manage the ticket ids
 */
#[derive(Clone)]
pub struct IdManager {
    pub next_id: u64,
}

impl IdManager {
    pub fn new() -> IdManager {
        IdManager {
            next_id: 0,
        }
    }

    pub fn get_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}