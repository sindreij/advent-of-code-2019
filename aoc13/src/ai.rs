pub struct Ai {}

impl Ai {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_next(&self, (paddle, _): (i16, i16), (ball, _): (i16, i16)) -> i8 {
        if paddle < ball {
            1
        } else if paddle > ball {
            -1
        } else {
            0
        }
    }
}
