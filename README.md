simple_scan
===

`Iterator` extension trait for simple scan.

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
<th>simple version
<th>scan version
<tbody>
<tr>
<td style="vertical-align: baseline;">

`trace`

<td style="vertical-align: baseline;">

```rust
(0..10).trace(0, |s, x| s + x)
```

<td style="vertical-align: baseline;">

```rust
(0..10).scan(0, |s, x| {
    *s += x;
    Some(*s)
});
```

<tr>
<td style="vertical-align: baseline;">

`trace2`

<td style="vertical-align: baseline;">

```rust
(0..10).trace2(0, |s, x| s + x)
```

<td style="vertical-align: baseline;">

```rust
(0..10).scan(0, |s, x| {
    *s += x;
    Some((*s, x))
});
```

<tr>
<td style="vertical-align: baseline;">

`diff`

<td style="vertical-align: baseline;">

```rust
(0..10).diff(0, |c, p| c - p)
```

<td style="vertical-align: baseline;">

```rust
(0..10).scan(0, |s, x| {
    let p = mem::replace(s, x);
    Some(x - p)
});
```

</table>
