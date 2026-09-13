# Technical verification by domain

Load only the sections relevant to the requested article. These are review prompts, not a frozen knowledge base. Verify actual APIs and versions against the ecosystem's official docs at writing time.

## C# / .NET

Record SDK, target framework, package versions, and nullable context. Check cancellation propagation, async disposal, DI lifetimes, exception handling, and thread safety. Show a reproducible build/run command. Use Microsoft Learn and official package repositories.

## Go

Record Go version and go.mod dependencies. Check context cancellation, error wrapping, goroutine termination, channel ownership, races, and HTTP timeouts. Include go test or an equivalent verification command; use race checks when concurrency is central. Use go.dev and upstream package documentation.

## Rust

Record toolchain, edition, and Cargo dependencies. Check ownership/lifetimes, error paths, Send/Sync assumptions, async blocking, and unsafe invariants. Prefer Result-based examples over unexplained unwrap in failure-prone paths. Use doc.rust-lang.org and upstream crate docs; run cargo test/check where available.

## JavaScript / TypeScript

State Node/browser targets, package manager and versions, ESM/CommonJS assumptions, and compiler options when relevant. Check async failures, cleanup, runtime input validation, and server/client boundaries. Use MDN, nodejs.org, typescriptlang.org, and upstream libraries.

## Angular

State Angular version and whether examples use standalone components or NgModules. Verify signal/RxJS behavior, subscription cleanup, dependency injection scope, forms, routing, and browser/SSR differences. Use angular.dev.

## React

State React/framework versions, rendering environment, and server/client component boundaries. Check effect dependencies, cleanup, stale closures, list keys, accessibility, hydration, and state ownership. Use react.dev and the framework's official docs.

## Mobile apps

Name Android, iOS, Flutter, React Native, or another selected stack and its versions. Check lifecycle, permissions, background execution, secure storage, offline behavior, platform differences, and accessibility. Distinguish emulator results from real-device verification. Use developer.android.com, developer.apple.com, flutter.dev, reactnative.dev, and upstream docs as applicable.

## AI agents, workflows, and coding

Specify provider, model/API version when known, tools, state, and human interaction. Distinguish deterministic workflow control from model decisions. Cover tool permissions, untrusted input, retries/idempotency, timeouts, context limits, cost assumptions, evaluation criteria, and failure recovery when relevant. Do not claim reliability from a single demo or invent throughput/cost measurements. Keep API keys outside examples and provide fake sample values. For AI coding, show how generated changes are inspected and tested. Verify provider APIs against official documentation.
