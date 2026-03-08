# valid-demo

`valid-demo` is a demo repository for showing what becomes possible, and what kinds of requirements work you may want to do, when you use the formal verification language [`valid`](https://github.com/gorira-tatsu/valid) during requirements definition.

[`valid`](https://github.com/gorira-tatsu/valid) is a Rust-first finite-state verification language and toolchain for modeling business rules, workflows, contracts, and other stateful requirements in a machine-checkable form.

Instead of being only a place to store specification documents, this repository shows how to turn requirements into verifiable artifacts.

- Organize specification documents by feature area
- Translate requirements into `valid` models with states, actions, and business rules
- Define expected properties explicitly and check them automatically
- Use coverage and explain-style diagnostics to find unreachable branches and missing constraints

The current demo target is a simple bulletin board application under the `bss` directory.

## What This Repository Shows

- How to go beyond screen-level specs and turn requirements into verifiable constraints
- How to split a problem into `state`, `action`, and `property` in [`valid`](https://github.com/gorira-tatsu/valid)
- How to verify cross-feature flows such as list -> create -> detail -> edit/delete -> comment, not just isolated screens
- How to operate contract drift checks, property verification, and coverage review in a practical order

## Repository Structure

- [`bss/README.md`](/Users/tatsuhiko/code/valid-demo/bss/README.md)
  Entry point for the bulletin board demo, including the recommended reading order, model overview, and commands.
- [`bss/docs/rdd/README.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/rdd/README.md)
  Index of the requirements and design definition documents.
- [`bss/docs/valid_registry_workflow.md`](/Users/tatsuhiko/code/valid-demo/bss/docs/valid_registry_workflow.md)
  Verification workflow notes for `valid-registry`.

## How To Read The Demo

1. Read the RDD documents to understand the bulletin board requirements
2. Inspect `valid/board_rdd_registry.rs` and the model files below it to see how the requirements were turned into `state`, `action`, and `property`
3. Run `inspect`, `check`, and `coverage` to see how the specification becomes machine-readable
4. Review cross-cutting models such as `board-flow` to understand how end-to-end consistency is expressed

## Current Demo Scope

`bss` currently contains 14 models.

- Common specification
- Post list
- Post creation
- Post detail
- Post edit / delete
- Comments
- List rendering
- Presentation contract
- API contract
- Edit key storage
- Retry UX
- Submission discipline
- Message contract
- Cross-cutting board flow

This structure makes it possible to verify more than just happy paths.

- Consistency of HTML escaping and error code behavior
- Invisibility of deleted posts
- Ordering and excerpt rules
- Contracts for UI messages and error-message placement
- Consistency between list and detail views
- Recovery behavior after retry

## Running The Demo

`bss` depends on [`valid`](https://github.com/gorira-tatsu/valid) through a pinned GitHub revision. After cloning this repository, you should be able to resolve dependencies without placing a local `../../valid` checkout next to it, as long as you have a Rust toolchain.

This repository itself still does not define a license. Before treating it as a polished public sample, adding a license file would be a good next step.
