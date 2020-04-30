pub mod client;
pub mod documents;
pub mod errors;
pub mod indexes;
pub mod progress;
mod request;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
