// Copyright 2016 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Memory stuff

use std::mem;

/// A hint as to how this memory will be used.
///
/// The nature of these hints make them very implementation specific. Different drivers on
/// different hardware will handle them differently. Only careful profiling will tell which is the
/// best to use.
#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Usage {
    /// GPU: read + write, CPU: copy. Optimal for render targets.
    GpuOnly,
    /// GPU: read, CPU: none. Optimal for resourced memory.
    Immutable,
    /// GPU: read, CPU: write.
    Dynamic,
    /// GPU: read + write, CPU: as specified.
    Persistent(Access),
    /// GPU: copy, CPU: as specified. Used for staged memory, to be copied back and forth with on-GPU targets.
    CpuOnly(Access),
}

bitflags!(
    /// Memory access
    pub flags Access: u8 {
        /// Read access
        const READ  = 0x1,
        /// Write access
        const WRITE = 0x2,
        /// Full access
        const RW    = 0x3,
    }
);

bitflags!(
    /// Bind flags
    pub flags Bind: u8 {
        /// Can be rendered into.
        const RENDER_TARGET    = 0x1,
        /// Can serve as a depth/stencil target.
        const DEPTH_STENCIL    = 0x2,
        /// Can be bound to the shader for reading.
        const SHADER_RESOURCE  = 0x4,
        /// Can be bound to the shader for writing.
        const UNORDERED_ACCESS = 0x8,
    }
);

/// A service trait used to get the raw data out of strong types.
/// Not meant for public use.
#[doc(hidden)]
pub trait Typed: Sized {
    /// The raw type behind the phantom.
    type Raw;
    /// Crete a new phantom from the raw type.
    fn new(raw: Self::Raw) -> Self;
    /// Get an internal reference to the raw type.
    fn raw(&self) -> &Self::Raw;
}

/// A trait for plain-old-data types.
///
/// A POD type does not have invalid bit patterns and can be safely
/// created from arbitrary bit pattern.
pub unsafe trait Pod {}

macro_rules! impl_pod {
    ( ty = $($ty:ty)* ) => { $( unsafe impl Pod for $ty {} )* };
    ( ar = $($tt:expr)* ) => { $( unsafe impl<T: Pod> Pod for [T; $tt] {} )* };
}

impl_pod! { ty = isize usize i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 }
impl_pod! { ar =
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
}

/// Cast a slice from one POD type to another.
pub fn cast_slice<A: Pod, B: Pod>(slice: &[A]) -> &[B] {
    use std::slice;

    let raw_len = mem::size_of::<A>().wrapping_mul(slice.len());
    let len = raw_len / mem::size_of::<B>();
    assert_eq!(raw_len, mem::size_of::<B>().wrapping_mul(len));
    unsafe {
        slice::from_raw_parts(slice.as_ptr() as *const B, len)
    }
}
