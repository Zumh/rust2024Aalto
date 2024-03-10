## Borrwing and dereferencing
- References are values that enable indirect access to other values. 
- They point, i.e. refer, to some data of another value rather than containing the data themselves.
- cannot modify variable that being borrowed by book

## string borrowing and onwer
- When we want to use a value from an expression or a function that depends on borrowed data, 
- we can use the to_owned method of a borrow to create an owned copy of the borrowed data.
- On a &str, the to_owned() method returns a new String, just like String::from.