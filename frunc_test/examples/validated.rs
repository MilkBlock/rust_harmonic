use frunk::{hlist_pat, prelude::*}; // for Result::into_validated

#[derive(PartialEq, Eq, Debug)]
struct Person {
    age: i32,
    name: String,
    street: String,
}

fn get_name() -> Result<String, Error> { /* elided */ }
fn get_age() -> Result<i32, Error> { /* elided */ }
fn get_street() -> Result<String, Error> { /* elided */ }

fn main(){
    // Build up a `Validated` by adding in any number of `Result`s
    let validation = get_name().into_validated() + get_age() + get_street();
    // When needed, turn the `Validated` back into a Result and map as usual
    let try_person = validation.into_result()
                            // Destructure our hlist
                            .map(|hlist_pat!(name, age, street)| {
                                Person {
                                    name: name,
                                    age: age,
                                    street: street,
                                }
                            });

    assert_eq!(try_person.unwrap(), Person { name: "James".to_owned(), age: 32, street: "Main".to_owned(), });
}