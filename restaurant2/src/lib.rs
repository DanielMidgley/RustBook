mod front_of_house; // this mod line only needs to be stated once in the project, and the module will be accessible everywhere within the project

pub use crate::front::of::house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}