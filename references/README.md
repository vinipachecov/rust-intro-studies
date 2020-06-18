# References 

- No limit to immutable referencers

- Only one mutable refence in a scope

- Can't create mutable reference if your program uses more than one immutable references.

# Data races
A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is beign used to write to the data.
- There's no mechanism being used to synchronize access to the data.