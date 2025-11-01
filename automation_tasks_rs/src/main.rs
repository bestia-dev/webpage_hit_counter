// automation_tasks_rs for webpage_hit_counter

// region: library and modules with basic automation tasks

mod build_cli_bin_mod;
mod build_cli_bin_musl_mod;
mod build_cli_bin_win_mod;
mod build_lib_mod;
mod build_wasm_mod;
mod cargo_auto_github_api_mod;
mod encrypt_decrypt_with_ssh_key_mod;
mod generic_functions_mod;
mod tasks_mod;

pub use cargo_auto_lib as cl;
#[allow(unused_imports)]
use crossplatform_path::CrossPathBuf;

use crate::cargo_auto_github_api_mod as cgl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;
use crate::generic_functions_mod as gn;
use crate::tasks_mod as ts;

pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

// region: library with basic automation tasks

/// The main() function returns ExitCode.
fn main() -> std::process::ExitCode {
    match main_returns_anyhow_result() {
        Err(err) => {
            eprintln!("{}", err);
            // eprintln!("Exit program with failure exit code 1");
            std::process::ExitCode::FAILURE
        }
        Ok(()) => std::process::ExitCode::SUCCESS,
    }
}

/// The main_returns_anyhow_result() function returns anyhow::Result.
fn main_returns_anyhow_result() -> anyhow::Result<()> {
    gn::tracing_init()?;
    cl::exit_if_not_run_in_rust_project_root_directory();
    ende::github_api_token_with_oauth2_mod::github_api_config_initialize()?;
    ende::crates_io_api_token_mod::crates_io_config_initialize()?;
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args)?;
    Ok(())
}

// region: match, help and completion

/// Match arguments and call tasks functions.
fn match_arguments_and_call_tasks(mut args: std::env::Args) -> anyhow::Result<()> {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help()?,
        Some(task) => {
            if &task == "completion" {
                completion()?;
            } else {
                println!("  {YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build()?;
                } else if &task == "release" {
                    task_release()?;
                } else if &task == "doc" {
                    task_doc()?;
                } else if &task == "test" {
                    task_test()?;
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2)?;
                } else if &task == "publish_to_web" {
                    task_publish_to_web()?;
                } else if &task == "github_new_release" {
                    task_github_new_release()?;
                } else {
                    eprintln!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help()?;
                }
            }
        }
    }
    Ok(())
}

/// Write a comprehensible help for user defined tasks.
fn print_help() -> anyhow::Result<()> {
    println!(
        r#"
  {YELLOW}Welcome to cargo-auto !{RESET}
  {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

  {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
  {YELLOW}It is preferred to use SSH for git push to GitHub.{RESET}
  {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
  {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto publish_to_web{RESET} - {YELLOW}publish to my google VM, git tag{RESET}
  {YELLOW}You need the SSH private key for publishing.{RESET}
  {YELLOW}You can type the SSH passphrase of the private key every time. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto update_automation_tasks_rs{RESET} - {YELLOW}updates the files in automation_tasks_rs{RESET}
  {YELLOW}Some files are fixed and the update is straight forward, other files need manual diff.{RESET}

  {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
    Ok(())
}

/// All example commands in one place.
fn print_examples_cmd() {
    /*
        println!(
            r#"
    {YELLOW}run examples:{RESET}
    {GREEN}cargo run --example plantuml1{RESET}
    "#
        );
    */
}

/// Sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`.
fn completion() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec![
            "build",
            "release",
            "doc",
            "test",
            "commit_and_push",
            "publish_to_web",
            "github_new_release",
            "update_automation_tasks_rs",
        ];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
    Ok(())
}

// endregion: match, help and completion

// region: tasks

/// Run 'cargo build' and appropriate functions.
fn task_build() -> anyhow::Result<()> {
    let cargo_toml = crate::build_cli_bin_musl_mod::task_build()?;
    println!(
        r#"
  {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
  {YELLOW}The postgres server and database must be accessible on the port 5432 on localhost{RESET}
{GREEN}./target/x86_64-unknown-linux-musl/debug/{package_name}{RESET}
  {YELLOW}In the browser or in curl open{RESET}
{GREEN}http://localhost:8011/webpage_hit_counter/get_svg_image/555555.svg{RESET}
  {YELLOW}if {package_name} ok then{RESET}
{GREEN}cargo auto release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
    Ok(())
}

/// Run 'cargo build --release' and appropriate functions.
fn task_release() -> anyhow::Result<()> {
    let cargo_toml = crate::build_cli_bin_musl_mod::task_release()?;

    println!(
        r#"
  {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}
{GREEN}alias {package_name}=target/release/{package_name}{RESET}
  {YELLOW}The postgres server and database must be accessible on the port 5432 on localhost{RESET}
{GREEN}./target/x86_64-unknown-linux-musl/release/{package_name}{RESET}
  {YELLOW}In the browser or in curl open{RESET}
{GREEN}http://localhost:8011/webpage_hit_counter/get_svg_image/555555.svg{RESET}
  {YELLOW}if {package_name} ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
    Ok(())
}

/// Run 'cargo doc', then copy to /docs/ folder, because this is a GitHub standard folder.
fn task_doc() -> anyhow::Result<()> {
    ts::task_doc()?;
    // message to help user with next move
    println!(
        r#"
  {YELLOW}After `cargo auto doc`, ctrl-click on `docs/index.html`. 
    It will show the index.html in VSCode Explorer, then right-click and choose "Show Preview".
    This works inside the CRUSTDE container, because of the extension "Live Preview" 
    <https://marketplace.visualstudio.com/items?itemName=ms-vscode.live-server>
    If ok then run the tests in code and the documentation code examples.{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
    Ok(())
}

/// Run 'cargo test'.
fn task_test() -> anyhow::Result<()> {
    cl::run_shell_command_static("cargo test")?;
    println!(
        r#"
  {YELLOW}After `cargo auto test`. If ok then {RESET}
  {YELLOW}(commit message is mandatory){RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
"#
    );
    Ok(())
}

/// Run 'commit' and 'push'. Separate docs and other updates.
fn task_commit_and_push(arg_2: Option<String>) -> anyhow::Result<()> {
    ts::task_commit_and_push(arg_2)?;
    println!(
        r#"
  {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_web{RESET}
"#
    );
    Ok(())
}

/// Publish to web for podman container and git tag.
fn task_publish_to_web() -> anyhow::Result<()> {
    println!(
        r#"
    {YELLOW}Use ssh-agent and ssh-add to store the ssh credentials.{RESET}
"#
    );

    let cargo_toml = cl::CargoToml::read()?;
    // let package_name = cargo_toml.package_name();
    let version = cargo_toml.package_version();
    // take care of tags
    let _tag_name_version = cl::git_tag_sync_check_create_push(&version)?;

    // rsync upload files to the existing remote transfer_folder
    cl::run_shell_command_static("rsync -e ssh -a --info=progress2 ./target/x86_64-unknown-linux-musl/release/webpage_hit_counter luciano_bestia@35.199.190.85:/var/www/transfer_folder/webpage_hit_counter/")?;
    cl::run_shell_command_static(
        "rsync -e ssh -a --info=progress2 ./.env luciano_bestia@35.199.190.85:/var/www/transfer_folder/webpage_hit_counter/",
    )?;
    cl::run_shell_command_static("rsync -e ssh -a --info=progress2 ./deploy/buildah_image_webpage_hit_counter.sh luciano_bestia@35.199.190.85:/var/www/transfer_folder/webpage_hit_counter/")?;
    cl::run_shell_command_static("rsync -e ssh -a --info=progress2 ./deploy/webpage_hit_counter_pod_create.sh luciano_bestia@35.199.190.85:/var/www/transfer_folder/webpage_hit_counter/")?;

    println!(
        r#"
    {YELLOW}After `cargo auto publish_to_web`, connect to the google VM bash using SSH.{RESET}
{GREEN}ssh -i ~/.ssh/ssh_certificate username@domain -v{RESET}
    {YELLOW}There are 2 possible way to install webpage_hit_counter on the server:{RESET}
    {YELLOW}1. standalone executable or{RESET}
    {YELLOW}2. podman container (it takes more space){RESET}

    {YELLOW}STANDALONE EXECUTABLE{RESET}
    {YELLOW}First stop the executable webpage_hit_counter in Zellij or Screen terminal multiplexer.{RESET}
{GREEN}mkdir -p ~/bin/hit_counter_and_env{RESET}
{GREEN}cp /var/www/transfer_folder/webpage_hit_counter/webpage_hit_counter ~/bin/hit_counter_and_env/webpage_hit_counter{RESET}
{GREEN}cp /var/www/transfer_folder/webpage_hit_counter/.env ~/bin/hit_counter_and_env/.env{RESET}
{GREEN}sudo chmod +x ~/bin/hit_counter_and_env/webpage_hit_counter{RESET}
{GREEN}cd ~/bin/hit_counter_and_env{RESET}
{GREEN}./webpage_hit_counter{RESET}

    {YELLOW}PODMAN CONTAINER{RESET}
    {YELLOW}Stop and remove the old pod for webpage_hit_counter{RESET}
{GREEN}podman pod rm -f webpage_hit_counter_pod{RESET}
    {YELLOW}There run the bash scripts to create the image and to create the pod.{RESET}
{GREEN}cd /var/www/transfer_folder/webpage_hit_counter{RESET}
{GREEN}sh buildah_image_webpage_hit_counter.sh{RESET}
{GREEN}sh webpage_hit_counter_pod_create.sh{RESET}
    {YELLOW}Test the postgres server:{RESET}
{GREEN}psql -h localhost -p 5432 -U admin -W{RESET}

    {YELLOW}In both cases test the web application locally:{RESET}
{GREEN}curl http://localhost:8011/webpage_hit_counter/get_svg_image/555555.svg{RESET}
    {YELLOW}Test the web application on the internet:{RESET}
{GREEN}curl https://bestia.dev/webpage_hit_counter/get_svg_image/555555.svg{RESET}
"#
    );
    Ok(())
}

/// Create a new release on github and uploads binary executables.
fn task_github_new_release() -> anyhow::Result<()> {
    ts::task_github_new_release()?;
    println!(
        r#"
  {YELLOW}No more automation tasks. {RESET}
"#
    );
    Ok(())
}
// endregion: tasks
