pub mod solutions;
pub mod active;

// Define a trait that all runnable solutions will implement.
pub trait Runnable {
    fn run();
}
