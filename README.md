# strum overflow reproducers

`cargo test`

# origin story

this issue was originally detected by the
[kani](https://github.com/model-checking/kani) formal
checker on a company codebase.

after following the [installation steps](https://github.com/model-checking/kani#installation)
running `cargo kani autoharness -Z autoharness` on this reproducer
raises the same overflow warnings, though most of them can't really
happen with normally-sized enums on 32/64-bit platforms (Rust does
support some 16-bits ones though, for which a 32768+ variants enum
could trigger overflow or incorrect behavior).

# fixes

I've pushed a few fixes on the `fix-overflows` branch of my strum fork,
you're welcome to integrate them (or not)

The first commit fixes a real overflow (triggerable via this reproducer)


The problematic behavior under 16-bit archs and/or really large enums
is constrained by the 2nd commit on `strum`:
`fix: limit variant count to usize::MAX / 2`

The 3rd commit just documents that this invariant must be upheld


The 4th commit `feat: add kani assumptions` place the relevant assumptions
so `kani` understands the new constraints. However, they don't
_enforce_ those constraints, and while they don't add any actual dependency
on `kani` itself, they add a conceptual one: future contributors may wonder
what this is about, it requires `#[allow(unexpected_cfgs)]` and a change
in `kani` could break syntax for example.

About this, I see three possible solutions:

- officially adopt `kani` among `strum` 's tests. I would be interested in writing more
  `kani` tests for strum (albeit I'm a beginner at this) if this ends up the chosen solution.
- replace `kani::assume` calls by `if <impossible_condition> { return <Err/None/0>;}`
- not integrating this commit and not caring about downstream users that rely on `kani`


## AI usage

a hot topic recently, so: I did _minimally_ use a LLM, mostly to check
"if I haven't missed anything similar/in-scope across this unfamiliar codebase".

I consider having writen all the code in the reproducer and the fixes myself,
even if technically, I did copy paste a couple lines from the AI and the internet

See my full stance if needed at: https://blog.michaelbleuez.com/ai-manifesto/
