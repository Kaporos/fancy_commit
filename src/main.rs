use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input, Select};
use std::{
    env, fs,
    process::{exit, Command},
};

struct CommitType {
    name: &'static str,
    emoji: &'static str,
}

struct CommitTypesManager {
    types: Vec<CommitType>,
}

impl CommitTypesManager {
    fn new() -> CommitTypesManager {
        CommitTypesManager { types: vec![] }
    }
    fn add_type(&mut self, name: &'static str, emoji: &'static str) {
        self.types.push(CommitType { name, emoji })
    }

    fn get_names(&self) -> Vec<&'static str> {
        Vec::from_iter(self.types.iter().map(|value| value.name))
    }
}

fn text_input(content_prompt: &str, multiline: bool) -> String {
    let mut prompt = String::from(content_prompt);
    if multiline {
        prompt = format!("{}{}", content_prompt, " (\\n to pass a line)")
    }
    let mut text: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap();
    if multiline {
        text = text.replace("\\n ", "\n");
        text = text.replace("\\n", "\n");
    }
    text
}

fn if_confirmed_string(confirm_prompt: &str, content_prompt: &str, multiline: bool) -> String {
    let mut text = String::new();
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(confirm_prompt)
        .interact()
        .unwrap()
    {
        text = text_input(content_prompt, multiline)
    }
    text
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args
        .get(args.len() - 1)
        .unwrap_or_else(|| panic!("You have to provide a file to edit"));
    if !file_name.contains("COMMIT") {
        if args.len() <= 2 {
            panic!("You're asking to edit a file that isn't a commit and you didn't specify fallback editor");
        } else {
            Command::new(args[args.len() - 2].clone())
                .args([file_name])
                .spawn()
                .expect("Failed to start !")
                .wait()
                .expect("Failed to start");
            exit(0)
        }
    }
    let term = Term::stdout();
    term.write_line("Fancy Git Commit v0.1").unwrap();
    term.write_line("What's your commit's type ?").unwrap();
    let mut types_manager = CommitTypesManager::new();
    types_manager.add_type("feat", "✨");
    types_manager.add_type("fix", "🐛");
    types_manager.add_type("hotfix", "🚑️");
    types_manager.add_type("ci", "👷");
    types_manager.add_type("docs", "📝");
    types_manager.add_type("chore", "🔧");
    types_manager.add_type("style", "💄");
    types_manager.add_type("refactor", "♻️");
    types_manager.add_type("perf", "⚡️");
    types_manager.add_type("test", "🧪");
    let index: usize = Select::with_theme(&ColorfulTheme::default())
        .items(&types_manager.get_names())
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();
    let c_type = &types_manager.types[index];
    let mut scope =
        if_confirmed_string("Is there a scope for your commit ?", "Commit scope", false);
    if scope.len() > 0 {
        scope = format!("({})", scope)
    }
    let name = text_input("Commit name", false);
    let body = if_confirmed_string("Do you want to include a body", "Commit's body", true);
    let mut breaking_changes = if_confirmed_string(
        "Is there any breaking changes in your commit ?",
        "Breaking changes",
        true,
    );
    if breaking_changes.len() > 1 {
        breaking_changes = format!("BREAKING CHANGES: {}", breaking_changes);
    }
    fs::write(
        file_name,
        format!(
            "{}{}: {} {}\n\n{}\n\n{}",
            c_type.name, scope, c_type.emoji, name, body, breaking_changes
        ),
    )
    .unwrap();
}
