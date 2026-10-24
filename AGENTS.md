# near-kit-tool-box-rs

🦀 near kit rs tool box, functions and more for specific near interactions.

this repo is a part of near kit tool box project
- [TS tool box](https://github.com/sleet-js/near-kit-tool-box-ts) - the typescript tool box
- [RUST tool box](https://github.com/sleet-rs/near-kit-tool-box-rs) - this repo, the rust tool box
- [SKILLS](https://github.com/sleetplayground/near-kit-tool-box-skills) - the skill library

- `src/bin` - example bins
- `src/fun` the resuable functions
- `src/lib/contract` near kit rs typed contracts
- `src/lib/methods` methods consts
- `src/lib/const_id` contract id constants
- `src/lib/types` shared response / data shapes
- `docs` mdbook

---

#### CONTRIBUTING

- see [WORKFLOW.md](./WORKFLOW.md) for workflow
- see [STANDARDS.md](./STANDARDS.md) for naming and layout standards
- see [README.md](./README.md) for the cargo commands and dependencies
- see [near-kit-tool-box-skills](https://github.com/sleetplayground/near-kit-tool-box-skills) for the skill library, keep the two in sync when a contract method changes

---

#### USING

the lib holds the reusable functions, the bins are examples. use `cargo run` to run a bin and `cargo check` to type check the crate.

the docs folder is an mdbook, use `mdbook serve --open` to read it locally.

==========================
<br/>
copyright 2026 by sleet.near
