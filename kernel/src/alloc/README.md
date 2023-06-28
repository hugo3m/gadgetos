# Heap allocation

How to add heap allocation for the kernel.

#### Local and static variables

Local variables are stored on the [call stack](https://en.wikipedia.org/wiki/Call_stack) and are only valid until the surrounding function returns. Static variables are stored at a fixed memory and always live for the complete lifetime of the program.

![alt text](/assets/img/call-stack.svg)
*Here the call of the inner function add on the call stack the value.*

![alt text](/assets/img/call-stack-return.svg)
*When the function is done the local variables are destroyed.*

Static variables are stored at a fixed memory location separate from the stack. This memory location is assigned at compile time by the linker and encoded in the executable. Statics live for the complete runtime of the program, so they have the 'static lifetime and can always be referenced from local variables.

![alt text](/assets/img/call-stack-static.svg)

Static variables also have the useful property that their location is known at compile time, so that no reference is needed for accessing them. However, this property of static variables brings a crucial drawback: **they are read-only by default**.

#### Dynamic memory

Limitations of local and static variables:

* Local variables only live until the end of the surrounding function or block;
* Static variables have unclear ownership semantics and are read-only by default;
* Local and static variables have a fixed size.

To circumvent these drawbacks, programming languages often support a third memory region for storing variables called the *heap*. The heap supports dynamic memory allocation at runtime through two functions called **allocate** and **deallocate**:
* `allocate`: function returns a free chunk of memory of the specified size for the `variable`;
* `deallocate`: `variable` lives until it is freed by calling `deallocate`.

The advantage of using heap memory compared to static memory is that the memory can be reused after it is freed.