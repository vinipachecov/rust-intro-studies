# Enums

* Use enum as a type for fields of a structure
* Predefined Values
* Using different types inside enum.

```rs
enum IpdAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpdAddrKind,
    address: String,
}
```

## Option Enum

* Option, which is another enum defined by the standard library.

* The option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing

```rs
enum Option<T> {
    Some(T),
    None,
}
```