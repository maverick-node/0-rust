use std::rc::Rc;
use core::cell::RefCell;
pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: i64,
    pub max: i64,
}
impl Tracker {
    pub fn new(max: i64) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: 0,
            max: max,
        }
    }
    pub fn set_value(&self, vl: &Rc<i64>) {
        let count = Rc::strong_count(vl) as i64;
        if count > self.max {
            self.messages.borrow_mut().push(format!("Error: You can't go over your quota!"));
        } else {
            let mut cc = (count * 100) / self.max;
            if cc > 70 {
                self.messages
                    .borrow_mut()
                    .push(format!("Warning: You have used up over {}% of your quota!", cc));
            }
        }
    }
    pub fn peek(&self, rc: &Rc<i64>) {
        let count = Rc::strong_count(rc) as i64;
        let mut cc = (count * 100) / self.max;
        self.messages
            .borrow_mut()
            .push(format!("Info: This value would use {}% of your quota", cc));
    }
}
