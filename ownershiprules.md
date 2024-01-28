## Rust Owner ship rules
1. **Ownership:**
   - Each value in Rust has a variable that is its "owner."
   - There can only be one owner at a time.
   - When the owner goes out of scope, the value is dropped.

2. **Borrowing:**
   - Instead of transferring ownership, Rust allows references (or "borrows") to be created.
   - Borrowing can be either mutable or immutable.
   - Immutable borrows allow reading but not modifying the data.
   - Mutable borrows allow both reading and modifying the data.

3. **References and Lifetimes:**
   - References have lifetimes, which specify how long they are valid.
   - Lifetimes ensure that references don't outlive the data they point to.
   - Lifetimes are used to prevent dangling references and ensure memory safety.

4. **No Double Free:**
   - Rust's ownership system prevents double free errors.
   - Once a value is owned by a variable, the ownership cannot be transferred without using explicit mechanisms like cloning or moving.

5. **Ownership Transfer:**
   - Ownership of a value can be transferred from one variable to another using a move operation.
   - After the move, the original owner can no longer access the value.

6. **Ownership and Functions:**
   - When passing values to functions, ownership can be transferred (moved) to the function, or references can be passed.
   - The borrowing rules apply to function parameters as well.

7. **Ownership and Structs:**
   - Structs in Rust follow the ownership rules.
   - The ownership of a struct field depends on the ownership of its individual components.

8. **Ownership and Enums:**
   - Enums in Rust can hold different types of data with different ownership semantics.
   - Ownership rules apply to the values stored in enum variants.

These ownership rules, enforced by the Rust compiler, help prevent common programming errors such as data races, null pointer dereferences, and memory leaks. 
They contribute to Rust's safety guarantees and make it possible to write high-performance code without sacrificing memory safety.
