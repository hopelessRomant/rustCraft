// refcell provide a way to check for ownership rules during the run time therefore
// allowing mutability to immutable references.

use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, note: &str);
}

#[derive(Debug)]
struct Mockmessenger {
    output: RefCell<Vec<String>>
}

impl Messenger for Mockmessenger {
    fn send(&self, note: &str) {
        // self.output.push(note.to_string()); // we don't want ot make this ref mutable for API security
        self.output.borrow_mut().push(note.to_string());
    }
}

#[derive(Debug)]
struct Tracker<'a, T: Messenger> {
    messege: &'a T, // implimenting type constraint for messege value.
    value: usize,
    max: usize
}

impl<'a, T> Tracker<'a, T>
where
    T: Messenger
{
    pub fn new (messege: &'a T, max: usize) -> Tracker<'a, T> {
        Tracker{
            messege,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let fraction = self.value as f32 / self.max as f32;

        if fraction >=1.0 {
            self.messege.send("you have exhausted your API quota");
        } else if 1.0 >= fraction && fraction >= 0.75 {
            self.messege.send("you have used up over 75 percent API calls");
        }  else {
            self.messege.send("safe range (")
        }
    }
}

pub fn multi_mut_ref() {
    let mock = Mockmessenger{
        output: RefCell::new(vec![])
    };

    let mut ref1 = mock.output.borrow_mut();
    let mut ref2 = mock.output.borrow_mut();

    ref1.push("hello".to_string());
    ref2.push("world".to_string());

    // This function compiles but panics in runtime
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mock= Mockmessenger {
            output: RefCell::new(vec![])
        };
        let mut tracker = Tracker::new(&mock, 100);

        tracker.set_value(78);
        assert_eq!(mock.output.borrow()[0], "you have used up over 75 percent API calls");
    }
}
