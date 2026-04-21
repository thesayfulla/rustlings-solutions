// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    pub use self::fruits::PEAR as fruits;
    pub use self::veggies::CUCUMBER as veggies;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits,
        delicious_snacks::veggies,
    );
}
