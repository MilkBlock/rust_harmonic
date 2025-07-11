use impl_trait_for_tuples::impl_for_tuples;

trait M {
    type Ret;
    type Arg;
    type FixedType;
    const VALUE: u32;

    fn test(arg: Self::Arg) -> Self::Ret;

    fn test_with_self(&self) -> Result<(), ()>;
}
#[impl_for_tuples(1, 2)]
impl M for Tuple {
    // Here we expand the `Ret` and `Arg` associated types.
    for_tuples!( type Ret = ( #( Tuple::Ret ),* ); );
    for_tuples!( type Arg = ( #( Tuple::Arg ),* ); );
    for_tuples!( const VALUE: u32 = #( Tuple::VALUE )+*; );

    // Here we set the `FixedType` to `u32` and add a custom where bound that forces the same
    // `FixedType` for all tuple types.
    type FixedType = u32;
    for_tuples!( where #( Tuple: M<FixedType=u32> )* );

    fn test(arg: Self::Arg) -> Self::Ret {
        for_tuples!( ( #( Tuple::test(arg.Tuple) ),* ) )
    }

    fn test_with_self(&self) -> Result<(), ()> {
        for_tuples!( #( Tuple.test_with_self()?; )* );
        Ok(())
    }
}
fn main(){

}
