# write!(write매크로 알아보기)


- https://doc.rust-lang.org/std/fmt/fn.write.html


- unsafe 쓰고 꼭 assert를 써서 안전한지 체크 해준다.  10줄에 한개 쓴다 마인드로 하자

  - https://doc.rust-lang.org/src/core/fmt/mod.rs.html#1150-1168

```rs

unsafe fn run(fmt: &mut Formatter<'_>, arg: &rt::Placeholder, args: &[rt::Argument<'_>]) -> Result {
    fmt.fill = arg.fill;
    fmt.align = arg.align;
    fmt.flags = arg.flags;
    // SAFETY: arg and args come from the same Arguments,
    // which guarantees the indexes are always within bounds.
    unsafe {
        fmt.width = getcount(args, &arg.width);
        fmt.precision = getcount(args, &arg.precision);
    }

    // Extract the correct argument
    debug_assert!(arg.position < args.len());
    // SAFETY: arg and args come from the same Arguments,
    // which guarantees its index is always within bounds.
    let value = unsafe { args.get_unchecked(arg.position) };

    // Then actually do some printing
    value.fmt(fmt)
}

```
