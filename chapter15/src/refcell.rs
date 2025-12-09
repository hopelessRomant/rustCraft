// refcell provide a way to check for ownership rules during the run time therefore
// allowing mutability to immutable references.

pub trait Messenger {
    fn send(&self, note: &str);
}

#[derive(Debug)]
pub struct Tracker<'a, T: Messenger> {
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
            self.messege.send("you have exhausted your quota");
        } else if fraction >= 0.75 {
            self.messege.send("you have over 75 percent");
        }  else {
            self.messege.send("safe range")
        }
    }
}
