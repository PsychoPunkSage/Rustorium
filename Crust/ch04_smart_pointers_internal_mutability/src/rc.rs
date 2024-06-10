// Single threaded reference counting pointer
// Similar to RefCell BUT does not provide Mutability... want mutability? use `Cell` ot `Refcell` inside Rc.
// Keep track on no. of shared references and it Drops the value once the last reference gets out of scope.

use crate::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
pub struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

/*
PhantomData::
    - Tells that when we `drop` an `Rc` an `RcInner<T>` might be dropped. <Check that>
*/

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        Rc {
            // Consumes `Box` and returns a Pointer to it.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last `Rc` goes out of scope.
        // For now we have an Rc, therefore Box has not been deallocated, so `deref` is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();

        if count == 1 {
            drop(inner);
            let _ = unsafe { drop(Box::from_raw(self.inner.as_ptr())) };
        } else {
            inner.refcount.set(count - 1);
        }
    }
}
