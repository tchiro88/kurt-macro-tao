# Macro iOS cold-start link backport

Base: `tao-v0.37.0` (`2f9eecf2`).

Backports [tauri-apps/tao#1257](https://github.com/tauri-apps/tao/pull/1257)
(head `7005107630ece9267749aece8321d461f7446719`) by balabiturembekov.
The upstream PR remains unmerged; remove this fork once a compatible release
includes the fix and the nullable-accessor review correction.

## Changes beyond the PR

- Preserve Tao 0.37's scene lifecycle and per-window resume/suspend events.
- Read `URLContexts` and `userActivities` as optional retained sets. UIKit can
  return nil despite its nonnull annotations, including on ordinary launches.
  This addresses [the blocking review](https://github.com/tauri-apps/tao/pull/1257#discussion_r3823531720).
- Also tolerate a nil `UIOpenURLContext.URL` rather than aborting the process.
- Log delivery source and URL count without logging link contents.
- Add simulator-executable tests for universal-link activity filtering: ignore
  missing URLs and handoff activities even when they contain a webpage URL.

## Verification

Nine Rust tests executed on the iOS 27 simulator (not just cross-compiled):

```sh
export DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer
export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_RUNNER="xcrun simctl spawn <simulator-udid>"
cargo test --target aarch64-apple-ios-sim --lib -- --test-threads=1
```

Macro with Tauri fork `957785f5`: built for iOS Simulator; normal startup
reached the welcome screen; a cold `macro://app/login` link reached login;
a warm `macro://app/welcome` link navigated back. Logs showed one `Opened`
event per tested link and none for a normal launch.

Universal-link extraction is covered with real `NSUserActivity` objects in
simulator tests. Signed associated-domain delivery, physical devices, iOS 26,
and document-open/share-sheet integration still require end-to-end testing.
