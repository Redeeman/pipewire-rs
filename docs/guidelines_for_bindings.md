# Guidelines for writing bindings

## Struct bindings
For raw C structs that require manual memory management, usually by calling a constructor to create it on the heap
and a destructor to free it and its internal allocations, this project uses two different kinds of structs/types that work similar to rusts `str` and `String` types: An owning struct and a ref struct.

A ref struct simply provides high-level bindings for a raw C struct by being a transparent
wrapper around it, so that a pointer to the raw type can be casted to a pointer to the wrapper type.
It does not perform any management for creating or destroying these types, as it does not own the raw struct.
It being a transparent wrapper means that we can take a pointer to some raw C type, cast it to a reference to the ref type,
and return it to the user without needing to set up extra indirection or worrying about memory management.

On the other hand, an owning struct is a wrapper around a pointer to a raw C struct, and usually
has bindings to the constructor function of the raw type as well as a `std::ops::Drop` 
implementation that invokes the C destructor function on the raw type. \
For other methods, it defers to the ref type by providing an `std::ops::Deref` implementation
with the ref type as its target.

In some cases, a ref struct is not needed because references to the type are not used anywhere.
In this case, all functionality can be put into a single owning type.

In other cases, the owning struct needs to be reference-counted so it can be kept alive in other structs that are using it.
To achieve this, it can be split up into an inner and outer type, and the outer type contains an RC containing the inner type.

### Example
In this example, we provide bindings to a `foo` C struct (these are usually provided automatically by a sys crate),
which has a constructor, destructor and a method implemented in C.

```rust
// Usually generated automatically in sys crate
mod sys {
    #[repr(C)]
    pub struct foo {/* ... */}

    extern "C" {
        pub fn foo_new() -> *mut foo;
        pub fn foo_destroy(foo: *mut foo);
        pub fn foo_do_something(foo: *mut foo);
    }
}

/// This struct is a ref struct for [`sys::foo`].
///
/// It is marked `#[repr(transparent)]` so that we can cast directly from a pointer to the C struct to a pointer to the ref struct.
#[repr(transparent)]
struct FooRef(sys::foo);

impl FooRef {
    // Each ref type should implement at least the `as_raw` and `as_raw_ptr` functions,
    // so that users of the bindings can choose to use sys functions, etc. themselves.
    pub fn as_raw(&self) -> &sys::foo {
        &self.0
    }

    pub fn as_raw_ptr(&self) -> *mut sys::foo {
        std::ptr::addr_of!(self.0).cast_mut()
    }

    // Methods on the C type that do require ownership are bound here
    // on the reference type.
    pub fn do_something(&self) {
        unsafe {
            sys::foo_do_something(self.as_raw_ptr());
        }
    }
}

// The owning type contains a NonNull pointer to the raw type, which provides better safety to the internal implementation.
struct Foo {
    ptr: std::ptr::NonNull<sys::foo>,
    // Optionally, this struct may also contain references to other structs it needs to keep alive during its own lifetime.
}

impl Foo {
    // Bindings to constructors are here on the owning type.
    pub fn new() -> Foo {
        // In this case, we assume that `foo_new()` always succeeds.
        // In other cases, if null or errors can be returned, we should
        // return an `Option` or `Result` instead.
        unsafe {
            let raw = sys::foo_new();
            Self::from_raw(raw)
        }
    }

    // Owning types should provide at least `from_raw` and `into_raw` methods to allow conversion
    // between the owning wrapper struct and a pointer to an owned raw struct.
    pub unsafe fn from_raw(raw: *mut sys::foo) -> Self {
        Self {
            ptr: std::ptr::NonNull::new(raw).expect("Provided pointer is null"),
        }
    }

    pub fn into_raw(self) -> *mut sys::foo {
        std::mem::ManuallyDrop::new(self).ptr.as_ptr()
    }
}

// The owning struct implements `Deref` with the ref type as its target,
// as it is a smart pointer that should also give access to the methods of the managed type.
impl std::ops::Deref for Foo {
    type Target = FooRef;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.cast::<FooRef>().as_ref() }
    }
}

impl AsRef<FooRef> for Foo {
    fn as_ref(&self) -> &FooRef {
        &*self
    }
}

// The owning type implements the Drop trait to clean up the raw type automatically.
impl std::ops::Drop for Foo {
    fn drop(&mut self) {
        unsafe {
            sys::foo_destroy(self.as_raw_ptr());
        }
    }
}
```

## Enums

Different from Rust, C-style enums are simply named integers and do not offer the same type safety.
When writing bindings for these enums in Rust, do not use Rust `enum`s, instead use a Tuple struct wrapping the raw integer type and create public constants for each enum variant.

This has the folling advantages:
- Conversions are zero-cost, instead of requiring big match statements mapping the raw integers to enum variants
  and the other way around
- Unknown variants (such as new additions to the C library) don't need to be handled, they are represented as any other variant
  and are simply missing the associated constant on the wrapper type
- As a rust enum would be marked `#[non_exhaustive]` to allow for furture additions anyways, the tuple wrapper type
  "feels" almost the same as an enum, a user can create and match over any known variants like with a real enum

### Example

C Header:
```c
enum foo {
    foo_a, foo_b
}
```

Rust bindings:
```rust
mod sys {
    pub type foo = ::std::os::raw::c_uint;
    pub const foo_a: foo = 0;
    pub const foo_b: foo = 1;
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Foo(sys::foo);

impl Foo {
    pub const A: Self = Self(sys::foo_a);
    pub const B: Self = Self(sys::foo_b);

    pub fn from_raw(raw: sys::foo) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> sys::foo {
        self.0
    }
}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("Foo::{}", match *self {
            Self::A => "A",
            Self::B => "B",
            _ => "Unknown",
        });
        f.write_str(&name)
    }
}
```
