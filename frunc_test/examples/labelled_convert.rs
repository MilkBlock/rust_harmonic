// Suppose that again, we have different User types representing the same data
// in different stages in our application logic.
use frunk::{field, LabelledGeneric, labelled::{Field}};
use frunk::{labelled_convert_from, Coprod, Coproduct, };

#[derive(LabelledGeneric)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}
fn main(){
    let n_user = NewUser {
        first_name: "Joe",
        last_name: "Blow",
        age: 30
    };

    // Convert from a NewUser to a Saved using LabelledGeneric
    //
    // This will fail if the fields of the types converted to and from do not
    // have the same names or do not line up properly :)
    //
        // Also note that we're using a helper method to avoid having to use universal
    // function call syntax
    let s_user: SavedUser = labelled_convert_from(n_user);
    <s_user as LabelledGeneric>

    assert_eq!(s_user.first_name, "Joe");
    assert_eq!(s_user.last_name, "Blow");
    assert_eq!(s_user.age, 30);

    // Uh-oh ! last_name and first_name have been flipped!
    #[derive(LabelledGeneric)]
    struct DeletedUser<'a> {
        first_name: &'a str,
        last_name: &'a str,
        age: usize,
    }


    let a = DeletedUser as 
    //  This would fail at compile time :)
    let d_user: DeletedUser = labelled_convert_from(s_user);

    // This will, however, work, because we make use of the Sculptor type-class
    // to type-safely reshape the representations to align/match each other.
    let d_user: DeletedUser = frunk::transform_from(s_user);


    type T = Coprod!(i32,bool,u32);
}