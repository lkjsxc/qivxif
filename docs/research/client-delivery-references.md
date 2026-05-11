# Client Delivery References

## Source Reports

- `tmp/deep-research-report (53).md`
- Supporting synthesis from reports `(49)`, `(50)`, and `(51)`

## Durable Findings

- The current repository has a server, probe, and headless client slice.
- The next credible Windows milestone is a headless/protocol client executable
  built as a Windows artifact.
- The longer delivery path is desktop shell, renderer shell, then mobile shells.
- Use one shared Rust client core behind thin platform shells.
- Use `winit` for native shell lifecycle and `wgpu` for the renderer family when
  client implementation begins.

## Windows Artifact Findings

- Linux Docker can cross-compile repeatable `x86_64-pc-windows-gnu` artifacts
  when the MinGW toolchain is installed.
- `x86_64-pc-windows-msvc` release artifacts should come from a Windows-native
  MSVC job.
- Windows-container-first delivery from a Linux host is not the recommended path.

## Owner Targets

| Finding | Durable owner |
| --- | --- |
| Headless/protocol client milestone | `architecture/client/` |
| Shared client core | `architecture/client/` |
| Desktop shell | `architecture/client/` |
| Renderer family | `architecture/client/` and `research/rendering-references.md` |
| Windows artifact policy | `decisions/open-questions.md` |
