#[macro_use]
mod delegate_structural;

#[macro_use]
mod list;

#[macro_use]
mod ident;

#[macro_use]
mod make_struct;


#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter{
    ( 
        $(unsafe)?
        impl[$($typarams:tt)*]
            GetField <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::GetField<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Ty=$field_ty;

            fn get_field_(&self)->&Self::Ty{
                &self.$field_name
            }
        }
    };
    ( 
        unsafe impl[$($typarams:tt)*]
            GetFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] GetField<$field_name:$field_ty,$name_param> for $self_
            $( where[$($where_)*] )?
        }
    
        unsafe impl<$($typarams)*> $crate::GetFieldMut<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                &mut self.$field_name
            }

            $crate::unsafe_impl_get_field_raw_mut_method!{
                Self,
                field_name=$field_name,
                name_generic=$name_param
            }
        }
    };
    ( 
        $(unsafe)?
        impl[$($typarams:tt)*]
            IntoField <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] 
                GetField<$field_name:$field_ty,$name_param> 
            for $self_
            $( where[$($where_)*] )?
        }
    
        impl<$($typarams)*> $crate::IntoField<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn into_field_(self)->Self::Ty{
                self.$field_name
            }
            $crate::impl_box_into_field_method!{$name_param}
        }
    };
    ( 
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            unsafe impl[$($typarams)*] 
                GetFieldMut<$field_name:$field_ty,$name_param> 
            for $self_
            $( where[$($where_)*] )?
        }
    
        impl<$($typarams)*> $crate::IntoField<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn into_field_(self)->Self::Ty{
                self.$field_name
            }
            $crate::impl_box_into_field_method!{$name_param}
        }
    };
} 


macro_rules! default_if {
    ( 
        $(#[$attr:meta])*
        cfg($($cfg_attr:tt)*) 
        $($default_impl:tt)*
    ) => (
        #[cfg($($cfg_attr)*)]
        $(#[$attr])*
        default $($default_impl)*

        #[cfg(not($($cfg_attr)*))]
        $(#[$attr])*
        $($default_impl)*
    )
}



/// For manual implementors of the GetFieldMut trait,
/// implementing the methods used for accession multiple mutable fields.
///
/// # Safety
///
/// This is an unsafe macro,
/// because it requires each invocation of it to borrow a different field for the type
/// (the `field_name=` argument),
/// otherwise this would cause undefined behavior because it would 
/// create multiple mutable borrows to the same field.
///
/// # Example
///
/// For an example where this macro is used,
/// you can look at the
/// [manual implementation example of the GetFieldMut trait
/// ](./field_traits/trait.GetFieldMut.html)
#[macro_export]
macro_rules! unsafe_impl_get_field_raw_mut_method {
    ( $Self:ident,field_name=$field_name:tt,name_generic=$name_param:ty ) => (
        unsafe fn get_field_mutref(
            this:*mut (),
            _:$crate::pmr::PhantomData<$name_param>,
        )->*mut $Self::Ty{
            &mut (*(this as *mut $Self)).$field_name as *mut $Self::Ty
        }

        fn get_field_mutref_func(
            &self
        )->$crate::field_traits::GetFieldMutRefFn<$name_param,$Self::Ty>{
            <$Self as $crate::field_traits::GetFieldMut<$name_param>>::get_field_mutref
        }
    )
}



/// For use in manual implementations of the IntoField trait.
/// 
/// Implements the `IntoField::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoField](./field_traits/trait.IntoField.html)
#[macro_export]
#[cfg(not(feature="alloc"))]
macro_rules! impl_box_into_field_method {
    ($($anything:tt)*) => ()
}

/// For use in manual implementations of the IntoField trait.
/// 
/// Implements the `IntoField::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoField](./field_traits/trait.IntoField.html)
#[macro_export]
#[cfg(feature="alloc")]
macro_rules! impl_box_into_field_method {
    ($field_name:ty) => (
        fn box_into_field_(self:structural::alloc::boxed::Box<Self>)->Self::Ty{
            $crate::IntoField::<$field_name>::into_field_(*self)
        }
    )
}



#[doc(hidden)]
#[macro_export]
macro_rules! impl_structural{
    (
        impl[$($typarams:tt)*] Structural for $self_:ty 
        where[$($where_:tt)*]
        {
            field_names=[$( 
                (
                    $field_name:tt : $field_ty:ty,
                    $name_param_ty:ty,
                    $name_param_str:expr,
                ),
            )*]
        }
    )=>{
        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {
            const FIELDS:&'static[$crate::structural_trait::FieldInfo]={
                use $crate::structural_trait::FieldInfo;

                &[
                    $( 
                        FieldInfo{
                            original_name:stringify!($field_name),
                            accessor_name:$name_param_str,
                        },
                    )*
                ]
            };

            type Fields=$crate::TList![
                $(
                    $crate::structural_trait::TField<
                        $name_param_ty,
                        $field_ty,
                    >,
                )*
            ];
        }

        impl<$($typarams)*> $crate::structural_trait::StructuralDyn for $self_
        where $($where_)*
        {
            fn fields_info(&self)->&'static[$crate::structural_trait::FieldInfo]{
                <Self as $crate::Structural>::FIELDS
            }
        }

    }
}



/// Implements StructuralDyn for some type,by delegating to Structural. 
#[macro_export]
macro_rules! impl_structural_dyn{
    (
        impl[$($typarams:tt)*] $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::structural_trait::StructuralDyn for $self_
        $( where $($where_)* )?
        {
            fn fields_info(&self)->&'static[$crate::structural_trait::FieldInfo]{
                <Self as $crate::Structural>::FIELDS
            }
        }
    }
}



#[doc(hidden)]
#[macro_export]
macro_rules! impl_getters_for_derive{
    (   
        impl $typarams:tt $self_:ty 
        where $where_preds:tt
        {
            $((
                $getter_trait:ident< 
                    $field_name:tt : $field_ty:ty,
                    $name_param_ty:ty,
                    $name_param_str:expr,
                > 
            ))*
        }
    )=>{

        $crate::impl_structural!{
            impl $typarams Structural for $self_
            where $where_preds
            {
                field_names=[ 
                    $( 
                        (
                            $field_name : $field_ty,
                            $name_param_ty,
                            $name_param_str,
                        ),
                    )* 
                ]
            }
        }

        $(
            $crate::impl_getter!{
                unsafe impl $typarams 
                    $getter_trait<$field_name : $field_ty,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}

/**

The `structural_alias` defines a trait alias for multiple field accessors.

# The entire syntax

```
# use structural::structural_alias;
# pub trait SuperTrait{}

structural_alias!{
    pub trait Foo<'a,T:Copy>:SuperTrait
    where
        T:SuperTrait
    {
             a:u32,
        ref  b:T,
        mut  c:i64,
        move d:String,
        mut move e:String,
    }
}

# fn main(){}
```

Outside of the `{...}` the trait syntax is the same as the 
regular one,with the same meaning.

Inside the `{...}` is a list of fields,
each of which get turned into supertraits on `Foo`:

- `     a:u32`:
    Corresponds to the `GetField<TString<(_a,)>,Ty=u32>` shared reference 
    field accessor trait.

- `ref  b:T`
    Corresponds to the `GetField<TString<(_b,)>,Ty=T>` shared reference 
    field accessor trait.

- `mut  c:i64`:
    Corresponds to the `GetFieldMut<TString<(_c,)>,Ty=i64>` mutable reference 
    field accessor trait (which`itself implies `GetField`).

- `move d:String`:
    Corresponds to the `IntoField<TString<(_d,)>,Ty=String>` by value
    field accessor trait (which itself implies `GetField`).

- `mut move e:String`:
    Corresponds to the `IntoFieldMut<TString<(_e,)>,Ty=String>` trait,
    allowing shared,mutable,and by value access to the field.

# Examples

### Defining a Point trait alias

```rust
use structural::{
    structural_alias,
    fp,
    GetFieldExt,
    Structural,
};

use core::{
    cmp::PartialEq,
    fmt::{Debug,Display},
};

structural_alias!{
    trait Point<T>{
        mut move x:T,
        mut move y:T,
    }
}

fn print_point<T,U>(value:&T)
where
    T:Point<U>,
    U:Debug+Display+PartialEq,
{
    // This gets references to the `x` and `y` fields.
    let (x,y)=value.fields(fp!(x,y));
    assert_ne!(x,y);
    println!("x={} y={}",x,y);
}

#[derive(Structural)]
#[struc(access="mut move")]
struct Point3D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
}

#[derive(Structural)]
#[struc(access="mut move")]
struct Rectangle<T>{
    pub x:T,
    pub y:T,
    pub w:T,
    pub h:T,
}

#[derive(Structural)]
#[struc(access="mut move")]
struct Entity{
    pub id:PersonId,
    pub x:f32,
    pub y:f32,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);

# fn main(){

print_point(&Point3D{ x:100, y:200, z:6000 });

print_point(&Rectangle{ x:100, y:200, w:300, h:400 });

print_point(&Entity{ x:100.0, y:200.0, id:PersonId(0xDEAD) });


# }

```

### Defining a trait aliases with all accessibilities

```
use structural::{
    structural_alias,
    fp,
    GetFieldExt,
};

structural_alias!{
    trait Person{
        // shared access (a & reference to the field)
        id:PersonId,
        
        // shared access (a & reference to the field)
        name:String,

        // mutable access (a &mut reference to the field),as well as shared access.
        mut friends:Vec<PersonId>,

        // by value access to the field (as well as shared)
        move candy:Candy,

        // by value access to the field (as well as shared and mutable)
        mut move snack:Snack,
    }
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Seconds(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Candy;

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Snack;

# fn main(){}

```


*/
#[macro_export]
macro_rules! structural_alias{
    ( $($everything:tt)* )=>{
        $crate::structural_alias_impl!{ $($everything)* }
    }
}
