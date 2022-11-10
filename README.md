# carender

Unix `cal` utility, written in Rust, inspired by:

- [`cal.c` in suckless's sbase](https://git.suckless.org/sbase/file/cal.c.html)
- [`cal.c` in util-linux](https://github.com/util-linux/util-linux/blob/master/misc-utils/cal.c)
- [`example-calendar.rs`](https://github.com/rust-lang/rust/blob/master/src/test/ui/impl-trait/example-calendar.rs)

## Roadmap

- [x] Plain calendar for a month
- [x] Multi-month formatting
- [ ] First weekday `[-s | -m | -f num]`
- [ ] Columns `[-c num]` and terminal width detection
- [ ] Weekend and today colorization
- [ ] Previous and next month hint
