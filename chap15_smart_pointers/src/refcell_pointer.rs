pub trait Messanger{
    fn send(&self, message:&str);
}

pub struct LimitTracker<'a, T>
where T: Messanger,
{
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where T:Messanger,
{
    pub fn new(messanger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messanger,
            max,
            value: 0
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messanger.send("Err: You are out of you quota");
        } else if percentage_of_max >= 0.9 {
            self.messanger.send("Warn: You have used 90% of your quota");
        } else if percentage_of_max >= 0.75{
            self.messanger.send("Warn: You have used 75% of your quota");
        }
    }
}