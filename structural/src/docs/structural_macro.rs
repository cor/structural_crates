/*!

The Structural derive macro implements the Structural trait, as well as accessor traits.

The accessor traits are
 [`GetField`]/[`GetFieldMut`]/[`IntoField`] for structs,
and [`GetVariantField`]/[`GetVariantFieldMut`]/[`IntoVariantField`] for enums.

Every instance of `<DerivingType>` in the documentation is the name of the type.
If have a `Money` type,`<DerivingType>_Foo` means `Money_Foo`.

# Enums

For complementary documentation on using the `Structural` derive macro with enums
[look here](../enums/index.html)

# Default Behavior for Structs

By default,this derive generates:

- Implementation of the structural trait for the deriving type,
with documentation describing all the accessor trait impls for the type.

- Implementations of the accessor traits ([`GetField`]/[`GetFieldMut`]/[`IntoField`])
for pub fields.

- Implementation of the [`DropFields`] trait,
if the type has by-value accessors (implements [`IntoField`]) for any field.

- A trait named `<DerivingType>_SI`,aliasing the accessor traits for the type,
implemented for all types with the same accessor trait impls.

- A trait named `<DerivingType>_VSI`,
to use the struct (or any other struct that implements the same accessor traits)
in a newtype variant,by annotating the variant with
`#[struc(newtype(bounds="<DerivingType>_VSI<@variant>"))]`.

Many of these can be overriden.

[`GetField`]: ../../field/trait.GetField.html
[`GetFieldMut`]: ../../field/trait.GetFieldMut.html
[`IntoField`]: ../../field/trait.IntoField.html
[`GetVariantField`]: ../../field/trait.GetVariantField.html
[`GetVariantFieldMut`]: ../../field/trait.GetVariantFieldMut.html
[`IntoVariantField`]: ../../field/trait.IntoVariantField.html
[`TStr`]: ../../struct.TStr.html

[`StructuralExt::into_fields`]: ../../trait.StructuralExt.html#method.into_fields

[`StrucWrapper::vals`]: ../../struct.StrucWrapper.html#method.vals

[`DropFields`]: ../../field/ownership/trait.DropFields.html

[`DropFields::drop_fields`]:
../../field/ownership/trait.DropFields.html#tymethod.drop_fields

[`DropFields::pre_move`]:
../../field/ownership/trait.DropFields.html#tymethod.pre_move

[`PrePostDropFields`]: ../../field/ownership/trait.PrePostDropFields.html

[`PrePostDropFields::pre_drop`]:
../../field/ownership/trait.PrePostDropFields.html#method.pre_drop

[`PrePostDropFields::post_drop`]:
../../field/ownership/trait.PrePostDropFields.html#method.post_drop

<span id="drop-behavior-section"></span>
# Drop behavior

When converting a type into multiple fields by value
(using [`StructuralExt::into_fields`] or [`StrucWrapper::vals`]),
the regular destructor (`Drop::drop`) won't run,
instead the steps [in the section below](#drop-order-section) will happen.

If your type has code that must run when it's dropped,
you can use the [`#[struc(pre_move="foo_function")]`](#strucpre_move) attribute to
run that code before the type is converted into multiple fields by value,

For an example of using the `#[struc(pre_move="foo")]` attribute,[look here](#with-pre-move).

<span id="drop-order-section"></span>
### Drop order

The order of operations when invoking [`StructuralExt::into_fields`] or [`StrucWrapper::vals`]
is this by default:

- Call [`DropFields::pre_move`].

- Move out the fields.

- Call [`PrePostDropFields::pre_drop`].

- Drop the public fields (those with accessor impls) that weren't moved out,in declaration order.

- Drop the private fields(fields that don't have accessor impls),in declaration order.

- Call [`PrePostDropFields::post_drop`].

- Return the moved out fields

# Container Attributes

### `#[struc(debug_print)]`

Prints the output of the derive macro by panicking.

### `#[struc(bound="T:Trait")]`

Adds a bound to every accessor trait impl.

### `#[struc(no_trait)]`

Disables the generation of the `*SI` traits.

[Here is an example using this attribute](#disabling-the-trait-alias)

### `#[struc(no_docs)]`

Removes the docs for the generated traits,and impl of `Structural`.

The documentation describes variants and fields that the accessor trait impls represent.

### `#[struc(from_structural)]`

Causes the [`FromStructural`] and [`TryFromStructural`] traits to be derived.

The trait impls require that the converted-from type has
by-value accessors with the same names and type as this type's public fields.

Private fields must be annotated with one of the [`#[struc(init_*)]`](#init-attributes)
attributes.

### `#[non_exhaustive]`

This is only usable on enums.

The structural macro recognizes this built-in attribute,
and marks the enum as having non-exhaustive variants,
meaning that:

- You will not be able to exhaustively match on it with the `switch` macro

- It will not implement the `VariantCount` trait.

- It will not have a `*_ESI` trait generated for it.

- Using the `#[struc(variant_count_alias)]` attribute will cause an error.


### `#[struc(variant_count_alias)]`

This is only usable on enums.

This generates a type alias with the amount of variants in the enum.

Small example:<br>
For this enum:`pub enum Foo{Bar,Baz}`<br>
This macro would generate:`pub type Foo_VC=TS!(2);`<br>
As well as documentaion explaining what the alias is.

### `#[struc(pre_move="foo")]`

This only has an effect for types with at least one by-value accessor impl.

Changes the implementation of [`DropFields::pre_move`] to run the `foo` function,
allowing custom code to run before the type is converted into multiple fields by value.

For enums: `foo` is called before calling `DropFields::pre_move` on
the delegated-to field in newtype variants.

For an example of using the `#[struc(pre_move="foo")]` attribute,[look here](#with-pre-move).

For a reference on the what happens when converting a type into multiple fields by value
[go here](#drop-behavior-section).

### `#[struc(pre_post_drop_fields)]`

This only has an effect for types with at least one by-value accessor impl.

Changes the implementation of [`DropFields`] to run
[`PrePostDropFields::pre_drop`] before and [`PrePostDropFields::post_drop`]
after the non-moved-out fields are dropped in [`DropFields::drop_fields`].

This requires a manual implementation of [`PrePostDropFields`].

For a reference on the what happens when converting a type into multiple fields by value
[go here](#drop-behavior-section).

# Variant Attributes

### `#[struc(rename="<new_name>")]`

Changes the name for the variant in the accessor trait impls.

The name can be anything,including non-ascii identifiers.

[For an example of renaming variants to non-ascii identifiers look here](#non-ascii-idents)

### `#[struc(replace_bounds="bounds")]`

Replaces (in the generated trait) the bounds for this particular variant with
the ones in the attribute.

All `@variant` in the bounds will be replaced with a [`TStr`] containing the
name of the variant(eg: `TS!(Foo)` for the `Foo` variant ),

### `#[struc(newtype)]`

Marks a variant as a newtype variant,
delegating access to fields in the variant to the single field of the variant.

This attribute can have an optional argument:

- `#[struc(newtype(bounds="Baz_VSI<'a,u8,@variant>"))]`:

All `@variant` in the bounds will be replaced with a [`TStr`] containing the
name of the variant(eg: `TS!(Foo)` for the `Foo` variant ),

Example:`#[struc(newtype(bounds = "Foo_VSI<@variant>"))]` <br>
Example:`#[struc(newtype(bounds = "Bar_VSI<T,U,@variant>"))]` <br>
Example:`#[struc(newtype(bounds = "Baz_VSI<'a,u8,@variant>"))]` <br>

# Field Attributes

### `#[struc(rename="<new_name>")]`

Changes the name for the field in the accessor trait impls.

The name can be anything,including non-ascii identifiers.

[For an example of renaming fields to non-ascii identifiers look here](#non-ascii-idents)

### `#[struc(impl="<trait bounds>")]`

This requires the `nightly_impl_fields` cargo feature
(or `impl_fields` if associated type bounds stabilized after the latest release).

Changes the generated `*SI` traits (which aliases the accessor traits for this type)
not to refer to the type of this field,
instead it will be required to implement the bounds passed to this attribute.

Note that these bounds are only added to the generated `*SI` traits.

[Here is the example for this attribute.](#impl-trait-fields)

### `#[struc(delegate_to)]`

This can only be used with structs.

Delegates the implementation of the Structural and accessor traits to this field.

You can only delegate the implementation and Structural and accessor traits
to a single field.

Using this attribute will disable the generation of traits.

Optional arguments for `delegate_to`:

- `bound="T:bound"`: Adds the constraint to all the trait impls.
- `mut_bound="T:bound"`: Adds the constraint to the `GetField` impl.
- `into_bound="T:bound"`: Adds the constraint to the `IntoField` impl.

<span id="init-attributes"></span>

### `#[struc(init_with_fn = "<callable_expression>")]`

This can only be used in combination with the `#[struc(from_structural)]` container attribute.

Initialize the field in the `FromStructural` impl  with the return value of
calling the `<callable_expression>` expression.

The `init_*` attributes are necessary when deriving `FromStructural` in structs
with private fields.

Public fields with `init_*` attributes aren't added as bounds for the converted-from type,
which means that removeing the `init_*` attributes is a breaking change.

Examples:<br>
- `#[struc(init_with_fn = "foo::bar")]` <br>
- `#[struc(init_with_fn = r#"|| expensive(100, "hello") "#)]` <br>

### `#[struc(init_with_val = "<expression>")]`

This can only be used in combination with the `#[struc(from_structural)]` container attribute.

Initialize the field in the `FromStructural` impl  with the `<expression>` expression.

[`init_with_lit`](#init_with_lit) is better to initialize the field with a string literal,
since this attribute requires escaping the string literal, or using raw strings.

The `init_*` attributes are necessary when deriving `FromStructural` in structs

Public fields with `init_*` attributes aren't added as bounds for the converted-from type,
which means that removeing the `init_*` attributes is a breaking change.
with private fields.

Examples:<br>
- `#[struc(init_with_val = "()")]`<br>
- `#[struc(init_with_val = "100")]`<br>
- `#[struc(init_with_val = r"100")]`<br>
- `#[struc(init_with_val = "()")]`<br>

<span id="init_with_lit"></span>
### `#[struc(init_with_lit = <literal>)]`

This can only be used in combination with the `#[struc(from_structural)]` container attribute.

Initialize the field in the `FromStructural` impl  with the `<literal>` literal.
Note that only literals parseable as `syn::Lit` can be used here,
numbers, strings, bools, etc.

The `init_*` attributes are necessary when deriving `FromStructural` in structs
with private fields.

Public fields with `init_*` attributes aren't added as bounds for the converted-from type,
which means that removeing the `init_*` attributes is a breaking change.

Examples:<br>
- `#[struc(init_with_lit = "foo")]`<br>
- `#[struc(init_with_lit = 100)]`<br>


### `#[struc(init_with_default)]`

This can only be used in combination with the `#[struc(from_structural)]` container attribute.

Initialize the field with its default value, requires the field type to implement `Default`.

The `init_*` attributes are necessary when deriving `FromStructural` in structs
with private fields.

Public fields with `init_*` attributes aren't added as bounds for the converted-from type,
which means that removeing the `init_*` attributes is a breaking change.

# Container/Variant/Field Attributes

Unless stated otherwise,
when these attributes are put on the container or variant it will have the same effect as
being put on the field,and are overriden by attributes directly on the field.

### `#[struc(access="")]`

Changes the implemented accessor traits for the field(s).

`#[struc(access="ref")]`:
Generates impls of the `GetField` trait for the field(s).

`#[struc(access="mut")]`:
Generates impls of the `GetField`+`GetFieldMut` traits for the field(s).

`#[struc(access="move")]`:
Generates impls of the `GetField`+`IntoField` traits for the field(s).

`#[struc(access="mut move")]`:
Generates impls of the `GetField`+`GetFieldMut`+`IntoField` traits for the field(s).

When this attribute is used on a non-pub field,
it'll mark the field as public for the purpose of generating accessor trait impls.

When these attribute are used on enums it generates impls for the
`GetVariantField`/`GetVariantFieldMut`/`IntoVariantField`
traits instead of `GetField`/`GetFieldMut`/`IntoField`.

# Container/Field Attributes

Unless stated otherwise,
when these attributes are put on the container it will have the same effect as
being put on the field,and are overriden by attributes directly on the field.

### `#[struc(public)]`

Marks the fields as public,generating the accessor traits for the field.

### `#[struc(not_public)]`

Marks the fields as private,not generating the accessor traits for the field.


# Examples


### Accessing Fields

This example shows many of the ways that fields can be accessed.

```
use structural::{StructuralExt,Structural,fp};

fn main(){
    let array=[
        10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,
        26,27,28,29,30,31,32,33,34,35,36,37,38,39,30,31,
    ];

    with_struct(Foo{
        name: "foo",
        year: 2020,
        tuple: Some((3,5,8)),
        array,
    });

    with_struct(Bar{
        name: "foo",
        surname:"metavariable",
        year: 2020,
        tuple: Some((3,5,8)),
        array,
    });
}

fn with_struct<This>(mut foo:This)
where
    This: Foo_SI + Clone,
{
    ////////////////////////////////////////////////////
    ////            field_ method
    assert_eq!( foo.field_(fp!(name)), &"foo" );
    assert_eq!( foo.field_(fp!(year)), &2020 );

    assert_eq!( foo.field_(fp!(tuple)), &Some((3,5,8)) );
    assert_eq!( foo.field_(fp!(tuple?)), Some(&(3,5,8)) );
    assert_eq!( foo.field_(fp!(tuple?.0)), Some(&3) );
    assert_eq!( foo.field_(fp!(tuple?.1)), Some(&5) );
    assert_eq!( foo.field_(fp!(tuple?.2)), Some(&8) );

    ////////////////////////////////////////////////////
    ////            field_mut method
    assert_eq!( foo.field_mut(fp!(name)), &mut "foo" );
    assert_eq!( foo.field_mut(fp!(year)), &mut 2020 );

    assert_eq!( foo.field_mut(fp!(tuple)), &mut Some((3,5,8)) );
    assert_eq!( foo.field_mut(fp!(tuple?)), Some(&mut (3,5,8)) );
    assert_eq!( foo.field_mut(fp!(tuple?.0)), Some(&mut 3) );
    assert_eq!( foo.field_mut(fp!(tuple?.1)), Some(&mut 5) );
    assert_eq!( foo.field_mut(fp!(tuple?.2)), Some(&mut 8) );


    ////////////////////////////////////////////////////
    ////            into_field method
    assert_eq!( foo.clone().into_field(fp!(name)), "foo" );
    assert_eq!( foo.clone().into_field(fp!(year)), 2020 );

    assert_eq!( foo.clone().into_field(fp!(tuple)), Some((3,5,8)) );
    assert_eq!( foo.clone().into_field(fp!(tuple?)), Some((3,5,8)) );
    assert_eq!( foo.clone().into_field(fp!(tuple?.0)), Some(3) );
    assert_eq!( foo.clone().into_field(fp!(tuple?.1)), Some(5) );
    assert_eq!( foo.clone().into_field(fp!(tuple?.2)), Some(8) );

    ////////////////////////////////////////////////////
    ////            fields method
    assert_eq!( foo.fields(fp!(name, year)), (&"foo",&2020) );
    assert_eq!( foo.fields(fp!(=>name,year)), (&"foo",&2020) );

    // Where you place the `?`  matters,
    // if it's after the `=>`,it returns an `Option` for every single field.
    assert_eq!(
        foo.fields(fp!(tuple=> ?.0, ?.1, ?.2)),
        (Some(&3),Some(&5),Some(&8))
    );
    // If the `?` is before the `=>`,
    // it returns an `Option` wrapping all references to the fields.
    assert_eq!( foo.fields(fp!(tuple?=>0,1,2)), Some((&3,&5,&8)) );

    assert_eq!(
        foo.fields(fp!(array=>0,1,2,3,4,5,6,7)),
        (&10,&11,&12,&13,&14,&15,&16,&17),
    );

    // If you access more than 8 fields,the fields method returns tuples of tuples,
    // in which the nested tuples reference 8 fields each.
    assert_eq!(
        foo.fields(fp!(array=>0,1,2,3,4,5,6,7,8)),
        (
            (&10,&11,&12,&13,&14,&15,&16,&17),
            (&18,),
        )
    );
    assert_eq!(
        foo.fields(fp!(array=>0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17)),
        (
            (&10,&11,&12,&13,&14,&15,&16,&17),
            (&18,&19,&20,&21,&22,&23,&24,&25),
            (&26,&27),
        )
    );


    ////////////////////////////////////////////////////
    ////            fields_mut method
    assert_eq!( foo.fields_mut(fp!(name, year)), (&mut "foo",&mut 2020) );
    assert_eq!( foo.fields_mut(fp!(=>name,year)), (&mut "foo",&mut 2020) );

    assert_eq!(
        foo.fields_mut(fp!(tuple=> ?.0, ?.1, ?.2)),
        (Some(&mut 3),Some(&mut 5),Some(&mut 8))
    );
    assert_eq!( foo.fields_mut(fp!(tuple?=>0,1,2)), Some((&mut 3, &mut 5, &mut 8)) );

    assert_eq!(
        foo.fields_mut(fp!(array=>0,1,2,3,4,5,6,7)),
        (&mut 10, &mut 11, &mut 12, &mut 13, &mut 14, &mut 15, &mut 16, &mut 17),
    );

    // If you access more than 8 fields,the `fields_mut` method returns tuples of tuples,
    // in which the nested tuples reference 8 fields each.
    assert_eq!(
        foo.fields_mut(fp!(array=>0,1,2,3,4,5,6,7,8)),
        (
            (&mut 10, &mut 11, &mut 12, &mut 13, &mut 14, &mut 15, &mut 16, &mut 17),
            (&mut 18,),
        )
    );
    assert_eq!(
        foo.fields_mut(fp!(array=>0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17)),
        (
            (&mut 10, &mut 11, &mut 12, &mut 13, &mut 14, &mut 15, &mut 16, &mut 17),
            (&mut 18, &mut 19, &mut 20, &mut 21, &mut 22, &mut 23, &mut 24, &mut 25),
            (&mut 26, &mut 27),
        )
    );

    ////////////////////////////////////////////////////
    ////            into_fields method
    assert_eq!( foo.clone().into_fields(fp!(name, year)), ("foo",2020) );
    assert_eq!( foo.clone().into_fields(fp!(=>name,year)), ("foo",2020) );

    assert_eq!( foo.clone().into_fields(fp!(tuple?=>0,1,2)), Some((3, 5, 8)) );

    assert_eq!(
        foo.clone().into_fields(fp!(array=>0,1,2,3,4,5,6,7)),
        (10, 11, 12, 13, 14, 15, 16, 17),
    );

    // If you access more than 8 fields,the `into_fields` method returns tuples of tuples,
    // in which the nested tuples have 8 fields each.
    assert_eq!(
        foo.clone().into_fields(fp!(array=>0,1,2,3,4,5,6,7,8)),
        (
            (10, 11, 12, 13, 14, 15, 16, 17),
            (18,),
        )
    );
    assert_eq!(
        foo.clone().into_fields(fp!(array=>0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17)),
        (
            (10, 11, 12, 13, 14, 15, 16, 17),
            (18, 19, 20, 21, 22, 23, 24, 25),
            (26, 27),
        )
    );


}

#[derive(Structural,Clone)]
#[struc(public)]
struct Foo{
    name: &'static str,
    year: i64,
    tuple: Option<(u32,u32,u32)>,
    array: [u8;32],
}

#[derive(Structural,Clone)]
# #[struc(no_trait)]
#[struc(public)]
struct Bar{
    name:&'static str,
    surname:&'static str,
    year:i64,
    tuple: Option<(u32,u32,u32)>,
    array: [u8;32],
}


```

### Basic example

```rust
use structural::{Structural,StructuralExt,structural_alias,fp};


fn reads_pair<O>(pair:&O)
where
    // This uses the trait generated by `#[derive(Structural)]`,
    // aliasing the accessor traits implemented for `Hello`,
    // allowing any type with (at least) those fields to be passed here.
    O:Hello_SI
{
    let (a,b)=pair.fields(fp!( a, b ));
    assert_eq!(a,&11);
    assert_eq!(b,&33);
}


#[derive(Debug,Structural,PartialEq,Eq)]
#[struc(public)]
struct Hello{
    a:u32,
    b:u32
}

#[derive(Structural)]
#[struc(access="mut move")]
#[struc(public)]
struct World{
    run:String,
    a:u32,
    b:u32,
}

fn main(){
    reads_pair(&Hello{ a:11, b:33 });

    reads_pair(&World{ run:"nope".into(), a:11, b:33 });
}


```

### Mutating fields

```rust
use structural::{Structural,StructuralExt,structural_alias,fp};


structural_alias!{
    trait Tuple2<T>{
        0:T,
        1:T,
    }
}


fn mutates_pair<O>(pair:&mut O)
where
    O:Tuple2<u32>
{
    let a=pair.field_mut(fp!(0));
    assert_eq!(a,&mut 14);
    *a*=2;

    let b=pair.field_mut(fp!(1));
    assert_eq!(b,&mut 16);
    *b*=2;
}


#[derive(Debug,Structural,PartialEq,Eq)]
struct Point(
    #[struc(public)]
    u32,

    #[struc(public)]
    u32,

    #[struc(not_public)]
    pub u32,
);

fn main(){
    let mut point=Point(14,16,11);
    let mut tuple=(14,16);

    mutates_pair(&mut point);
    mutates_pair(&mut tuple);

    assert_eq!(point,Point(28,32,11));
    assert_eq!(tuple,(28,32));
}

```

### Disabling the trait alias

This example demonstrates how one disables the generation of the
`<DerivingType>_SI` trait to declare it manually.

```rust
use structural::{Structural,IntoFieldMut,StructuralExt,FP};

#[derive(Debug,Structural,PartialEq,Eq)]
#[struc(no_trait)]
#[struc(access="mut move")]
struct Hello{
    pub hello:u32,
    pub world:String,
}


pub trait Hello_SI:
    IntoFieldMut<FP!(hello), Ty=u32>+
    IntoFieldMut<FP!(world), Ty=String>
{}

impl<T> Hello_SI for T
where
    T:?Sized+
        IntoFieldMut<FP!(hello), Ty=u32>+
        IntoFieldMut<FP!(world), Ty=String>
{}

```

### Impl trait fields

This is an example of using the `#[struc(impl="<trait_bounds>")]` attribute

This requires the `nightly_impl_fields` cargo feature
(or `impl_fields` if associated type bounds stabilized after the latest release).

*/
#![cfg_attr(not(feature="nightly_impl_fields"),doc="```ignore")]
#![cfg_attr(feature="nightly_impl_fields",doc="```rust")]
/*!

// Remove this if associated type bounds (eg: `T: Iterator<Item: Debug>`)
// work without it.
#![feature(associated_type_bounds)]

use std::borrow::Borrow;

use structural::{Structural,fp,make_struct,StructuralExt};


#[derive(Structural)]
#[struc(public)]
struct Person{
    #[struc(impl="Borrow<str>")]
    name:String,

    #[struc(impl="Copy+Into<u64>")]
    height_nm:u64
}

/// `Person_SI` was generated for `Person` by the `Structural` derive macro.
fn takes_person(this:&impl Person_SI){
    let (name,height)=this.fields(fp!(name,height_nm));
    assert_eq!( name.borrow(), "bob" );

    assert_eq!( (*height).into(), 1_500_000_000 );
}


// Notice how `name` is a `&'static str` instead of a `String`,
// and `height_nm` is a `u32` instead of a `u64`?
// This is possible because the concrete types of the fields weren't used in
// the `Person_SI` trait.
takes_person(&make_struct!{
    name:"bob",
    height_nm: 1_500_000_000_u32,
});

takes_person(&Person{
    name:"bob".to_string(),
    height_nm: 1_500_000_000_u64,
});

```

### Delegation

This is an example of using the `#[struc(delegate_to)]` attribute.

```
use structural::{fp,make_struct,StructuralExt,Structural};


#[derive(Structural,Clone)]
struct Foo<T>{
    #[struc(delegate_to)]
    value:T
}

# // ensuring that Foo_SI wasn't generated
# trait Foo_SI{}

#[derive(Structural,Clone)]
#[struc(public,access="ref")]
struct AnimalCounts{
    cows:u32,
    chickens:u32,
    pigs:u32,
}


fn total_count(animals:&dyn AnimalCounts_SI)->u64{
    *animals.field_(fp!(cows)) as u64+
    *animals.field_(fp!(chickens)) as u64+
    *animals.field_(fp!(pigs)) as u64
}

{
    let count=total_count(&Foo{
        value:make_struct!{
            cows:100,
            chickens:200,
            pigs:300,
        }
    });

    assert_eq!( count, 600 );
}

{
    let count=total_count(&Foo{
        value:AnimalCounts{
            cows:0,
            chickens:500,
            pigs:0,
        }
    });

    assert_eq!( count, 500 );
}

{
    let count=total_count(&AnimalCounts{
        cows:0,
        chickens:500,
        pigs:1_000_000_000,
    });

    assert_eq!( count, 1_000_000_500 );
}





```

### Delegation,with bounds

This is an example of using the `#[struc(delegate_to())]` attribute with
extra bounds in the accessor trait impls.

```
use structural::{fp,make_struct,StructuralExt,Structural};

use std::{
    fmt::Debug,
    ops::Add,
};


#[derive(Structural,Debug,Copy,Clone,PartialEq)]
struct Foo<T>{
    #[struc(delegate_to(
        bound="T:PartialEq",
        mut_bound="T:Copy",
        into_bound="T:Debug",
    ))]
    value:T
}

#[derive(Structural,Debug,Copy,Clone,PartialEq)]
#[struc(public)]
struct AnimalCounts<T>{
    cows:T,
    chickens:T,
    pigs:T,
}

fn total_count<T>(animals:&dyn AnimalCounts_SI<T>)->T
where
    T: Clone+Add<Output=T>,
{
    let (a,b,c)=animals.cloned_fields(fp!( cows, chickens, pigs ));
    a + b + c
}

{
    let count=total_count(&Foo{
        value:AnimalCounts{
            cows:100,
            chickens:200,
            pigs:300,
        }
    });

    assert_eq!( count, 600 );
}

// This doesn't compile because
// AddableString doesn't satisfy the Copy bound added by `mut_bound="T:Copy"`
/*
{
    let count=total_count(&Foo{
        value: AnimalCounts::<AddableString> {
            cows: "foo".into(),
            chickens: "bar".into(),
            pigs: "baz".into(),
        }
    });

    assert_eq!( count.0, "foobarbaz" );
}
*/


#[derive(Debug,Clone,PartialEq)]
struct AddableString(String);

impl<'s> From<&'s str> for AddableString{
    fn from(s:&'s str)-> AddableString {
        AddableString( s.to_string() )
    }
}

impl Add for AddableString{
    type Output=Self;

    fn add(self,other:Self)->Self{
        AddableString( self.0 + other.0.as_str() )
    }
}

```

### Non-ascii idents

This is an example of using non-ascii identifiers.

Unfortunately,without enabling the "use_const_str" feature to use const generics internally,
compile-time errors are significantly less readable than with ascii identifiers.

```rust
use structural::{fp,make_struct,StructuralExt,Structural};

////////////////////////////////////////////////////
//                    structs

#[derive(Structural)]
#[struc(public)]
struct Family{
    #[struc(rename="儿子数")]
    sons: u32,
    #[struc(rename="女儿们")]
    daughters: u32,
}

let mut this=Family{
    sons: 34,
    daughters: 55,
};

assert_eq!( this.fields(fp!("儿子数","女儿们")), (&34,&55) );
assert_eq!( this.fields_mut(fp!("儿子数","女儿们")), (&mut 34,&mut 55) );

////////////////////////////////////////////////////
//                    Enums

#[derive(Structural)]
enum Vegetable{
    #[struc(rename="Ziemniak")]
    Potato{
        #[struc(rename="centymetry objętości")]
        volume_cm: u32,
    },
    #[struc(rename="生菜")]
    Letuce{
        #[struc(rename="树叶")]
        leaves: u32,
    }
}

let mut potato=Vegetable::Potato{ volume_cm: 13 };
let mut letuce=Vegetable::Letuce{ leaves: 21 };

assert_eq!( potato.field_(fp!(::"Ziemniak"."centymetry objętości")), Some(&13) );
assert_eq!( potato.field_(fp!(::"生菜"."树叶")), None );

assert_eq!( letuce.field_(fp!(::"Ziemniak"."centymetry objętości")), None );
assert_eq!( letuce.field_(fp!(::"生菜"."树叶")), Some(&21) );

assert_eq!( potato.field_mut(fp!(::"Ziemniak"."centymetry objętości")), Some(&mut 13) );
assert_eq!( potato.field_mut(fp!(::"生菜"."树叶")), None );

assert_eq!( letuce.field_mut(fp!(::"Ziemniak"."centymetry objętości")), None );
assert_eq!( letuce.field_mut(fp!(::"生菜"."树叶")), Some(&mut 21) );

```

<span id="with-pre-move"></span>
### Using the `#[struc(pre_move="foo")]` attribute

This demonstrates how you can run code when converting a type into multiple fields by value.

```rust
use structural::{fp, StructuralExt, Structural};

{
    let mut vector=Vec::new();
    drop(WithDropLogic{
        vector: &mut vector,
        x: 0,
        y: 0,
        z: 0
    });
    // The vector was written to in WithDropLogic's impl of `Drop::drop`
    assert_eq!(vector, [1001]);
}
{
    let mut vector=Vec::new();
    let this=WithDropLogic{
        vector: &mut vector,
        x: 3,
        y: 5,
        z: 8
    };
    assert_eq!(into_xyz(this), (3, 5, 8));
    // The vector was written to in `WithDropLogic::drop_`,
    // because `DropFields::pre_move` delegates to it.
    // (`WithDropLogic::drop_` was passed to the `#[struc(pre_move="...")]` attribute)
    assert_eq!(vector, [1001]);
}
{
    let this=Variables{
        x: 13,
        y: 21,
        z: 34,
    };
    assert_eq!(into_xyz(this), (13, 21, 34));
}

// The `Variables_SI` trait was generated by the `Structural` derive on `Variables`,
// aliasing its accessor trait impls.
fn into_xyz(this: impl Variables_SI)->(u32,u32,u32) {
    this.into_fields(fp!(x,y,z))
}


#[derive(Structural)]
struct Variables{
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Structural)]
# #[struc(no_trait)]
#[struc(pre_move="WithDropLogic::drop_")]
struct WithDropLogic<'a>{
    vector: &'a mut Vec<u32>,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl WithDropLogic<'_>{
    fn drop_(&mut self){
        self.vector.push(1001);
    }
}

impl Drop for WithDropLogic<'_>{
    fn drop(&mut self){
        self.drop_();
    }
}


```

[`FromStructural`]: ../../convert/trait.FromStructural.html
[`TryFromStructural`]: ../../convert/trait.TryFromStructural.html


*/
