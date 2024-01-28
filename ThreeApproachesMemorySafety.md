## The Three approaches to memory safety

- **Manual Memory Management:**
  - Gives control to the programmer.
  - Requires careful handling to avoid memory-related bugs.
  - Memory safety is solely the responsibility of the programmer.

- **Garbage Collection:**
  - Provides ease for developers.
  - Ensures memory safety automatically.
  - Allocates memory as needed and reclaims garbage during runtime.
  - Incurs a cost in terms of resources and performance.

- **Rust's Ownership:**
  - Guarantees memory safety efficiently.
  - Avoids risks associated with manual memory management.
  - Uses a unique ownership system to manage memory effectively.
