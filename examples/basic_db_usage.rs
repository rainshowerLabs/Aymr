use aymr::aymr_db::{
    db,
    traits::AymrDatabase,
};
use zerocopy::AsBytes;

fn main() {
    // One can open a new Aymr DB by calling the open fn.
    //
    // Keep in mind that the way of declaring the config changes
    // with each database.
    let mut aymr = db::AymrDb::open();

    // Insert inserts a value into our database with a key.
    // It returns the previous value at that key if it existed.
    let _ = aymr.insert(123.as_bytes(), 456.as_bytes());

    // We can get our value by using `get` and a key.
    let our_value = aymr.get(&123.as_bytes()).unwrap().unwrap();
    let our_value: i32 = i32::from_le_bytes(our_value.as_slice().try_into().unwrap());
    println!("Hello from aymr! The value at key `123` is {}", our_value);

    // If we want to remove a value, just call remove!
    let old_value = aymr.remove(&123.as_bytes()).unwrap().unwrap();
    let old_value: i32 = i32::from_le_bytes(old_value.as_slice().try_into().unwrap());
    let removed_value = aymr.get(&123.as_bytes()).unwrap();

    println!(
        "We got bored of the value {} at key `123`. The value at key `123` is now {:?}",
        old_value, removed_value
    );
}
