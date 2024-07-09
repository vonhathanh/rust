use std::{
    borrow::Borrow,
    marker::{PhantomData, PhantomPinned},
};

use super::{header::Header, EMPTY_STRING_CODE};
use bytes::{BufMut, Bytes, BytesMut};

pub trait Encodable {
    /// Encodes the type into the `out` buffer
    fn encode(&self, out: &mut dyn BufMut);

    /// Returns the length of the encoding of this type in bytes
    /// The default implementation computes this by encoding the type.
    /// When possible, we recommend implementers overeride this with a
    /// specialized implementation
    #[inline]
    fn length(&self) -> usize {
        let mut out = Vec::new();
        self.encode(&mut out);
        out.len()
    }
}

// The existence of this function makes the compiler catch if the Encodable
// trait is "object-safe" or not.
fn _assert_trait_object(_b: &dyn Encodable) {}

/// Defines the max length of an Encodable type as a const generic
/// # Safety
/// An invalid value can cause the encoder to panic
pub unsafe trait MaxEncodedLength<const LEN: usize>: Encodable {}

/// Defines the max length of an [`Encodable`] type as an associated constant.
///
/// # Safety
///
/// An invalid value can cause the encoder to panic.
pub unsafe trait MaxEncodedLenAssoc: Encodable {
    /// The maximum length.
    const LEN: usize;
}

/// Implement [`MaxEncodedLength`] and [`MaxEncodedLenAssoc`] for a type
/// # Safety
/// An invalid value can cause the encoder to panic
#[macro_export]
macro_rules! impl_max_encoded_len {
    ($t:ty, $len:expr) => {
        unsafe impl $crate::rlp::encode::MaxEncodedLength<{ $len }> for $t {}
        unsafe impl $crate::rlp::encode::MaxEncodedLenAssoc for $t {
            const LEN: usize = $len;
        }
    };
}

macro_rules! to_be_bytes_trimmed {
    ($be:ident, $x: expr) => {{
        $be = $x.to_be_bytes();
        &$be[($x.leading_zeros() / 8) as usize..]
    }};
}
pub(crate) use to_be_bytes_trimmed;

/// Determine the length in bytes of the length prefix of an RLP item.
#[inline]
pub const fn length_of_length(payload_length: usize) -> usize {
    if payload_length < 56 {
        1
    } else {
        1 + (usize::BITS as usize / 8) - payload_length.leading_zeros() as usize / 8
    }
}

impl Encodable for [u8] {
    fn encode(&self, out: &mut dyn BufMut) {
        if self.len() != 1 || self[0] >= EMPTY_STRING_CODE {
            Header {
                list: false,
                payload_length: self.len(),
            }
            .encode(out);
        }
        out.put_slice(self);
    }

    #[inline]
    fn length(&self) -> usize {
        let mut len = self.len();
        if len != 1 || self[0] >= EMPTY_STRING_CODE {
            len += length_of_length(len);
        }
        len
    }
}

impl<T: ?Sized> Encodable for PhantomData<T> {
    #[inline]
    fn length(&self) -> usize {
        0
    }

    #[inline]
    fn encode(&self, _out: &mut dyn BufMut) {}
}

impl Encodable for PhantomPinned {
    #[inline]
    fn length(&self) -> usize {
        0
    }

    #[inline]
    fn encode(&self, _out: &mut dyn BufMut) {}
}

impl<const N: usize> Encodable for [u8; N] {
    #[inline]
    fn length(&self) -> usize {
        self[..].length()
    }

    fn encode(&self, out: &mut dyn BufMut) {
        self[..].encode(out)
    }
}

unsafe impl<const N: usize> MaxEncodedLenAssoc for [u8; N] {
    const LEN: usize = N + length_of_length(N);
}

impl Encodable for str {
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_bytes().encode(out)
    }

    #[inline]
    fn length(&self) -> usize {
        self.as_bytes().length()
    }
}

impl Encodable for bool {
    fn encode(&self, out: &mut dyn BufMut) {
        out.put_u8(if *self { 1 } else { EMPTY_STRING_CODE })
    }

    #[inline]
    fn length(&self) -> usize {
        1
    }
}

impl_max_encoded_len!(bool, <u8 as MaxEncodedLenAssoc>::LEN);

macro_rules! uint_impl {
    ($($t:ty),+ $(,)?) => {$(
        impl Encodable for $t {
            #[inline]
            fn length(&self) -> usize {
                let x= *self;
                if x < EMPTY_STRING_CODE as $t {
                    1
                } else {
                    1 + (<$t>::BITS as usize / 8) - (x.leading_zeros() as usize / 8)
                }
            }

            #[inline]
            fn encode(&self, out: &mut dyn BufMut) {
                let x = *self;
                if x == 0 {
                    out.put_u8(EMPTY_STRING_CODE);
                } else if x < EMPTY_STRING_CODE as $t {
                    out.put_u8(x as u8);
                } else {
                    let be;
                    let be = to_be_bytes_trimmed!(be, x);
                    out.put_u8(EMPTY_STRING_CODE + be.len() as u8);
                    out.put_slice(be);
                }
            }
        }

        impl_max_encoded_len!($t, {
            let bytes = <$t>::BITS as usize / 8;
            bytes + length_of_length(bytes)
        });
    )+};
}

uint_impl!(u8, u16, u32, u64, usize, u128);

impl<T: Encodable> Encodable for Vec<T> {
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        encode_list(self, out)
    }

    #[inline]
    fn length(&self) -> usize {
        list_length(self)
    }
}

/// Calculate the length of a list.
#[inline]
pub fn list_length<B, T>(list: &[B]) -> usize
where
    B: Borrow<T>,
    T: ?Sized + Encodable,
{
    let payload_length = rlp_list_header(list).payload_length;
    payload_length + length_of_length(payload_length)
}

#[inline]
fn rlp_list_header<B, T>(list: &[B]) -> Header
where
    B: Borrow<T>,
    T: ?Sized + Encodable,
{
    let mut h = Header{ list: true, payload_length: 0 };
    for value in list {
        h.payload_length += value.borrow().length();
    }
    h
}

macro_rules! deref_impl {
    ($($(#[$attr:meta])* [$($gen:tt)*] $t:ty),+ $(,)?) => {$(
        $(#[$attr])*
        impl<$($gen)*> Encodable for $t {
            #[inline]
            fn length(&self) -> usize {
                (**self).length()
            }

            #[inline]
            fn encode(&self, out: &mut dyn BufMut) {
                (**self).encode(out)
            }
        }
    )+};
}


