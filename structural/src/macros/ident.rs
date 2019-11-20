

/// Constructs field path(s) (TString/TStringSet).
///
/// The arguments to this macro can be a single or multiple identifiers.
///
/// When passed a single argument,this instantiates a `FieldPath`,
/// which can be passed to the
/// `GetFieldExt::{field_,field_mut,into_field,box_into_field}` methods
/// to access a field.
///
/// When passed multiple arguments,this instantiates a `FieldPathSet`.
/// It can then be passed to the `GetFieldExt::fields` method,
/// To be passed to `GetFieldExt::fields_mut`,
/// `FieldPathSet` must  constructed with syntactically unique paths,
/// since there is no cheap way to check for equality of type-level strings yet.
///
/// # Syntax 
///
/// ### Splicing
///
/// You can use a `FieldPath` type (not a value) 
/// inside the `fp` macro with the `( FooType )` syntax.
/// 
/// This will splice the `FieldPath` into the position it was used in.
/// 
/// An example:
/// ```
/// use structural::{fp,FP,field_path_aliases};
/// use structural::reexports::AssertEq;
/// 
/// field_path_aliases!{
///     wooo,
///     chain=b.c.d,
///     get_x=pos.x,
/// }
///
/// # fn main(){
///
/// AssertEq::new( fp!( a.(wooo).e ) , fp!(a.wooo.e) );
/// 
/// AssertEq::new( fp!( a.(get_x).e ), fp!(a.pos.x.e) );
///
/// # }
///
/// ```
/// 
/// ### Inserting
///
/// You can use a `TString` type or a single-ident `FieldPath` type
/// inside the `fp` macro with the `[ FooType ]` syntax.
/// 
/// This inserts the value of the `TString`or of the single identifier `FieldPath`
/// into that position.
/// 
/// An example:
/// ```
/// use structural::{fp,FP,field_path_aliases};
/// use structural::reexports::AssertEq;
/// 
/// field_path_aliases!{
///     foo,
///     bar=what,
///     baz=the,
/// }
///
/// // This can also be `type RectangleStr=FP!(rectangle);` from Rust 1.40 onwards
/// type RectangleStr=FP!(r e c t a n g l e);
///
///
/// # fn main(){
/// let _:foo;
/// let _:bar;
/// let _:baz;
///
/// AssertEq::new( fp!( a[foo].e ), fp!(a.foo.e) );
/// AssertEq::new( fp!( a[bar].e ), fp!(a.what.e) );
/// AssertEq::new( fp!( a[baz].e ), fp!(a.the.e) );
/// AssertEq::new( fp!( a[RectangleStr].e ), fp!(a.rectangle.e) );
///
/// # }
///
/// ```
/// 
///
///
/// 
/// # Example
///
/// ```
/// use structural::{GetFieldExt,fp};
///
/// {
///     let tup=("I","you","they");
///    
///     assert_eq!( tup.field_(fp!(0)), &"I" );
///     assert_eq!( tup.field_(fp!(1)), &"you" );
///     assert_eq!( tup.field_(fp!(2)), &"they" );
///    
///     assert_eq!( tup.fields(fp!(0,1)), (&"I",&"you") );
///    
///     assert_eq!( tup.fields(fp!(0,1,2)), (&"I",&"you",&"they") );
/// }
///
/// #[derive(structural::Structural)]
/// #[struc(public)]
/// #[struc(access="mut move")]
/// struct Foo{
///     bar:u32,
///     baz:u32,
///     ooo:(u32,u32),
/// }
///
/// {
///     let mut foo=Foo{
///         bar:0,
///         baz:44,
///         ooo:(66,99),
///     };
///     
///     assert_eq!( foo.field_(fp!(bar)), &0 );
///     assert_eq!( foo.field_(fp!(baz)), &44 );
///     assert_eq!( foo.field_(fp!(ooo)), &(66,99) );
///     assert_eq!( foo.field_(fp!(ooo.0)), &66 );
///     assert_eq!( foo.field_(fp!(ooo.1)), &99 );
///
///     assert_eq!( foo.fields(fp!(ooo,bar)), (&(66,99),&0) );
///     assert_eq!( foo.fields(fp!(bar,ooo,baz)), (&0,&(66,99),&44) );
///
///     assert_eq!( foo.fields_mut(fp!(ooo,bar)), (&mut (66,99), &mut 0) );
///     assert_eq!( foo.fields_mut(fp!(bar,ooo,baz)), (&mut 0, &mut (66,99), &mut 44) );
///         
/// }
///
/// ```
///
#[macro_export]
macro_rules! fp {
    ( $($strings:tt)* ) => {{
        $crate::_delegate_fp!{$($strings)*}
    }};
}


#[macro_export]
#[doc(hidden)]
//#[cfg(not(feature="better_macros"))]
macro_rules! _delegate_fp {
    ($($everything:tt)*) => ({
        #[allow(unused_imports)]
        mod dummy{
            use super::*;
            $crate::old_fp_impl_!{$($everything)*}
        }
        dummy::VALUE
    })
}

// #[macro_export]
// #[doc(hidden)]
// #[cfg(feature="better_macros")]
// macro_rules! _delegate_fp {
//     ($($everything:tt)*) => (
//         let $crate::new_fp_impl_!($($everything)*)
//     )
// }




/// Constructs a type-level identifier for use as a generic parameter.
///
/// # Improved macro
///
/// To get an improved version of this macro (it requires Rust nightly or Rust 1.40) 
/// which can use the same syntax as the `fp` macro,
/// you can use either the `nightly_better_macros` or `better_macros` cargo features.
///
///
/// # Examples
///
/// This demonstrates how one can bound types by the accessor traits in a where clause.
///
/// ```rust
/// use structural::{GetField,GetFieldExt,fp,FP};
///
/// fn greet_entity<This,S>(entity:&This)
/// where
///     This:GetField<FP!(n a m e),Ty=S>,
///     S:AsRef<str>,
/// {
///     println!("Hello, {}!",entity.field_(fp!(name)).as_ref() );
/// }
///
/// ```
/// 
/// # Example
/// 
/// This demonstrates the improved version of this macro,which requires either the 
/// the `nightly_better_macros` or `better_macros` cargo features.
/// Once proc-macros in types reaches stable this will be usable automatically
/// for Rust versions since.
/// 
#[cfg_attr(feature="better_macros",doc=" ```rust")]
#[cfg_attr(not(feature="better_macros"),doc=" ```ignore")]
/// use structural::{GetField,GetFieldExt,fp,FP};
///
/// fn greet_entity<This,S>(entity:&This)
/// where
///     This:GetField<FP!(name),Ty=S>,
///     S:AsRef<str>,
/// {
///     println!("Hello, {}!",entity.field_(fp!(name)).as_ref() );
/// }
///
/// type NumericIdent=FP!(0);
/// type StringyIdent=FP!(huh);
///
/// ```
///
#[macro_export]
macro_rules! FP {
    ($($char:tt)*) => {
        $crate::_delegate_FP!($($char)*)
    };
}


#[macro_export]
#[doc(hidden)]
#[cfg(not(feature="better_macros"))]
macro_rules! _delegate_FP {
    ($($char:tt)*) => (
        $crate::pmr::FieldPath<(
            $crate::pmr::TString<($($crate::TChar!($char),)*)>,
        )>
    )
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature="better_macros")]
macro_rules! _delegate_FP {
    ($($everything:tt)*) => (
        $crate::_FP_impl_!($($everything)*)
    )
}


/*

Code to generate the non-default branches

fn main() {
    for b in 0..=255u8 {
        let c=b as char;
        if c.is_alphanumeric() && b<128 || c=='_' {
            println!("({})=>( $crate::chars::_{} );",c,c)
        }
    }
}

*/

#[doc(hidden)]
#[macro_export]
macro_rules! TChar{
    (0)=>( $crate::chars::_0 );
    (1)=>( $crate::chars::_1 );
    (2)=>( $crate::chars::_2 );
    (3)=>( $crate::chars::_3 );
    (4)=>( $crate::chars::_4 );
    (5)=>( $crate::chars::_5 );
    (6)=>( $crate::chars::_6 );
    (7)=>( $crate::chars::_7 );
    (8)=>( $crate::chars::_8 );
    (9)=>( $crate::chars::_9 );
    (A)=>( $crate::chars::_A );
    (B)=>( $crate::chars::_B );
    (C)=>( $crate::chars::_C );
    (D)=>( $crate::chars::_D );
    (E)=>( $crate::chars::_E );
    (F)=>( $crate::chars::_F );
    (G)=>( $crate::chars::_G );
    (H)=>( $crate::chars::_H );
    (I)=>( $crate::chars::_I );
    (J)=>( $crate::chars::_J );
    (K)=>( $crate::chars::_K );
    (L)=>( $crate::chars::_L );
    (M)=>( $crate::chars::_M );
    (N)=>( $crate::chars::_N );
    (O)=>( $crate::chars::_O );
    (P)=>( $crate::chars::_P );
    (Q)=>( $crate::chars::_Q );
    (R)=>( $crate::chars::_R );
    (S)=>( $crate::chars::_S );
    (T)=>( $crate::chars::_T );
    (U)=>( $crate::chars::_U );
    (V)=>( $crate::chars::_V );
    (W)=>( $crate::chars::_W );
    (X)=>( $crate::chars::_X );
    (Y)=>( $crate::chars::_Y );
    (Z)=>( $crate::chars::_Z );
    (_)=>( $crate::chars::__ );
    (a)=>( $crate::chars::_a );
    (b)=>( $crate::chars::_b );
    (c)=>( $crate::chars::_c );
    (d)=>( $crate::chars::_d );
    (e)=>( $crate::chars::_e );
    (f)=>( $crate::chars::_f );
    (g)=>( $crate::chars::_g );
    (h)=>( $crate::chars::_h );
    (i)=>( $crate::chars::_i );
    (j)=>( $crate::chars::_j );
    (k)=>( $crate::chars::_k );
    (l)=>( $crate::chars::_l );
    (m)=>( $crate::chars::_m );
    (n)=>( $crate::chars::_n );
    (o)=>( $crate::chars::_o );
    (p)=>( $crate::chars::_p );
    (q)=>( $crate::chars::_q );
    (r)=>( $crate::chars::_r );
    (s)=>( $crate::chars::_s );
    (t)=>( $crate::chars::_t );
    (u)=>( $crate::chars::_u );
    (v)=>( $crate::chars::_v );
    (w)=>( $crate::chars::_w );
    (x)=>( $crate::chars::_x );
    (y)=>( $crate::chars::_y );
    (z)=>( $crate::chars::_z );
    ($byte:ident)=>{
        $crate::chars::$byte
    }
} 




////////////////////////////////////////////////////////////////////////////////


/**

Declares a module with aliases for type-level idents,used to access fields.

Every one of these aliases are types and constants of the same name.

These aliases *cannot* be combined to pass to
`GetFieldExt::fields` or `GetFieldExt::fields_mut`.
When macros take in identifiers those must be the
literal identifiers for the fields(you can't pass aliases),
since they must check that the field names passed to the macro don't repeat
within the macro invocation.

# Example

```rust
use structural::{field_path_aliases_module,GetField,GetFieldExt};

field_path_aliases_module!{
    pub mod names{
        // Equivalent to _a=_a
        _a,
        // Equivalent to _b=_b
        _b,
        // Equivalent to _0=_0
        _0,
        // Equivalent to c=c
        c,
        zero=0,
        one=1,
        two=2,
        e=10,
        g=abcd,

        // Used to access the `0`,`1`,and `2` fields
        // with the fields or fields_mut method.
        FirstThree=(0,1,2),
        h=(a,b,c),
        i=(0,3,5),

        j=(p), // The identifier can also be parenthesised

    }
}


fn assert_fields<T>(this:&T)
where
    T:GetField<names::zero,Ty=i32>+
        GetField<names::one,Ty=i32>+
        GetField<names::two,Ty=i32>
{
    assert_eq!( this.field_(names::zero), &2 );
    assert_eq!( this.field_(names::one), &3 );
    assert_eq!( this.field_(names::two), &5 );
    assert_eq!( this.fields(names::FirstThree), (&2,&3,&5) );
}

fn main(){
    assert_fields(&(2,3,5));
    assert_fields(&(2,3,5,8));
    assert_fields(&(2,3,5,8,13));
    assert_fields(&(2,3,5,8,13,21));
}


```


*/
#[macro_export]
macro_rules! field_path_aliases_module {
    (
        $(#[$attr:meta])*
        $vis:vis mod $mod_name:ident{
            $($mod_contents:tt)*
        }
    ) => (
        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        #[allow(unused_imports)]
        $(#[$attr])*
        $vis mod $mod_name{
            use super::*;
            $crate::declare_name_aliases!{
                $($mod_contents)*
            }
        }
    );
}


/**

Declares aliases for type-level idents,used to access fields.

Every one of these aliases are types and constants of the same name.

TODO:document whether this will work on 1.40

# Example

```rust
use structural::{field_path_aliases,GetField,GetFieldExt};

field_path_aliases!{
    // Equivalent to hello=hello
    hello,

    // Equivalent to world=world
    world,

    zero=0,
    one=1,
    two=2,

    // Used to access the `0`,`1`,and `2` fields
    // with the fields or fields_mut method.
    FirstThree=(0,1,2),

    h=(a,b,c),

    j=(p), // The identifier can also be parenthesised
}


fn assert_fields<T>(this:&T)
where
    T:GetField<zero,Ty=i32>+
        GetField<one,Ty=i32>+
        GetField<two,Ty=i32>
{
    assert_eq!( this.field_(zero), &2 );
    assert_eq!( this.field_(one), &3 );
    assert_eq!( this.field_(two), &5 );
    assert_eq!( this.fields(FirstThree), (&2,&3,&5) );
}

fn main(){
    assert_fields(&(2,3,5));
    assert_fields(&(2,3,5,8));
    assert_fields(&(2,3,5,8,13));
    assert_fields(&(2,3,5,8,13,21));
}

```


*/
#[macro_export]
macro_rules! field_path_aliases {
    (
        $($mod_contents:tt)*
    ) => (
        $crate::declare_name_aliases!{
            $($mod_contents)*
        }
    );
}


