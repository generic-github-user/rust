define_vector! {
    /// Vector of two `isize` values
    #[derive(Eq, Ord, Hash)]
    struct isizex2([isize; 2]);
}

define_vector! {
    /// Vector of four `isize` values
    #[derive(Eq, Ord, Hash)]
    struct isizex4([isize; 4]);
}

define_vector! {
    /// Vector of eight `isize` values
    #[derive(Eq, Ord, Hash)]
    struct isizex8([isize; 8]);
}

#[cfg(target_pointer_width = "32")]
from_transmute_x86! { unsafe isizex4 => __m128i }
#[cfg(target_pointer_width = "32")]
from_transmute_x86! { unsafe isizex8 => __m256i }

#[cfg(target_pointer_width = "64")]
from_transmute_x86! { unsafe isizex2 => __m128i }
#[cfg(target_pointer_width = "64")]
from_transmute_x86! { unsafe isizex4 => __m256i }
//#[cfg(target_pointer_width = "64")]
//from_transmute_x86! { unsafe isizex8 => __m512i }
