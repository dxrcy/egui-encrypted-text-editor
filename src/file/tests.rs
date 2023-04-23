use super::*;

#[test]
fn check_save_state() {
    // * Unregistered
    // is_saved: Always FALSE
    // is_changed: TRUE if non-empty
    // actual save state should not affect outcome

    // Default (Unregistered, Empty)
    let file = File::default();
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), false);

    // Unregistered, Empty
    let file = File {
        path: None,
        contents: String::new(),
        saved: false,
    };
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), false);
    // Same, but saved (should not matter)
    let file = File {
        path: None,
        contents: String::new(),
        saved: true,
    };
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), false);

    // Unregistered, NON-Empty
    let file = File {
        path: None,
        contents: String::from("Some contents"),
        saved: false,
    };
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), true);
    // Same, but saved (should not matter)
    let file = File {
        path: None,
        contents: String::from("Some contents"),
        saved: true,
    };
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), true);

    // * Registered
    // is_saved: TRUE if saved is TRUE
    // is_changed: TRUE if saved is FALSE
    // contents should not affect outcome

    // Registered, unsaved
    let file = File {
        path: Some(String::from("some/path")),
        contents: String::new(),
        saved: false,
    };
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), true);
    // Same, but non-empty (should not matter)
    let file = File {
        path: Some(String::from("some/path")),
        contents: String::from("Some contents"),
        saved: false,
    };
    assert_eq!(file.is_registered_and_saved(), false);
    assert_eq!(file.is_changed(), true);

    // Registered, saved
    let file = File {
        path: Some(String::from("some/path")),
        contents: String::new(),
        saved: true,
    };
    assert_eq!(file.is_registered_and_saved(), true);
    assert_eq!(file.is_changed(), false);
    // Same, but non-empty (should not matter)
    let file = File {
        path: Some(String::from("some/path")),
        contents: String::from("Some contents"),
        saved: true,
    };
    assert_eq!(file.is_registered_and_saved(), true);
    assert_eq!(file.is_changed(), false);
}
