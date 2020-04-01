/*!

This library provides field accessor traits,and emulation of structural types.

# Features

These are some of the features this library provides:

- [`Structural`] derive macro to implement accessor traits for every public field:
[`GetField`]/[`GetFieldMut`]/[`IntoField`] for structs,
and [`GetVariantField`]/[`GetVariantFieldMut`]/[`IntoVariantField`] for enums.

- The [`StructuralExt`] extension trait,which defines the main methods to access fields,
so long as the type implements the accessor traits for those fields.

- The [`StrucWrapper`] wrapper type,defined as an alternative to [`StructuralExt`].

- The [`structural_alias`] macro, to declare trait aliases for accessor traits,
using field-in-trait syntax.

- The [`impl_struct`] macro to declare structural parameter/return types,
as well as [`make_struct`] to construct anonymous structs


# Clarifications

The way that this library emulates structural types is by using traits as bounds
or trait objects.

By default all structural types are open,
structs and enums can have more variants and or fields than are required.

The only exception to this is exhaustive enums,
in which the variant count and names must match exactly,
this is useful for exhaustive matching of variants (in the [`switch`] macro).

### Required macros

The only macros that are required to use this crate are the ones for [`TStr`],
every other macro expands to code that can be written manually.

# Conditional methods

### `*box_*` methods

Every `*Into*Field*` trait has a `*box_*` method that takes a `Box<_>` parameter
which only exists when the "alloc" feature is enabled (it is enabled by default).

If you don't enable the "alloc" feature yourself (it is enabled by default),
you must implement those methods using the macros indicated in the `Features`
section of the documentation for each trait

For an example of how to use those macros,
you can look at the examples in the docs for each of the `*Into*Field*` traits.

<span id="root-mod-examples"></span>
# Examples


### Structural Derive for structs

This demonstrates how you can use any type with a superset of the fields of
another one in a function.

[`Structural`] derive macro docs for more details on derivation.

```rust
use structural::{StructuralExt,Structural,fp};


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive generated the `Point3D_SI` trait for `Point3D`,
    // aliasing the accessor traits for it.
    S:Point3D_SI<u32>
{
    let (a,b,c)=point.fields(fp!( x, y, z ));

    assert_eq!(a,&0);
    assert_eq!(b,&11);
    assert_eq!(c,&33);
}

fn main(){
    reads_point4(&Point3D { x: 0, y: 11, z: 33 });

    reads_point4(&Point4D {
        x: 0,
        y: 11,
        z: 33,
        a: 0xDEAD,
    });

    reads_point4(&Point5D {
        x: 0,
        y: 11,
        z: 33,
        a: 0xDEAD,
        b: 0xBEEF,
    });
}


#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Point3D<T>{
    x:T,
    y:T,
    z:T,
}

#[derive(Structural)]
// By default only public fields get accessor trait impls,
// using `#[struc(public)]` you can have impls to access private fields.
#[struc(public)]
struct Point4D<T>{
    x:T,
    y:T,
    z:T,
    a:T,
}

#[derive(Structural)]
struct Point5D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
    pub a:T,
    pub b:T,
}

```

### Structural Derive for enums

This demonstrates how you can use structural enums.

For details on [enums look here](./docs/enums/index.html).

```rust
use structural::{StructuralExt,Structural,fp,switch};

fn main(){
    {
        // Command

        run_command(Command::SendEmail(SendEmail{
            to:"ferris@lib.rs".to_string(),
            content:"Hello".to_string(),
        }));
        run_command(Command::RemoveAddress("gopher".to_string()));
    }
    {
        // ExtraCommand
        //
        // ExtraCommand can't be passed to `run_command` because that function requires
        // an enum with exactly the `SendEmail` and `RemoveAddress` variants.

        // The `SendEmail` variant can have more fields than the one in the `Command` enum,
        // they're just ignored.
        run_command_nonexhaustive(ExtraCommand::SendEmail{
            to:"squatter@crates.io".to_string(),
            content:"Can you stop squatting crate names?".to_string(),
            topic:"squatting".to_string(),
        }).unwrap();

        let ra_cmd=ExtraCommand::RemoveAddress("smart_person".to_string());
        run_command_nonexhaustive(ra_cmd).unwrap();

        let ca_cmd=ExtraCommand::CreateAddress("honest_person".to_string());
        let res=run_command_nonexhaustive(ca_cmd.clone());
        assert_eq!( res, Err(UnsupportedCommand(ca_cmd)) );
    }
}

// Runs the passed in command.
//
// The `Command_ESI` trait allows only enums with the same variants as
// `Command` to be passed in(they can have a superset of the fields in `Command`).
fn run_command<S>(cmd:S)
where
    S:Command_ESI
{
    run_command_nonexhaustive(cmd)
        .ok()
        .expect("`run_command_nonexhaustive` must match all `Command` variants")
}

// Runs the passed in command.
//
// The `Command_SI` trait allows enums with a superset of the variants in `Command`
// to be passed in,
// requiring the a `_=>` branch when it's matched on with the `switch` macro.
fn run_command_nonexhaustive<S>(cmd:S)->Result<(),UnsupportedCommand<S>>
where
    S:Command_SI
{
    switch!{cmd;
        // This matches the SendEmail variant and destructures it into the
        // `to` and `content` fields (by reference,because of the `ref`).
        ref SendEmail{to,content}=>{
            println!("Sending message to the '{}' email address.",to);
            println!("Content:{:?}",content);
            Ok(())
        }
        // `cmd` is moved into the branch here,
        // wrapped into a `VariantProxy<S,TS!(RemoveAddress)>`,
        // which allows direct access to the fields in the variant.
        //
        // This does not destructure the variant because
        // it's not possible to unwrap a structural type into multiple fields yet
        // (special casing the single field case doesn't seem like a good idea).
        RemoveAddress=>{
            let address=cmd.into_field(fp!(0));
            println!("removing the '{}' email address",address);
            Ok(())
        }
        _=>Err(UnsupportedCommand(cmd))
    }
}

#[derive(Structural)]
enum Command{
    // The `newtype(bounds="...")` attribute marks the variant as being a newtype variant,
    // delegating field accessors for the variant to `SendEmail`(its one field),
    // as well as replacing the bounds for the variant in the generated
    // `Command_SI` and `Command_ESI` traits with `SendEmail_VSI<TS!(SendEmail)>`.
    //
    // `SendEmail_VSI` was generated by the `Structural` derive on `SendEmail`,
    // with accessor trait bounds for accessing the struct's fields
    // in a variant (it takes the name of the variant as a generic parameter).
    #[struc(newtype(bounds="SendEmail_VSI<@variant>"))]
    SendEmail(SendEmail),
    RemoveAddress(String),
}

#[derive(Structural)]
pub struct SendEmail{
    pub to: String,
    pub content: String,
}

#[derive(Debug,Structural,Clone,PartialEq)]
// This attribute stops the generation of the
// `ExtraCommands_SI` and `ExtraCommands_ESI` traits
#[struc(no_trait)]
pub enum ExtraCommand{
    SendEmail{
        to: String,
        content: String,
        topic: String,
    },
    RemoveAddress(String),
    CreateAddress(String),
}

#[derive(Debug,PartialEq)]
pub struct UnsupportedCommand<T>(pub T);

```

### Structural alias for struct

This demonstrates how you can define a trait aliasing field accessors,
using a fields-in-traits syntax.

For more details you can look at the docs for the [`structural_alias`] macro.

```rust

use structural::{StructuralExt,Structural,structural_alias,fp};

use std::borrow::Borrow;

structural_alias!{
    trait Person<H:House>{
        name:String,
        house:H,
    }

    trait House{
        dim:Dimension3D,
    }
}


fn print_name<T,H>(this:&T)
where
    T:?Sized+Person<H>,
    H:House,
{
    let (name,house_dim)=this.fields(fp!( name, house.dim ));
    println!("Hello, {}!", name);

    let (w,h,d)=house_dim.fields(fp!( width, height, depth ));

    if w*h*d >= 1_000_000 {
        println!("Your house is enormous.");
    }else{
        println!("Your house is normal sized.");
    }
}

// most structural aliases are object safe
fn print_name_dyn<H>(this:&dyn Person<H>)
where
    H:House,
{
    print_name(this)
}



#[derive(Structural)]
#[struc(public)]
struct Dimension3D{
    width:u32,
    height:u32,
    depth:u32,
}

//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate


fn main(){
    let worker=Worker{
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
        house:Mansion{
            dim:Dimension3D{
                width:300,
                height:300,
                depth:300,
            },
            money_vault_location:"In the basement".into(),
        }
    };

    let student=Student{
        name:"Jake English".into(),
        birth_year:1995,
        house:SmallHouse{
            dim:Dimension3D{
                width:30,
                height:30,
                depth:30,
            },
            residents:10,
        }
    };

    print_name(&worker);
    print_name(&student);

    print_name_dyn(&worker);
    print_name_dyn(&student);
}


#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Worker{
    name:String,
    salary:Cents,
    house:Mansion,
}

#[derive(Structural)]
#[struc(public)]
struct Student{
    name:String,
    birth_year:u32,
    house:SmallHouse,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Cents(u64);

#[derive(Structural)]
#[struc(public)]
struct Mansion{
    dim:Dimension3D,
    money_vault_location:String,
}

#[derive(Structural)]
#[struc(public)]
struct SmallHouse{
    dim:Dimension3D,
    residents:u32,
}

```

### Structural alias for enums

This demonstrates how you can use structural aliases for enums.

This shows both exhaustive and nonexhaustive enum structural aliases.

For more details you can look at the docs for the [`structural_alias`] macro.

```rust
use structural::{StructuralExt,Structural,structural_alias,switch,fp};
use std::fmt::Debug;

# fn main(){
pet_animal_ex(&SomeMammals::Dog{years:1,volume_cm3:1});
pet_animal_ex(&SomeMammals::Horse);

// `MoreAnimals` cannot be passed to `pet_animal_ex`
// since that function requires an enum with only `Dog` and `Horse` variants.
assert_eq!( pet_animal(&MoreAnimals::Dog{years:10,volume_cm3:100}), Ok(()) );
assert_eq!( pet_animal(&MoreAnimals::Horse), Ok(()) );
assert_eq!( pet_animal(&MoreAnimals::Cat{lives:9}), Err(CouldNotPet) );
assert_eq!( pet_animal(&MoreAnimals::Seal), Err(CouldNotPet) );
# }

fn pet_animal(animal: &dyn Animal)-> Result<(),CouldNotPet> {
    // `::Dog` accesses the `Dog` variant
    // (without the `::` it'd be interpreted as a field access),
    // The `=>` allows getting multiple fields from inside a nested field
    // (this includes enum variants).
    // `years,volume_cm3` are the field accessed from inside `::Dog`
    let dog_fields = fp!(::Dog=>years,volume_cm3);

    if animal.is_variant(fp!(Horse)) {
        println!("You are petting the horse");
    }else if let Some((years,volume_cm3))= animal.fields(dog_fields) {
        println!("You are petting the {} year old,{} cm³ dog",years,volume_cm3);
    }else{
        return Err(CouldNotPet);
    }
    Ok(())
}

// This can't take a `&dyn Animal_Ex` because traits objects don't
// automatically support upcasting into other trait objects
// (except for auto traits like Send and Sync ).
fn pet_animal_ex(animal: &impl Animal_Ex) {
    pet_animal(animal)
        .expect("`pet_animal` must match on all variants from the `Animal` trait");
}

// The same as `pet_animal` ,except that this uses a `switch`
fn pet_animal_switch(animal: &dyn Animal)-> Result<(),CouldNotPet> {
    switch!{animal;
        ref Horse=>{
            println!("You are petting the horse");
        }
        ref Dog{years,volume_cm3}=>{
            println!("You are petting the {} year old,{} cm³ dog",years,volume_cm3);
        }
        _=>return Err(CouldNotPet)
    }
    Ok(())
}


#[derive(Debug,PartialEq)]
struct CouldNotPet;

structural_alias!{
    // The `#[struc(and_exhaustive_enum(suffix="_Ex"))]` attribute
    // generates the `Animal_Ex` trait with this trait as a supertrait,
    // and with the additional requirement that the enum
    // only has the `horse` and `Dog` variants
    // (They variants can still have more fields than required).
    //
    // structural aliases can have supertraits,here it's `Debug`
    #[struc(and_exhaustive_enum(suffix="_Ex"))]
    trait Animal: Debug{
        Horse,
        Dog{years:u16,volume_cm3:u64},
    }
}


#[derive(Debug,Structural)]
# #[struc(no_trait)]
enum SomeMammals{
    Horse,
    Dog{years:u16,volume_cm3:u64},
}

#[derive(Debug,Structural)]
# #[struc(no_trait)]
enum MoreAnimals{
    Cat{lives:u8},
    Dog{years:u16,volume_cm3:u64},
    Horse,
    Seal,
}



```

### Anonymous structs (`make_struct` macro)

This demonstrates how you can construct an anonymous struct.

For more details you can look at the docs for the
[`make_struct`](./macro.make_struct.html) macro.

Docs for the [`impl_struct` macro](./macro.impl_struct.html) macro.

```rust

use structural::{StructuralExt,fp,impl_struct,make_struct,structural_alias};

structural_alias!{
    trait Person<T>{
        // We only have shared access (`&String`) to the field.
        ref name:String,

        // We have shared,mutable,and by value access to the field.
        // Not specifying any of `mut`/`ref`/`move` is equivalent to `mut move value:T,`
        value:T,
    }
}

fn make_person(name:String)-> impl_struct!{ ref name:String, value:() } {
    make_struct!{
        name,
        value: (),
    }
}


fn print_name(mut this: impl_struct!{ ref name:String, value:Vec<String> } ) {
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
    assert_eq!( this.into_field(fp!(value)), list );
}


// most structural aliases are object safe
//
// This has to use the Person trait,
// since `impl_struct!{....}` expands to `impl Trait0+Trait0+etc`
fn print_name_dyn(this:&mut dyn Person<Vec<String>>){
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
}

//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate

fn main(){
    let worker=make_struct!{
        // This derives clone for the anonymous struct
        #![derive(Clone)]
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
        value:vec![],
    };

    let student=make_struct!{
        // This derives clone for the anonymous struct
        #![derive(Clone)]
        name:"Jake English".into(),
        birth_year:1995,
        value:vec![],
    };

    print_name(worker.clone());
    print_name(student.clone());

    print_name_dyn(&mut worker.clone());
    print_name_dyn(&mut student.clone());

    let person=make_person("Louis".into());

    assert_eq!( person.field_(fp!(name)), "Louis" );
    assert_eq!( person.field_(fp!(value)), &() );
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Cents(u64);

```


[`Structural`]: ./docs/structural_macro/index.html
[`GetField`]: ./field/trait.GetField.html
[`GetFieldMut`]: ./field/trait.GetFieldMut.html
[`IntoField`]: ./field/trait.IntoField.html
[`GetVariantField`]: ./field/trait.GetVariantField.html
[`GetVariantFieldMut`]: ./field/trait.GetVariantFieldMut.html
[`IntoVariantField`]: ./field/trait.IntoVariantField.html

[`StrucWrapper`]: ./struct.StrucWrapper.html

[`StructuralExt`]: ./trait.StructuralExt.html
[`impl_struct`]: ./macro.impl_struct.html
[`make_struct`]: ./macro.make_struct.html
[`structural_alias`]: ./macro.structural_alias.html
[`switch`]: ./macro.switch.html

*/
#![cfg_attr(feature = "nightly_impl_fields", feature(associated_type_bounds))]
#![cfg_attr(feature = "nightly_specialization", feature(specialization))]
#![cfg_attr(
    all(feature = "nightly_use_const_str", not(feature = "disable_const_str")),
    feature(const_if_match)
)]
#![cfg_attr(
    all(feature = "nightly_use_const_str", not(feature = "disable_const_str")),
    feature(const_generics)
)]
#![cfg_attr(
    all(feature = "nightly_use_const_str", not(feature = "disable_const_str")),
    allow(incomplete_features)
)]
#![deny(rust_2018_idioms)]
#![allow(non_camel_case_types)]
#![no_std]
// The associated constants from this crate use trait bounds,
// so they can't be translated to `const fn`.
// Also,the constants don't use cell types,they're just generic.
#![allow(clippy::declare_interior_mutable_const)]
// This triggers for types that represent values, like `NestedFieldPath<(TS!(0), TS!(1))>`,
// so it's mostly noise in this crate.
#![allow(clippy::type_complexity)]
// This lint is silly
#![allow(clippy::blacklisted_name)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::wildcard_imports)]

#[cfg(feature = "std")]
pub extern crate std;

#[doc(hidden)]
pub extern crate core as std_;

#[cfg(all(feature = "alloc"))]
#[cfg_attr(feature = "hide_reexports", doc(hidden))]
pub extern crate alloc;

extern crate self as structural;

pub use structural_derive::Structural;

#[doc(hidden)]
pub use structural_derive::{
    _FP_impl_, _FP_literal_, _TStr_ident_impl_, _TStr_impl_, _TStr_lit_impl_,
    _field_path_aliases_impl, _impl_struct_impl, _switch_tstring_aliases, _tstring_aliases_impl,
    structural_alias_impl,
};

#[macro_use]
mod macros;

#[doc(hidden)]
#[cfg(all(feature = "use_const_str", not(feature = "disable_const_str")))]
pub mod const_generic_utils;
pub mod docs;
pub mod enums;
pub mod field;
#[doc(hidden)]
pub mod msg;
pub mod path;
mod structural_ext;
mod structural_trait;
pub mod type_level;
pub mod utils;
mod wrapper;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
pub mod tests;

include! {"p.rs"}

pub use crate::{
    field::{
        FieldType, GetField, GetFieldMut, GetFieldType, GetFieldType2, GetFieldType3,
        GetFieldType4, GetVariantField, GetVariantFieldMut, GetVariantFieldType, IntoField,
        IntoFieldMut, IntoVariantField, IntoVariantFieldMut,
    },
    structural_ext::StructuralExt,
    structural_trait::Structural,
    wrapper::StrucWrapper,
};

/// Reexports from other crates.
///
/// This reexports from `core_extensions` unconditionally,
/// and `alloc` conditionally.
pub mod reexports {
    #[doc(no_inline)]
    pub use core_extensions::{
        collection_traits::{Cloned, IntoArray},
        const_default, ConstDefault,
    };

    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "hide_reexports", doc(hidden))]
    #[doc(no_inline)]
    pub use crate::alloc::boxed::Box;
}

// pmr(proc macro reexports):
// Reexports for the proc macros in structural_derive.
//
// Importing stuff from this module anywhere other than `structural_derive` is
// explicitly disallowed,and is likely to break.
#[doc(hidden)]
pub mod pmr {
    pub use crate::enums::variant_count::*;
    pub use crate::enums::*;
    pub use crate::field::*;
    pub use crate::path::*;
    pub use crate::type_level::collection_traits::*;
    pub use crate::type_level::*;
    pub use crate::utils::{_Structural_BorrowSelf, as_phantomdata};
    pub use core_extensions::type_level_bool::{Boolean, False, True};
    pub use core_extensions::{ConstDefault, MarkerType};

    pub use crate::std_::{
        marker::PhantomData,
        mem::drop,
        option::Option::{self, None, Some},
        ptr::NonNull,
    };

    #[cfg(feature = "alloc")]
    pub use crate::alloc::boxed::Box;
}

/// Structural-deriving types used in examples,
///
/// These are in the docs purely so that documentation examples only use
/// types that are documented.
///
/// You can only use items from this module when the "for_examples" feature is enabled.
#[cfg(any(feature = "for_examples", all(rust_1_41, doc)))]
pub mod for_examples;

/// Structural-deriving types used in examples,
///
/// You can only use items from this module when the "for_examples" feature is enabled.
#[cfg(all(not(feature = "for_examples"), not(all(rust_1_41, doc))))]
pub mod for_examples {}

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

//////////////////////////////

use std_::marker::PhantomData;
use std_::mem::ManuallyDrop;

include! {"path/declare_field_path_types.rs"}
