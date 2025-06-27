use std::cmp::Ord;
pub trait TimeStamp : Ord {}


pub trait TimeStampProvider where Self::TimeStamp:TimeStamp {
    type TimeStamp;
    fn now(&mut self) -> Self::TimeStamp;
}



#[derive(PartialEq,
         Eq,
         PartialOrd,
         Ord,
         Clone,
         derive_more::From,
         derive_more::Into)]
pub struct DiscreteTimeStamp(usize);

impl DiscreteTimeStamp {
    fn increment(self) -> Self {
        (<usize>::from(self)+1).into()
    }
}


impl TimeStamp for DiscreteTimeStamp {}


pub struct DiscreteTimeStampProvider {
    last_time_stamp_provided:Option<DiscreteTimeStamp>
}

impl DiscreteTimeStampProvider {
    pub fn new() -> Self {
        Self{last_time_stamp_provided:None}
    }
}

impl TimeStampProvider for DiscreteTimeStampProvider {
    type TimeStamp = DiscreteTimeStamp;
    fn now(&mut self) -> DiscreteTimeStamp {
        let now=match &self.last_time_stamp_provided {
            Some(last_ts) => last_ts.clone().increment(),
            None => 0.into()
        };
        self.last_time_stamp_provided=Some(now.clone());
        now
    }
}