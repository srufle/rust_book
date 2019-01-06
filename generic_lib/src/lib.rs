
pub mod outermost {
    pub fn middle_function() {
        println!("middle function");
        middle_secret_function();
    }
    fn middle_secret_function() {
        println!("middle secret function");
    }

    pub mod inside {
        pub fn inner_function() {
            println!("inner function");
            secret_function();
        }
        fn secret_function() {
            println!("secret function");
        }
    }
}

pub fn try_me() {
    outermost::middle_function();
    //outermost::middle_secret_function();
    outermost::inside::inner_function();
    //outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    //use crate::try_me;
    use super::outermost;

    #[test]
    fn it_works() {
        //try_me();
        // To run test and see println output
        // https://stackoverflow.com/questions/25106554/why-doesnt-println-work-in-rust-unit-tests
        // cargo test -- --nocapture
        outermost::middle_function();
    }
}
