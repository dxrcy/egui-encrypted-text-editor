# TODO

1.  Add 'writing' save state

    -   Separate boolean state
    -   Add fake 1s delay to file `save_to_path` function
    -   Fix `is_saved` and `is_changed` functions
    -   Check tests
    -   Test manually!

    -   Move save state to `State` struct ?

2.  Add dialog for closing while saving

    -   Test manually!

3.  Add encryption

    -   Encrypt on file `save_to_path`
    -   Decrypt on file `open_path`
    -   Throw error if cryption fails

4.  Add error message for corrupted file (not encrypted properly)

5.  Add filter for `.enc` files (easy)
