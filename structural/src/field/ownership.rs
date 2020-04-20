///
///
/// # Panics
///
/// Panicking in the `pre_drop` method causes an abort when called from the
/// [`DropFields`] implementation generated by the [delegation macro].
///
/// [`DropFields`]: ./trait.DropFields.html
/// [delegation macro]: ../../macro.unsafe_delegate_structural_with.html
///
mod on_drop;

pub use self::on_drop::{AndDroppedFields, RunDrop, RunDropFields, RunPostDrop, RunPreDrop};

/////////////////////////////////////////////////////////////////////////////////

pub unsafe trait PrePostDropFields {
    #[inline(always)]
    unsafe fn pre_drop(_this: *mut Self) {}

    #[inline(always)]
    unsafe fn post_drop(_this: *mut Self) {}
}

/////////////////////////////////////////////////////////////////////////////////

pub unsafe trait DropFields {
    /// Drops all the fields that were not moved out.
    ///
    unsafe fn drop_fields(&mut self, dropped: DroppedFields);
}

/////////////////////////////////////////////////////////////////////////////////

/// Which fields have been dropped for a type.
///
/// This is used while moving fields out of a value.
#[derive(Debug, Copy, Clone)]
pub struct DroppedFields(u64);

impl DroppedFields {
    #[inline(always)]
    pub const fn new() -> Self {
        DroppedFields(0)
    }

    #[inline(always)]
    pub fn set_dropped(&mut self, bit: DropBit) {
        self.0 |= bit.0;
    }

    #[inline(always)]
    pub const fn is_dropped(&self, bit: DropBit) -> bool {
        (self.0 & bit.0) != 0
    }
}

/////////////////////////////////////////////////////////////////////////////////

/// Represents the index for a field in DroppedFields.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DropBit(u64);

impl DropBit {
    #[inline(always)]
    pub const fn new(bit: u8) -> Self {
        [(); 64][bit as usize];
        DropBit(1 << bit)
    }
}
