use crate::{
    field::ownership::PrePostDropFields,
    test_utils::{FixedArray, PushOnDrop},
    Structural, StructuralExt,
};

use std_::cell::RefCell;

#[derive(Structural)]
#[struc(pre_post_drop_fields)]
struct PrePostStructA<'a> {
    cell: &'a RefCell<FixedArray>,
    pub a: PushOnDrop<'a, u32>,
    b: PushOnDrop<'a, u32>,
    pub c: PushOnDrop<'a, u32>,
    d: PushOnDrop<'a, u32>,
    pub e: PushOnDrop<'a, u32>,
    f: PushOnDrop<'a, u32>,
    pub g: PushOnDrop<'a, u32>,
}

unsafe impl<'a> PrePostDropFields for PrePostStructA<'a> {
    unsafe fn pre_drop(this: *mut Self) {
        (*this).cell.borrow_mut().push(254);
    }

    unsafe fn post_drop(this: *mut Self) {
        (*this).cell.borrow_mut().push(255);
    }
}

macro_rules! pre_post_drop_test {
    (
        type_name=$type:ident,
        constructor($($constructor:tt)*)
        post_constructor(|$cell:ident,$param:ident|$post_constructor:expr)
        $(
            variant=$variant:ident,
            post_method=$unwrap:ident,
        )?
        before($($before:expr),*)
        after($($after:expr),*)
    ) => (
        let arr=RefCell::new(FixedArray::new());
        fn make_pps($cell:&RefCell<FixedArray>)->$type<'_>{
            $cell.borrow_mut().clear();
            assert!($cell.borrow().as_slice().is_empty());
            // $()
            let $param=$($constructor)* {
                cell: $cell,
                a: PushOnDrop::new(3,$cell,1),
                b: PushOnDrop::new(5,$cell,2),
                c: PushOnDrop::new(8,$cell,3),
                d: PushOnDrop::new(13,$cell,4),
                e: PushOnDrop::new(21,$cell,5),
                f: PushOnDrop::new(34,$cell,6),
                g: PushOnDrop::new(55,$cell,7),
            };
            $post_constructor
        }

        {
            let this=make_pps(&arr);
            let (a,c)=this.into_fields(fp!($(::$variant=>)? a,c)) $(.$unwrap())? ;

            assert_eq!(&arr.borrow()[..], [$($before,)* 254,5,7,2,4,6,255 $(,$after)*]);
            assert_eq!(a.into_inner(), 3);
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,5,7,2,4,6,255 $(,$after)*,1]);
            assert_eq!(c.into_inner(), 8);
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,5,7,2,4,6,255 $(,$after)*,1,3]);
        }
        {
            let this=make_pps(&arr);
            let (c,e)=this.into_fields(fp!($(::$variant=>)? c,e)) $(.$unwrap())? ;
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,1,7,2,4,6,255 $(,$after)*]);
            assert_eq!(c.into_inner(), 8);
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,1,7,2,4,6,255 $(,$after)*,3]);
            assert_eq!(e.into_inner(), 21);
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,1,7,2,4,6,255 $(,$after)*,3,5]);
        }
        {
            let this=make_pps(&arr);
            let (e,g)=this.into_fields(fp!($(::$variant=>)? e,g)) $(.$unwrap())? ;
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,1,3,2,4,6,255 $(,$after)*]);
            assert_eq!(e.into_inner(), 21);
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,1,3,2,4,6,255 $(,$after)*,5]);
            assert_eq!(g.into_inner(), 55);
            assert_eq!(&arr.borrow()[..], [$($before,)* 254,1,3,2,4,6,255 $(,$after)*,5,7]);
        }
    )
}

#[test]
fn drop_order_struct() {
    pre_post_drop_test! {
        type_name=PrePostStructA,
        constructor(PrePostStructA)
        post_constructor(|_arr,a|a)
        before()
        after()
    }
}

////////////////////////////////////////

#[derive(Structural)]
#[struc(pre_post_drop_fields)]
enum PrePostEnum<'a> {
    Variant {
        #[struc(not_public)]
        cell: &'a RefCell<FixedArray>,

        a: PushOnDrop<'a, u32>,

        #[struc(not_public)]
        b: PushOnDrop<'a, u32>,

        c: PushOnDrop<'a, u32>,

        #[struc(not_public)]
        d: PushOnDrop<'a, u32>,

        e: PushOnDrop<'a, u32>,

        #[struc(not_public)]
        f: PushOnDrop<'a, u32>,

        g: PushOnDrop<'a, u32>,
    },
}

unsafe impl<'a> PrePostDropFields for PrePostEnum<'a> {
    unsafe fn pre_drop(this: *mut Self) {
        let PrePostEnum::Variant { ref mut cell, .. } = *this;
        cell.borrow_mut().push(254);
    }

    unsafe fn post_drop(this: *mut Self) {
        let PrePostEnum::Variant { ref mut cell, .. } = *this;
        cell.borrow_mut().push(255);
    }
}

#[test]
fn drop_order_enum() {
    pre_post_drop_test! {
        type_name=PrePostEnum,
        constructor(PrePostEnum::Variant)
        post_constructor(|_arr,a|a)
        variant=Variant,
        post_method=unwrap,
        before()
        after()
    }
}

////////////////////////////////////////

#[derive(Structural)]
#[struc(pre_post_drop_fields)]
struct PrePostStructDeleg<'a, T> {
    cell: &'a RefCell<FixedArray>,
    #[struc(delegate_to)]
    value: T,
    f0: PushOnDrop<'a, u32>,
    f1: PushOnDrop<'a, u32>,
}

unsafe impl<'a, T> PrePostDropFields for PrePostStructDeleg<'a, T> {
    unsafe fn pre_drop(this: *mut Self) {
        let Self { ref mut cell, .. } = *this;
        cell.borrow_mut().push(250);
    }

    unsafe fn post_drop(this: *mut Self) {
        let Self { ref mut cell, .. } = *this;
        cell.borrow_mut().push(251);
    }
}

#[test]
fn drop_order_derive_delegation() {
    type Alias<'a> = PrePostStructDeleg<'a, PrePostStructA<'a>>;

    pre_post_drop_test! {
        type_name=Alias,
        constructor(PrePostStructA)
        post_constructor(|cell,value| {
            PrePostStructDeleg {
                cell,
                value,
                f0: PushOnDrop::new(0,cell,150),
                f1: PushOnDrop::new(0,cell,151),
            }
        })
        before(250,150,151)
        after(251)
    }
}

////////////////////////////////////////

struct PrePostStructDelegMacro<'a, T> {
    cell: &'a RefCell<FixedArray>,
    value: T,
    f0: PushOnDrop<'a, u32>,
    f1: PushOnDrop<'a, u32>,
}

unsafe_delegate_structural_with! {
    // You must write a trailing comma.
    impl['a,T,] PrePostStructDelegMacro<'a,T>
    where[]
    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;

    GetField { &this.value }

    GetFieldMut { &mut this.value }
    as_delegating_raw{ &mut (*this).value as *mut T }

    IntoField{ this.value }
    move_out_field{ &mut this.value }

    DropFields = {
        dropped_fields[cell f0 f1]
        pre_post_drop_fields=true;
    }
}

unsafe impl<'a, T> PrePostDropFields for PrePostStructDelegMacro<'a, T> {
    unsafe fn pre_drop(this: *mut Self) {
        let Self { ref mut cell, .. } = *this;
        cell.borrow_mut().push(100);
    }

    unsafe fn post_drop(this: *mut Self) {
        let Self { ref mut cell, .. } = *this;
        cell.borrow_mut().push(110);
    }
}

#[test]
fn drop_order_delegation_macro() {
    type Alias<'a> = PrePostStructDelegMacro<'a, PrePostStructA<'a>>;

    pre_post_drop_test! {
        type_name=Alias,
        constructor(PrePostStructA)
        post_constructor(|cell,value| {
            PrePostStructDelegMacro {
                cell,
                value,
                f0: PushOnDrop::new(0,cell,150),
                f1: PushOnDrop::new(0,cell,151),
            }
        })
        before(100,150,151)
        after(110)
    }
}
