# Karp (Warp Fork) Knowledge Base

This repository is a customized, standalone fork of the open-source Warp terminal, branded as **Karp**. It has been intentionally modified to act as a fully offline, vanilla terminal emulator by removing cloud integrations, AI features, and user authentication requirements.

## Architecture & Build System
- **Rendering Engine:** Warp uses a 100% custom, GPU-accelerated UI framework called `warpui` built on top of Apple's Metal API and `wgpu`. It does NOT use standard macOS AppKit UI components.
- **Build Tooling:** The macOS application is bundled using `cargo-bundle`. The primary build script is `./script/run` (which internally executes `script/macos/run`).
- **Dependencies:** Compiling the codebase requires the macOS Metal Toolchain (`xcodebuild -downloadComponent MetalToolchain`) for GPU shader compilation, and Protocol Buffers (`brew install protobuf`) for the `warp_multi_agent_api` crate.

## "Vanilla Karp" Core Modifications
To maintain this repository as an offline-first terminal, the following surgical UI and authentication logic changes were made in the source code:

1. **Bypassed Auth/Onboarding:** In `app/src/root_view.rs`, the `requires_login` boolean logic is hardcoded to `false` to completely bypass the `LoginSlideView`, dropping the user directly into the terminal interface without credentials.
2. **Removed User Dropdown:** In `app/src/workspace/view.rs`, the `render_avatar_button` returns an empty `Stack` (e.g., `Stack::new().finish()`) to hide the user profile menu from the top navigation bar.
3. **Suppressed Login Traps:** In `app/src/workspace/view.rs`, the `open_require_login_modal` function immediately returns without triggering a state update. This prevents unpassable login popups if the user accidentally clicks on cloud-gated features (like Warp Drive or AI).
4. **Rebranded to Karp:** Modified `app/Cargo.toml` (`[package.metadata.bundle.bin.warp-oss]`) to rename the application bundle to `Karp` and use the alternative built-in icon `assets/bundled/png/dev_512x512.png`.

## Maintenance Strategy & Warnings
- **Refusing Upstream Merges:** Because the core feature flags and AI logic are deeply woven into the codebase, attempting to literally delete the `ai` or `cloud` folders will break compilation. The documented approach above "lobotomizes" the features via UI omission rather than code deletion.
- **OS Compatibility Ticking Clock:** Because Karp relies on a highly customized, low-level Metal rendering pipeline, major macOS updates (e.g., to macOS window compositing or graphics APIs) could introduce visual artifacting or crashes. If you choose *never* to pull upstream fixes from Warp, you will be solely responsible for debugging and fixing complex GPU rendering issues.