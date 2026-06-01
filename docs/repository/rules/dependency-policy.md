# Dependency Policy

## Rules

- Add dependencies only when they support documented architecture.
- Prefer Rust crates for domain logic.
- Keep server storage independent from Axum.
- Keep API structs independent from Axum.
- Use TypeScript only at browser Web API and UI boundaries unless docs say otherwise.
