// This basically makes the modules availible to the entire 
// library
pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        print!("Hello world!");
        // panic!("Shouldn't reach here!");
        // unimplemented!();
    }
}

pub mod test_provided;
// pub mod test_student;