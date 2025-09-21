use std::process::Command;
use std::str;

// Helper function to get the path to the compiled binary
fn get_binary_path() -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug"); // Tests are run against the debug profile
    path.push("macism-rust");
    path
}

#[test]
fn test_get_current_id_default() {
    let binary_path = get_binary_path();
    assert!(
        binary_path.exists(),
        "Binary not found at path: {}",
        binary_path.display()
    );

    let output = Command::new(&binary_path)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        panic!(
            "Command failed with status: {}\nstdout: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(!stdout.is_empty(), "stdout should not be empty");
}

#[test]
fn test_version() {
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        panic!(
            "Command failed with status: {}\nstdout: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(stdout.contains("macism-rust"));
}

#[test]
fn test_list_keyboard() {
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(!stdout.is_empty(), "Keyboard list should not be empty");
    assert!(!stdout.contains("com.apple.CharacterPaletteIM"), "Keyboard list should not contain palette IMs");
}

#[test]
fn test_list_palette() {
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .arg("-p")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(!stdout.contains("jp.sourceforge.inputmethod.aquaskk.Hiragana"));
    assert!(stdout.contains("com.apple.CharacterPaletteIM"));
}

#[test]
fn test_list_all() {
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .arg("-l")
        .arg("-p")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(!stdout.is_empty(), "All list should not be empty");
    assert!(stdout.contains("com.apple.CharacterPaletteIM"), "All list should contain palette IMs");
}

#[test]
fn test_set_and_verify() {
    let binary_path = get_binary_path();

    // Get initial state
    let initial_output = Command::new(&binary_path)
        .output()
        .expect("Failed to get initial state");
    let initial_id = str::from_utf8(&initial_output.stdout).unwrap().trim();

    // Determine target ID from available keyboard input sources
    let list_output = Command::new(&binary_path)
        .arg("-l")
        .output()
        .expect("Failed to list keyboard input sources");
    assert!(list_output.status.success());
    let available_keyboard_ids: Vec<&str> = str::from_utf8(&list_output.stdout)
        .unwrap()
        .trim()
        .lines()
        .collect();

    // Find a target ID that is different from the initial one, if possible
    let mut target_id = "";
    if available_keyboard_ids.len() > 1 {
        for id in available_keyboard_ids {
            if id != initial_id {
                target_id = id;
                break;
            }
        }
    } else if available_keyboard_ids.len() == 1 {
        target_id = available_keyboard_ids[0];
    } else {
        eprintln!("No keyboard input sources available for test_set_and_verify. Skipping.");
        return;
    }

    // Set to target ID
    let set_output = Command::new(&binary_path)
        .arg("--") // Add -- to separate options from positional arguments
        .arg(target_id)
        .output()
        .expect("Failed to execute set command");

    if !set_output.status.success() {
        panic!(
            "Set command failed with status: {}\nstdout: {}\nstderr: {}",
            set_output.status,
            String::from_utf8_lossy(&set_output.stdout),
            String::from_utf8_lossy(&set_output.stderr)
        );
    }
    let set_stdout = str::from_utf8(&set_output.stdout).unwrap().trim();
    assert_eq!(set_stdout, target_id);

    // Verify the change
    let final_output = Command::new(&binary_path)
        .output()
        .expect("Failed to get final state");
    let final_id = str::from_utf8(&final_output.stdout).unwrap().trim();
    assert_eq!(final_id, target_id);

    // Revert to initial state
    let revert_output = Command::new(&binary_path)
        .arg("--") // Add -- to separate options from positional arguments
        .arg(initial_id)
        .output()
        .expect("Failed to revert to initial state");
    assert!(revert_output.status.success());
}