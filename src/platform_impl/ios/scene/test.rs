// Copyright 2021-2025 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use super::*;
use objc2::AnyThread;

#[test]
fn browsing_activity_without_webpage_is_ignored() {
  let activity = NSUserActivity::initWithActivityType(NSUserActivity::alloc(), unsafe {
    NSUserActivityTypeBrowsingWeb
  });
  assert!(webpage_url_string(&activity).is_none());
}

#[test]
fn handoff_webpage_is_not_a_universal_link() {
  let activity = NSUserActivity::initWithActivityType(
    NSUserActivity::alloc(),
    &NSString::from_str("com.macro.handoff"),
  );
  let url = NSURL::URLWithString(&NSString::from_str("https://example.com/handoff")).unwrap();
  activity.setWebpageURL(Some(&url));
  assert!(webpage_url_string(&activity).is_none());
}

#[test]
fn connection_activities_keep_only_browsing_links() {
  let browsing = NSUserActivity::initWithActivityType(NSUserActivity::alloc(), unsafe {
    NSUserActivityTypeBrowsingWeb
  });
  let url = NSURL::URLWithString(&NSString::from_str(
    "https://example.com/app/login?source=test",
  ))
  .unwrap();
  browsing.setWebpageURL(Some(&url));
  let handoff = NSUserActivity::initWithActivityType(
    NSUserActivity::alloc(),
    &NSString::from_str("com.macro.handoff"),
  );
  handoff.setWebpageURL(Some(&url));
  let activities = NSSet::from_retained_slice(&[browsing, handoff]);
  assert_eq!(
    url_strings_from_user_activities(&activities),
    vec!["https://example.com/app/login?source=test"]
  );
}
