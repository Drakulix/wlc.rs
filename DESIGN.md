# Design Decisions for this Library

## Ownership

Rust embraces the concept of **Ownership** and **Borrowing**. In Rust-wording
wlc does *borrow* you *references* of `View`s and `Output`s, while the library
itself *owns* them.

Since the compiler may not know, when a `View` or `Output` may get deallocated, the
lifetime of `View`s and `Output`s passed to the user is limited to the current
callback of wlc.

There is no way I know of to reflect this relationship any better, although we
know a `View` or `Output` will get deallocated after the `view/output_destroyed`
callback.

## Weak References

Wlc however lets you still store View and Output handles and gracefully handles
`NULL`-Pointers of already deallocated Views and Outputs, still expecting you to
correctly manage them.

Rust gives the opportunity to express this behaviour with Weak-References just
like Rc or Arc in the standard library.

## Interior Mutablity

Furthermore a View or Output is actually no real pointer just an ID handle by
wlc internally. wlc does not expose the actual type, but instead lets us modify
the internal state of the Views and Output through well defined function, that
take and return by-value.

I make the argument, that `View` and `Output` act like e.g. `Cell` and are meant to be
used as if they had Interior Mutablity. I have not yet found an exception to
this rule that was crutial for functionality. All probablematic functions have
been adapted or are not provided by a safe api call.

If this turns out to be wrong and potentially unsafe, this library will be
rewritten to return `Rc<RefCell<View>>` and `Rc<RefCell<Output>>` instead.
Giving the rather huge circumstances for developers, I will try to avoid this if
anyhow possible.
