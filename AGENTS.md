# Rust practice exercises

The purpose of Dojo is to build Daniel's ability to independently implement useful Rust with the standard library and common crates, especially concurrency and Tokio. Exercise bodies remain unfinished for the learner; reference solutions belong in the matching Markdown file.

## Exercise design

- Give each exercise one coherent responsibility, usually combining a few related operations. Keep isolated API drills when the API has meaningful ownership, lifetime, or behavioral subtleties. Increase difficulty through useful requirements, not boilerplate or artificial restrictions.
- Specify observable behavior and acceptance criteria before writing the reference. Express ownership, borrowing, mutability, errors, and resource bounds through signatures and domain types where practical. Explain ordering, empty inputs, boundaries, and lifecycle behavior in the prompt.
- Let the learner choose and write implementation imports. Use qualified types in supplied signatures where necessary; supply domain types, fixtures, and test-only imports. Do not pre-import the intended solution's tools or put implementation recipes in TODO messages or learning-objective comments.
- Keep types required by the public contract, but let the learner choose private storage unless the representation itself is the learning objective. Do not leave the learner guessing undocumented domain rules.
- Prefer real standard-library/crate interfaces. Helpers must implement a complete documented contract. Do not provide a fake thread pool, fake cleanup, or fake I/O abstraction that bypasses the skill being exercised.
- Accept all implementations meeting the contract. Name an API in the prompt only when recalling that API is explicitly the exercise. Otherwise make its advantages follow from meaningful requirements such as allocation reuse, borrowed data, bounded concurrency, or ordering.
- For concurrency and Tokio, specify relevant worker/task limits, blocking and backpressure, ownership of spawned work, channel closure, cancellation, panic/error handling, and shutdown. Distinguish graceful draining from immediate cancellation. Never claim that a mutex makes a non-Send value Send or that a timeout proves deadlock.
- Use common crates where they solve the intended problem; avoid accidentally turning a file-processing task into implementing a CSV or Unicode standard. Keep dependency versions and examples compatible with Cargo.lock.
- Keep each exercise manageable for a review. Larger reusable components may be advanced exercises, but do not bundle unrelated types into one question. Merge redundant elementary drills when their skills naturally compose.

## Tests and references

- Assert exact outputs and important invariants, including non-Clone or borrowed inputs when those are part of the contract. Avoid permissive assertions, undocumented hidden requirements, and tests that always pass.
- Establish concurrent behavior with channels, barriers, and observable state. Use generous timeouts only as deadlock guards, not speed benchmarks. Test timers with Tokio's paused clock and test-util feature where appropriate.
- Use isolated temporary directories for file tests and ephemeral ports for network tests. Join or shut down test workers and servers. Make tests independent of ordering and external services.
- Keep the matching reference complete and runnable against the exercise's own tests. Verify changes using the reference-validation tooling as well as checking that unfinished exercise scaffolds compile. Passing stub compilation is not reference validation; passing zero tests is not success.
- During review, wait for Daniel's completion signal, re-read his live edits, run the focused tests, and assess behavior, complexity, ownership, and idiomatic Rust. Give hints or solutions only when requested or after the completed attempt. Suggestions alone do not authorize replacing his work.

## Revision deck

- Maintain exactly one notes card per exercise, and one exercise per card, in `/home/daniel/projects/notes/cards/dojo`. Module files and supplementary documentation are not exercises.
- Use the topic/exercise identifier in the card front so duplicate basenames resolve unambiguously. Keep the manifest's exercise, reference, and card mapping current when adding, renaming, merging, or removing exercises.
- Preserve review history for equivalent tasks and minor clarification/test fixes. Reset it only when explicitly authorized and the required skill or task changes substantially; record the reason in the audit manifest. New cards start with empty history. Retired exercises must not leave live orphan cards.
- Do not commit or push without a request covering the changes.

## Maintenance checks

- `exercises.json` is the exercise/card manifest and audit record. Every `src/<topic>/<exercise>.rs` except `mod.rs` has exactly one matching `.md` reference and notes card. Retired entries explain consolidation/removal; `history` records the audit's decision, not a permanent prohibition on later reviews.
- Run `python3 scripts/check_exercises.py` to check the manifest, module registration and sibling notes deck. Use `--notes PATH` for a different checkout or `--without-notes` to check Dojo alone.
- Run `cargo check --locked --tests` to type-check the learner scaffolds. Their `todo!()` bodies are intentional; do not run the entire unfinished suite as a correctness check.
- Run `python3 scripts/check_exercises.py --references` to build a separate temporary crate from the complete Rust block in each reference, then run that exercise's tests. This never replaces learner source. It rejects zero-test runs and uses a per-exercise deadlock timeout. The WebSocket test needs permission to bind an ephemeral localhost port.
- These commands need Python 3, Cargo, rustc and a C linker. Use the repository development shell where available. Focused learner reviews use `cargo test --lib topic::exercise::`.
- For an explicitly authorized audit reset, clear only that card's existing history rows and record why. Do not append invented ratings or dates. Ordinary review ratings still go through Retent.
