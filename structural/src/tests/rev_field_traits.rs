use crate::{
    for_examples::Variants, fp, path::UniquePaths, FieldPathSet, NestedFieldPath,
    NestedFieldPathSet, StructuralExt, TS,
};

#[test]
fn nested_field_path_set() {
    {
        let mut tuple = ((), (), (), ((), (), ((), (101,))));

        let nested: NestedFieldPathSet<
            NestedFieldPath<(TS!(3), TS!(2))>,
            (NestedFieldPath<(TS!(1), TS!(0))>,),
            UniquePaths,
        > = fp!(3.2=>1.0);

        assert_eq!(tuple.field_(nested), &101);
        assert_eq!(tuple.fields(nested), (&101,));
        assert_eq!(tuple.cloned_fields(nested), (101,));
        assert_eq!(tuple.field_mut(nested), &mut 101);
        assert_eq!(tuple.fields_mut(nested), (&mut 101,));
        assert_eq!(tuple.into_field(nested), 101);
    }
}

#[test]
fn field_path_set() {
    {
        let mut tuple = ((), (), (), ((), (), ((), (101,))));

        let nested: FieldPathSet<
            (NestedFieldPath<(TS!(3), TS!(2), TS!(1), TS!(0))>,),
            UniquePaths,
        > = fp!(3.2.1.0).into_set();

        assert_eq!(tuple.field_(nested), &101);
        assert_eq!(tuple.fields(nested), (&101,));
        assert_eq!(tuple.cloned_fields(nested), (101,));
        assert_eq!(tuple.field_mut(nested), &mut 101);
        assert_eq!(tuple.fields_mut(nested), (&mut 101,));
        assert_eq!(tuple.into_field(nested), 101);
    }
}

#[test]
fn tstr_path() {
    let mut array = [3, 5, 8, 13, 21];
    {
        let path = ts!(1);
        assert_eq!(array.field_(path), &5);
        assert_eq!(array.fields(path), (&5,));
        assert_eq!(array.cloned_fields(path), (5,));
        assert_eq!(array.field_mut(path), &mut 5);
        assert_eq!(array.fields_mut(path), (&mut 5,));
        assert_eq!(array.into_field(path), 5);
    }
    {
        let path = ts!(4);
        assert_eq!(array.field_(path), &21);
        assert_eq!(array.fields(path), (&21,));
        assert_eq!(array.cloned_fields(path), (21,));
        assert_eq!(array.field_mut(path), &mut 21);
        assert_eq!(array.fields_mut(path), (&mut 21,));
        assert_eq!(array.into_field(path), 21);
    }
}

#[test]
fn nested_field_path() {
    let mut tuple = ((), (101, Some(444)));
    {
        let nested = fp!(1.0);

        assert_eq!(tuple.field_(nested), &101);
        assert_eq!(tuple.fields(nested), (&101,));
        assert_eq!(tuple.cloned_fields(nested), (101,));
        assert_eq!(tuple.field_mut(nested), &mut 101);
        assert_eq!(tuple.fields_mut(nested), (&mut 101,));
        assert_eq!(tuple.into_field(nested), 101);
    }
    {
        let nested = fp!(1.1::Some.0);

        assert_eq!(tuple.field_(nested), Some(&444));
        assert_eq!(tuple.fields(nested), (Some(&444),));
        assert_eq!(tuple.cloned_fields(nested), (Some(444),));
        assert_eq!(tuple.field_mut(nested), Some(&mut 444));
        assert_eq!(tuple.fields_mut(nested), (Some(&mut 444),));
        assert_eq!(tuple.into_field(nested), Some(444));
    }
    {
        let nested = fp!(1.1::None);

        assert_eq!(tuple.field_(nested), None);
        assert_eq!(tuple.fields(nested), (None,));
        assert_eq!(tuple.cloned_fields(nested), (None,));
        assert_eq!(tuple.field_mut(nested), None);
        assert_eq!(tuple.fields_mut(nested), (None,));
        assert_eq!(tuple.into_field(nested), None);
    }
}

#[test]
fn variant_field() {
    let mut foo = Variants::Foo(13, 21);
    {
        let path = fp!(::Foo.0);

        assert_eq!(foo.field_(path), Some(&13));
        assert_eq!(foo.fields(path), (Some(&13),));
        assert_eq!(foo.cloned_fields(path), (Some(13),));
        assert_eq!(foo.field_mut(path), Some(&mut 13));
        assert_eq!(foo.fields_mut(path), (Some(&mut 13),));
        assert_eq!(foo.into_field(path), Some(13));
    }
    {
        let path = fp!(::Boom.a);

        assert_eq!(foo.field_(path), None);
        assert_eq!(foo.fields(path), (None,));
        assert_eq!(foo.cloned_fields(path), (None,));
        assert_eq!(foo.field_mut(path), None);
        assert_eq!(foo.fields_mut(path), (None,));
        assert_eq!(foo.into_field(path), None);
    }
}

#[test]
fn variant_name() {
    use crate::enums::VariantProxy;

    let mut foo = Variants::Foo(13, 21);
    let mut foo_b = Variants::Foo(13, 21);

    unsafe {
        let path = fp!(::Foo);
        let vname = ts!(Foo);

        assert_eq!(
            foo.field_(path),
            Some(VariantProxy::from_ref(&foo_b, vname))
        );
        assert_eq!(
            foo.fields(path),
            (Some(VariantProxy::from_ref(&foo_b, vname)),)
        );
        assert_eq!(
            foo.cloned_fields(path),
            (Some(VariantProxy::new(foo_b, vname)),)
        );
        assert_eq!(
            foo.field_mut(path),
            Some(VariantProxy::from_mut(&mut foo_b, vname))
        );
        assert_eq!(
            foo.fields_mut(path),
            (Some(VariantProxy::from_mut(&mut foo_b, vname)),)
        );
        assert_eq!(foo.into_field(path), Some(VariantProxy::new(foo_b, vname)));
    }
    {
        let path = fp!(::Boom);

        assert_eq!(foo.field_(path), None);
        assert_eq!(foo.fields(path), (None,));
        assert_eq!(foo.cloned_fields(path), (None,));
        assert_eq!(foo.field_mut(path), None);
        assert_eq!(foo.fields_mut(path), (None,));
        assert_eq!(foo.into_field(path), None);
    }
}
