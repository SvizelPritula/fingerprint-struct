use crate::Fingerprint;
use digest::Update;

macro_rules! impl_method {
    ($type: ty, $member: ident ($($arg: expr),*)) => {
        impl Fingerprint for $type {
            #[inline(always)]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                self.$member($($arg),*).fingerprint(hasher);
            }
        }
    };
}

macro_rules! impl_inner {
    ($type: ty) => {
        impl<T: Fingerprint> Fingerprint for $type {
            #[inline(always)]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                self.0.fingerprint(hasher);
            }
        }
    };
}

macro_rules! impl_deref {
    ($type: ty) => {
        impl<T: Fingerprint> Fingerprint for $type {
            #[inline(always)]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                (**self).fingerprint(hasher);
            }
        }
    };
}

impl_deref!(&T);
impl_deref!(&mut T);
#[cfg(feature = "alloc")]
impl_deref!(alloc::boxed::Box<T>);
#[cfg(feature = "alloc")]
impl_deref!(alloc::rc::Rc<T>);
#[cfg(feature = "alloc")]
impl_deref!(alloc::sync::Arc<T>);

#[cfg(feature = "alloc")]
impl<'a, T: Fingerprint + Clone> Fingerprint for alloc::borrow::Cow<'a, T> {
    #[inline(always)]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        (**self).fingerprint(hasher);
    }
}

impl<T: Fingerprint + Copy> Fingerprint for core::cell::Cell<T> {
    #[inline(always)]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        self.get().fingerprint(hasher);
    }
}

macro_rules! impl_primitive {
    ($type: ty) => {
        impl Fingerprint for $type {
            #[inline(always)]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                hasher.update(&self.to_le_bytes());
            }
        }
    };
}

impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(i128);
impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(u128);
impl_primitive!(f32);
impl_primitive!(f64);

macro_rules! impl_through_cast {
    ($type: ty, $cast: ty) => {
        impl Fingerprint for $type {
            #[inline(always)]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                (*self as $cast).fingerprint(hasher);
            }
        }
    };
}

impl_through_cast!(char, u32);
impl_through_cast!(bool, u8);

macro_rules! impl_through_from {
    ($type: ty, $into: ty) => {
        impl Fingerprint for $type {
            #[inline(always)]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                <$into>::from(*self).fingerprint(hasher);
            }
        }
    };
}

impl_through_from!(core::num::NonZeroI8, i8);
impl_through_from!(core::num::NonZeroI16, i16);
impl_through_from!(core::num::NonZeroI32, i32);
impl_through_from!(core::num::NonZeroI64, i64);
impl_through_from!(core::num::NonZeroI128, i128);
impl_through_from!(core::num::NonZeroIsize, isize);
impl_through_from!(core::num::NonZeroU8, u8);
impl_through_from!(core::num::NonZeroU16, u16);
impl_through_from!(core::num::NonZeroU32, u32);
impl_through_from!(core::num::NonZeroU64, u64);
impl_through_from!(core::num::NonZeroUsize, usize);

impl_method!(
    core::sync::atomic::AtomicBool,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicI8,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicI16,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicI32,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicI64,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicIsize,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicU8,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicU16,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicU32,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicU64,
    load(core::sync::atomic::Ordering::Relaxed)
);
impl_method!(
    core::sync::atomic::AtomicUsize,
    load(core::sync::atomic::Ordering::Relaxed)
);

impl Fingerprint for usize {
    // Encodes usize as a variable size integer.
    // This makes the hash consistant not only between 32 and 64-bit architectures, but also among
    // as of yet uninvented architectures with even larger pointers. The encoding is based on
    // protobuf varints. Each byte contains a base 128 digit, with the MSB being a continuation
    // flag. The digits are arranged in little endian order. See the protobuf documentation or
    // https://en.wikipedia.org/wiki/Variable-length_quantity for more details.
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        let mut rest = *self;

        loop {
            let digit = rest & ((1 << 7) - 1);
            rest >>= 7;

            let digit = digit as u8;
            let digit = digit | if rest > 0 { 1 << 7 } else { 0 };

            digit.fingerprint(hasher);

            if rest == 0 {
                break;
            }
        }
    }
}

impl Fingerprint for isize {
    // Encodes isize as a variable size integer.
    // This encoding as also based on protobuf varints. It uses "zigzag" encoding, which encodes
    // 0 to 0, -1 to 1, 1 to 2, -2 to 3, 2 to 4 and so on. See the protobuf documentation or
    // https://en.wikipedia.org/wiki/Variable-length_quantity#Zigzag_encoding for more details.
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        let value = *self;
        let value = (value << 1) ^ (value >> (isize::BITS - 1));
        let value = value as usize;
        value.fingerprint(hasher);
    }
}

impl<T: Fingerprint, const N: usize> Fingerprint for [T; N] {
    #[inline(always)]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        for i in self {
            i.fingerprint(hasher);
        }
    }
}

macro_rules! impl_tuple {
    ($($num: tt: $name: ident)*) => {
        impl<$($name: Fingerprint),*> Fingerprint for ($($name,)*) {
            #[inline(always)]
            #[allow(unused_variables)] // In case of the empty stuct
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                $(
                    self.$num.fingerprint(hasher);
                )*
            }
        }
    };
}

impl_tuple!();
impl_tuple!(0: T0);
impl_tuple!(0: T0 1: T1);
impl_tuple!(0: T0 1: T1 2: T2);
impl_tuple!(0: T0 1: T1 2: T2 3: T3);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9 10: T10);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9 10: T10 11: T11);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9 10: T10 11: T11 12: T12);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9 10: T10 11: T11 12: T12 13: T13);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9 10: T10 11: T11 12: T12 13: T13 14: T14);
impl_tuple!(0: T0 1: T1 2: T2 3: T3 4: T4 5: T5 6: T6 7: T7 8: T8 9: T9 10: T10 11: T11 12: T12 13: T13 14: T14 15: T15);

macro_rules! impl_string_like {
    ($type: ty) => {
        impl Fingerprint for $type {
            #[inline]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                self.len().fingerprint(hasher);
                hasher.update(self.as_bytes());
            }
        }
    };
}

impl_string_like!(str);
#[cfg(feature = "alloc")]
impl_string_like!(alloc::string::String);

#[cfg(feature = "std")]
macro_rules! impl_cstring_like {
    ($type: ty, $func: ident) => {
        impl Fingerprint for $type {
            #[inline]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                hasher.update(self.to_bytes_with_nul());
            }
        }
    };
}

#[cfg(feature = "std")]
impl_cstring_like!(std::ffi::CStr, to_bytes);
#[cfg(feature = "std")]
impl_cstring_like!(std::ffi::CString, to_bytes);

impl<T: Fingerprint> Fingerprint for Option<T> {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        match self {
            Some(value) => {
                0u8.fingerprint(hasher);
                value.fingerprint(hasher);
            }
            None => 1u8.fingerprint(hasher),
        }
    }
}

impl<T: Fingerprint, E: Fingerprint> Fingerprint for Result<T, E> {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        match self {
            Ok(value) => {
                0u8.fingerprint(hasher);
                value.fingerprint(hasher);
            }
            Err(value) => {
                1u8.fingerprint(hasher);
                value.fingerprint(hasher);
            }
        }
    }
}

impl<T> Fingerprint for core::marker::PhantomData<T> {
    #[inline]
    fn fingerprint<U: Update>(&self, _hasher: &mut U) {}
}

macro_rules! impl_ordered_seq {
    ($type: ty $(,$bound: tt)?) => {
        impl<T: Fingerprint $(+ $bound)?> Fingerprint for $type {
            #[inline]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                self.len().fingerprint(hasher);

                for element in self.iter() {
                    element.fingerprint(hasher);
                }
            }
        }
    };
}

impl_ordered_seq!([T]);
#[cfg(feature = "alloc")]
impl_ordered_seq!(alloc::vec::Vec<T>);
#[cfg(feature = "alloc")]
impl_ordered_seq!(alloc::collections::BTreeSet<T>, Ord);
#[cfg(feature = "alloc")]
impl_ordered_seq!(alloc::collections::LinkedList<T>);
#[cfg(feature = "alloc")]
impl_ordered_seq!(alloc::collections::VecDeque<T>);

#[cfg(feature = "alloc")]
macro_rules! impl_unordered_seq {
    ($type: ty) => {
        impl<T: Fingerprint + Ord> Fingerprint for $type {
            #[inline]
            fn fingerprint<U: Update>(&self, hasher: &mut U) {
                let mut vec: alloc::vec::Vec<&T> = self.iter().collect();
                vec.sort();

                vec.fingerprint(hasher);
            }
        }
    };
}

#[cfg(feature = "alloc")]
impl_unordered_seq!(alloc::collections::BinaryHeap<T>);
#[cfg(feature = "std")]
impl_unordered_seq!(std::collections::HashSet<T>);

impl<T: Fingerprint> Fingerprint for core::ops::Range<T> {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        self.start.fingerprint(hasher);
        self.end.fingerprint(hasher);
    }
}

impl<T: Fingerprint> Fingerprint for core::ops::RangeInclusive<T> {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        self.start().fingerprint(hasher);
        self.end().fingerprint(hasher);
    }
}

impl<T: Fingerprint> Fingerprint for core::ops::Bound<T> {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        match self {
            core::ops::Bound::Included(bound) => {
                0u8.fingerprint(hasher);
                bound.fingerprint(hasher);
            }
            core::ops::Bound::Excluded(bound) => {
                1u8.fingerprint(hasher);
                bound.fingerprint(hasher);
            }
            core::ops::Bound::Unbounded => {
                2u8.fingerprint(hasher);
            }
        }
    }
}

impl_method!(core::time::Duration, as_nanos());

#[cfg(feature = "std")]
impl Fingerprint for std::time::SystemTime {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        match self.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                0u8.fingerprint(hasher);
                duration.fingerprint(hasher);
            }
            Err(error) => {
                1u8.fingerprint(hasher);
                error.duration().fingerprint(hasher);
            }
        }
    }
}

#[cfg(feature = "std")]
impl_method!(std::net::Ipv4Addr, octets());
#[cfg(feature = "std")]
impl_method!(std::net::Ipv6Addr, octets());

#[cfg(feature = "std")]
impl Fingerprint for std::net::IpAddr {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        match self {
            std::net::IpAddr::V4(address) => {
                4u8.fingerprint(hasher);
                address.fingerprint(hasher);
            }
            std::net::IpAddr::V6(address) => {
                6u8.fingerprint(hasher);
                address.fingerprint(hasher);
            }
        }
    }
}

#[cfg(feature = "std")]
impl Fingerprint for std::net::SocketAddrV4 {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        self.ip().fingerprint(hasher);
        self.port().fingerprint(hasher);
    }
}

#[cfg(feature = "std")]
impl Fingerprint for std::net::SocketAddrV6 {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        self.ip().fingerprint(hasher);
        self.port().fingerprint(hasher);
    }
}

#[cfg(feature = "std")]
impl Fingerprint for std::net::SocketAddr {
    #[inline]
    fn fingerprint<U: Update>(&self, hasher: &mut U) {
        match self {
            std::net::SocketAddr::V4(address) => {
                4u8.fingerprint(hasher);
                address.fingerprint(hasher);
            }
            std::net::SocketAddr::V6(address) => {
                6u8.fingerprint(hasher);
                address.fingerprint(hasher);
            }
        }
    }
}

impl_inner!(core::num::Wrapping<T>);
impl_inner!(core::cmp::Reverse<T>);
