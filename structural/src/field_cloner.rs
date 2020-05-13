use crate::{
    field::{DropFields, GetField, GetVariantField, IntoField, IntoVariantField, MovedOutFields},
    TStr,
};

use core_extensions::ConstDefault;

/// Wrapper that emulates by-value access to fields by cloning them.
///
/// This allows using types that only provide shared access to fields
/// (implementing `GetField`/`GetVariantField`)
/// to be passed to functions expecting by-value access to them
/// (requiring `IntoField`/`IntoVariantField`),
/// by cloning those fields.
///
/// # Struct Example
///
/// ```rust
/// use structural::{FieldCloner, Structural, StructuralExt, fp, structural_alias};
///
/// # fn main(){
///
/// let expected = ("what".to_string(), vec![0,1,2]);
///
/// let this = TheStruct{foo: "what".to_string(), bar: vec![0,1,2]};
///
/// // This doesn't compile,because `TheStruct` only provides shared access to the fields,
/// // implementing `GetField` to access both the `foo` and `bar` fields.
/// //
/// // assert_eq!( into_foo_bar(this), expected.clone() );
///
/// assert_eq!( into_foo_bar(FieldCloner(this.clone())), expected.clone() );
///
/// assert_eq!( into_foo_bar(FieldCloner(&this)), expected.clone() );
///
/// assert_eq!( into_foo_bar(FieldCloner(&&&&&this)), expected.clone() );
///
/// # }
///
/// fn into_foo_bar<P, T>(this: P)-> (String, Vec<T>)
/// where
///     P: TypeMove<String, Vec<T>>,
/// {
///     this.into_fields(fp!(foo, bar))
/// }
///
/// #[derive(Structural,Clone)]
/// // Makes this struct only implement `GetField` for the fields,
/// // providing shared access to them.
/// #[struc(access="ref")]
/// struct TheStruct<T, U>{
///     pub foo: T,
///     pub bar: U,
/// }
///
/// structural::structural_alias!{
///     // The same fields as `TheStruct`, with shared and by-value access to the fields.
///     //
///     // This trait isn't implemented by `TheStruct` because it only
///     // provides shared access to the fields.
///     trait TypeMove<T, U>{
///         move foo: T,
///         move bar: U,
///     }
/// }
/// ```
///
/// # Enum Example
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
/// use structural::{FieldCloner, Structural, StructuralExt, fp, structural_alias};
///
/// # fn main(){
///
/// {
///     let expected = Some((vec!["foo","bar"], [0, 1, 2]));
///    
///     let this = TheEnum::Both(vec!["foo","bar"], [0, 1, 2]);
///    
///     // This doesn't compile,because `TheEnum` only provides shared access to the fields,
///     // implementing `GetField` to access both the `foo` and `bar` fields.
///     //
///     // assert_eq!( into_both(Box::new(this)), expected.clone() );
///    
///     assert_eq!( into_both(Box::new(FieldCloner(this.clone()))), expected.clone() );
///    
///     assert_eq!( into_both(Box::new(FieldCloner(&this))), expected.clone() );
///    
///     assert_eq!( into_both(Box::new(FieldCloner(&&&&&this))), expected.clone() );
/// }
/// {
///     let this = TheEnum::Left{left: vec!["foo","bar"]};
///    
///     assert_eq!( into_both(Box::new(FieldCloner(this.clone()))), None );
///    
///     assert_eq!( into_both(Box::new(FieldCloner(&this))), None );
///    
///     assert_eq!( into_both(Box::new(FieldCloner(&&&&&this))), None );
/// }
///
/// # }
///
/// fn into_both<'a,T>(
///     this: Box<dyn TypeMove<Vec<T>, [u32;3]> + 'a>
/// )-> Option<(Vec<T>, [u32;3])> {
///     this.into_fields(fp!(::Both=>0,1))
/// }
///
/// #[derive(Structural, Clone)]
/// // Makes this enum only implement `GetVariantField` for the fields,
/// // providing shared access to them.
/// #[struc(access="ref")]
/// pub enum TheEnum<L, R> {
///     Both(L, R),
///     Left{left: L},
///     Right{right: R},
/// }
///
/// structural::structural_alias!{
///     // The same fields as `TheEnum`, with shared and by-value access to the fields.
///     //
///     // This trait isn't implemented by `TheEnum` because it only
///     // provides shared access to the fields.
///     trait TypeMove<L, R>{
///         move Both(L, R),
///         move Left{left: L},
///         move Right{right: R},
///     }
/// }
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldCloner<T>(pub T);

impl<T> FieldCloner<T> {
    /// Turns a `&FieldCloner<T>` into a `FieldCloner<&T>`.
    ///
    /// # Struct Example
    ///
    /// ```rust
    /// use structural::{FieldCloner, StructuralExt, fp};
    /// use structural::for_examples::{FooBarRef, FooBarMove_SI};
    ///
    /// let this = FieldCloner(FooBarRef{foo: 100, bar: "baz"});
    ///
    /// assert_eq!( into_foo(this.as_ref()), 100 );
    ///
    /// fn into_foo<T, U>(this: impl FooBarMove_SI<T, U>)-> T {
    ///     this.into_field(fp!(foo))
    /// }
    ///
    /// ```
    ///
    /// # Enum Example
    ///
    /// ```rust
    /// use structural::{FieldCloner, StructuralExt, fp};
    /// use structural::for_examples::{EnumRef, EnumMove_SI};
    ///
    /// let this = FieldCloner(EnumRef::Left{left: 10});
    ///
    /// assert_eq!( into_left(this.as_ref()), Some(10) );
    ///
    /// fn into_left(this: impl EnumMove_SI<u32, bool>)-> Option<u32> {
    ///     this.into_field(fp!(::Left.left))
    /// }
    ///
    /// ```
    ///
    #[inline(always)]
    pub fn as_ref(&self) -> FieldCloner<&T> {
        FieldCloner(&self.0)
    }

    /// Turns a `&mut FieldCloner<T>` into a `FieldCloner<&mut T>`.
    ///
    /// # Struct Example
    ///
    /// ```rust
    /// use structural::{FieldCloner, StructuralExt, fp};
    /// use structural::for_examples::{FooBarRef, FooBarMove_SI};
    ///
    /// let this = FieldCloner(FooBarRef{foo: 100, bar: "baz"});
    ///
    /// assert_eq!( into_bar(this.as_ref()), "baz" );
    ///
    /// fn into_bar<T, U>(this: impl FooBarMove_SI<T, U>)-> U {
    ///     this.into_field(fp!(bar))
    /// }
    ///
    /// ```
    ///
    /// # Enum Example
    ///
    /// ```rust
    /// use structural::{FieldCloner, StructuralExt, fp};
    /// use structural::for_examples::{EnumRef, EnumMove_SI};
    ///
    /// let mut this = FieldCloner(EnumRef::Right{right: false});
    ///
    /// assert_eq!( into_right(this.as_mut()), Some(false) );
    ///
    /// fn into_right(this: impl EnumMove_SI<u32, bool>)-> Option<bool> {
    ///     this.into_field(fp!(::Right.right))
    /// }
    ///
    /// ```
    ///
    #[inline(always)]
    pub fn as_mut(&mut self) -> FieldCloner<&mut T> {
        FieldCloner(&mut self.0)
    }

    /// Transforms the wrapped value with the `func` function.
    ///
    ///
    #[inline(always)]
    pub fn map<F, U>(self, f: F) -> FieldCloner<U>
    where
        F: FnOnce(T) -> U,
    {
        FieldCloner(f(self.0))
    }

    /// Calls `func` with `self`,rewrapping its return value in a `FieldCloner<U>`
    #[inline(always)]
    pub fn then<F, U>(self, f: F) -> FieldCloner<U>
    where
        F: FnOnce(Self) -> U,
    {
        FieldCloner(f(self))
    }
}

impl<T: Clone> FieldCloner<&T> {
    /// Maps the wrapped reference into a clone.
    #[inline(always)]
    pub fn cloned(self) -> FieldCloner<T> {
        FieldCloner((*self.0).clone())
    }
}

impl<T: Clone> FieldCloner<&mut T> {
    /// Maps the wrapped mutable reference into a clone.
    #[inline(always)]
    pub fn cloned(self) -> FieldCloner<T> {
        FieldCloner((*self.0).clone())
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<T> ConstDefault for FieldCloner<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = FieldCloner(T::DEFAULT);
}

////////////////////////////////////////////////////////////////////////////////

unsafe impl<T, P> IntoField<P> for FieldCloner<T>
where
    T: GetField<P>,
    T::Ty: Clone,
{
    #[inline(always)]
    fn into_field_(self, path: P) -> Self::Ty {
        self.0.get_field_(path).clone()
    }

    #[inline(always)]
    unsafe fn move_out_field_(&mut self, path: P, _: &mut MovedOutFields) -> Self::Ty {
        self.0.get_field_(path).clone()
    }
}

unsafe impl<T, V, F> IntoVariantField<TStr<V>, F> for FieldCloner<T>
where
    T: GetVariantField<TStr<V>, F>,
    T::Ty: Clone,
{
    #[inline(always)]
    fn into_vfield_(self, variant_name: TStr<V>, field_name: F) -> Option<Self::Ty> {
        match self.0.get_vfield_(variant_name, field_name) {
            Some(x) => Some(x.clone()),
            _ => None,
        }
    }

    #[inline(always)]
    unsafe fn move_out_vfield_(
        &mut self,
        variant_name: TStr<V>,
        field_name: F,
        _: &mut MovedOutFields,
    ) -> Option<Self::Ty> {
        match self.0.get_vfield_(variant_name, field_name) {
            Some(x) => Some(x.clone()),
            _ => None,
        }
    }
}

unsafe impl<T> DropFields for FieldCloner<T> {
    #[inline(always)]
    fn pre_move(&mut self) {}

    #[inline(always)]
    unsafe fn drop_fields(&mut self, _: MovedOutFields) {
        // No field was moved out, so we can just drop Self.
        std::ptr::drop_in_place(self);
    }
}

unsafe_delegate_structural_with! {
    impl[T,] FieldCloner<T>
    where[]

    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;

    GetField { &this.0 }

    GetFieldMut{
        &mut this.0
    }
    as_delegating_raw{
        this as *mut T
    }

    FromStructural {
        constructor = FieldCloner;
    }
}
