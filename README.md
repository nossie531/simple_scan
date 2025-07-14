simple_scan
===

`Iterator` extensions for simple scan operation.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides the `IteratorSimpleScanExt` trait.
The `IteratorSimpleScanExt` trait is an extension trait of the `Iterator` trait,
which implements some methods similar to the `scan` methods of the `Iterator`
trait, but more simplified and specialized.

The following is a list of those methods. The sample code on the left shows the
case where those simplified methods are used, and the sample code on the right
shows the case where the same process is implemented with `scan` method.

<table>
<thead>
<tr>
<th>Name
<th>example / equivalent code
<tbody valign="baseline">
<tr>
<td> <code>trace</code>
<td>

```rust
(0..10).trace(0, |s, x| s + x)
```

```rust
(0..10).scan(0, |s, x| {
    *s += x;
    Some(*s)
});
```

<tr>
<td> <code>trace2</code>
<td>

```rust
(0..10).trace2(0, |s, x| s + x)
```

```rust
(0..10).scan(0, |s, x| {
    let prev = *s;
    *s += x;
    Some((prev, *s))
});
```

<tr>
<td><code>diff</code>

<td>

```rust
(0..10).diff(0, |c, p| c - p)
```

```rust
(0..10).scan(0, |s, x| {
    let p = mem::replace(s, x);
    Some(x - p)
});
```

</table>

## Versions

See [CHANGELOG](CHANGELOG.md).
