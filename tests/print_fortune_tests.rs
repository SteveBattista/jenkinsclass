use fortune::print_fortune;

#[test]
/// Verifies the default recipient name `you` is rendered in the output.
fn print_fortune_uses_default_you_name() {
    let output = print_fortune("Your stack is stable today.", "you");

    assert!(output.contains("This fortune is for you."));
    assert!(output.contains("\"Your stack is stable today.\""));
}

#[test]
/// Verifies a custom recipient name is rendered in the output.
fn print_fortune_uses_custom_name() {
    let output = print_fortune("May your deploy be green.", "Avery");

    assert!(output.contains("This fortune is for Avery."));
    assert!(output.contains("\"May your deploy be green.\""));
}

#[test]
/// Verifies empty fortune text is still quoted correctly.
fn print_fortune_handles_empty_fortune_text() {
    let output = print_fortune("", "Jordan");

    assert!(output.contains("This fortune is for Jordan."));
    assert!(output.contains("\"\""));
}

#[test]
/// Verifies recipient names containing spaces are preserved.
fn print_fortune_handles_spaces_in_name() {
    let output = print_fortune("Backups are your true friends.", "Captain Picard");

    assert!(output.contains("This fortune is for Captain Picard."));
    assert!(output.contains("\"Backups are your true friends.\""));
}

#[test]
/// Verifies UTF-8 emoji characters are preserved in fortune text.
fn print_fortune_preserves_utf8_emoji_text() {
    let output = print_fortune("Ship it 🚀 and celebrate with tacos 🌮.", "Dev Team");

    assert!(output.contains("This fortune is for Dev Team."));
    assert!(output.contains("\"Ship it 🚀 and celebrate with tacos 🌮.\""));
}

#[test]
/// Verifies accented UTF-8 characters are preserved in names and text.
fn print_fortune_preserves_utf8_accented_name() {
    let output = print_fortune("Refactors bring clarté.", "Renée");

    assert!(output.contains("This fortune is for Renée."));
    assert!(output.contains("\"Refactors bring clarté.\""));
}

#[test]
/// Verifies mixed UTF-8 scripts (CJK and Arabic) are preserved.
fn print_fortune_preserves_utf8_cjk_and_arabic_text() {
    let output = print_fortune("成功は準備から始まる - النجاح يبدأ بالتحضير", "Kai");

    assert!(output.contains("This fortune is for Kai."));
    assert!(output.contains("\"成功は準備から始まる - النجاح يبدأ بالتحضير\""));
}
