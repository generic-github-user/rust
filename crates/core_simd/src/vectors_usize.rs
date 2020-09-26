define_vector! {
    /// Vector of two `usize` values
    #[derive(Eq, Ord, Hash)]
    struct usizex2([usize; 2]);
}

define_vector! {
    /// Vector of four `usize` values
    #[derive(Eq, Ord, Hash)]
    struct usizex4([usize; 4]);
}

define_vector! {
    /// Vector of eight `usize` values
    #[derive(Eq, Ord, Hash)]
    struct usizex8([usize; 8]);
}

#[cfg(target_pointer_width = "32")]
from_transmute_x86! { unsafe usizex4 => __m128i }
#[cfg(target_pointer_width = "32")]
from_transmute_x86! { unsafe usizex8 => __m256i }

#[cfg(target_pointer_width = "64")]
from_transmute_x86! { unsafe usizex2 => __m128i }
#[cfg(target_pointer_width = "64")]
from_transmute_x86! { unsafe usizex4 => __m256i }
//#[cfg(target_pointer_width = "64")]
//from_transmute_x86! { unsafe usizex8 => __m512i }
