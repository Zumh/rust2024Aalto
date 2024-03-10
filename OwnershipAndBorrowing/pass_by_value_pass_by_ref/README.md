## Ref rules
- Like ownership has its set of rules, so does borrowing. The two rules of references summarize when references to data are allowed to exist:
- At any given time, a value can have either one mutable reference &mut or any number of immutable references &.
- References must always be valid.

## Explaination
- The first rule, together with disallowing mutation of the owner while a borrow is alive, ensures that we don't accidentally modify a value that is being used or modified somewhere else (especially problematic in multi-threaded applications with concurrent mutating of values).
- The second rule ensures that we don't have references to values that have already been dropped.