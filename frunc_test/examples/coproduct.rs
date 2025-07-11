use frunk::{hlist, Coprod, Coproduct};
fn main() {
    type I32F32 = Coprod!(i32, f32);
    // Constructing coproducts using inject:
    let co1_nice:I32F32= Coproduct::inject(1i32);
    let co2_nice:I32F32= Coproduct::inject(42f32);
    let m = hlist!(1,1.0);
    


    // // Compare this to the "hard way":
    // let co1_ugly: I32F32 = Coproduct::Inl(1i32);
    // let co2_ugly: I32F32 = Coproduct::Inr(Coproduct::Inl(42f32));

    // assert_eq!(co1_nice, co1_ugly);
    // assert_eq!(co2_nice, co2_ugly);

    // Feel free to use `inject` on a type alias, or even directly on the
    // `Coprod!` macro. (the latter requires wrapping the type in `<>`)
    let _ = I32F32::inject(42f32);

    // You can also use a turbofish to specify the type of the input when
    // it is ambiguous (e.g. an empty `vec![]`).
    // The Index parameter should be left as `_`.
    type Vi32Vf32 = Coprod!(Vec<i32>, Vec<f32>);
    let _: Vi32Vf32 = Coproduct::inject::<Vec<i32>, _>(vec![]);
}