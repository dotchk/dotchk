use colored::*;
use dotchk::CheckResult;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;
use std::time::Duration;

// Global flag for quiet mode
pub static QUIET_MODE: AtomicBool = AtomicBool::new(false);

// Professional color scheme
static AVAILABLE_COLOR: LazyLock<Color> = LazyLock::new(|| Color::BrightGreen);
static TAKEN_COLOR: LazyLock<Color> = LazyLock::new(|| Color::BrightRed);
static ERROR_COLOR: LazyLock<Color> = LazyLock::new(|| Color::Yellow);
static INFO_COLOR: LazyLock<Color> = LazyLock::new(|| Color::BrightBlue);
static DIM_COLOR: LazyLock<Color> = LazyLock::new(|| Color::BrightBlack);
static HEADER_COLOR: LazyLock<Color> = LazyLock::new(|| Color::BrightWhite);

pub fn set_quiet_mode(quiet: bool) {
    QUIET_MODE.store(quiet, Ordering::Relaxed);
}

pub fn is_quiet() -> bool {
    QUIET_MODE.load(Ordering::Relaxed)
}

/// Print a header section
pub fn print_header(text: &str) {
    if is_quiet() {
        return;
    }
    println!("\n{}", text.color(*HEADER_COLOR).bold());
    println!("{}", "─".repeat(text.len()).color(*DIM_COLOR));
}

/// Print an info message
pub fn print_info(text: &str) {
    if is_quiet() {
        return;
    }
    println!("INFO: {text}");
}

/// Print a success message
pub fn print_success(text: &str) {
    println!("[OK] {text}");
}

/// Print a warning message
pub fn print_warning(text: &str) {
    if is_quiet() {
        return;
    }
    eprintln!("WARNING: {text}");
}

/// Format a domain result with colors
pub fn format_domain_result(result: &CheckResult) -> String {
    let status = if result.available {
        "AVAILABLE".color(*AVAILABLE_COLOR).bold()
    } else {
        "TAKEN".color(*TAKEN_COLOR)
    };

    format!(
        "{} {} {}",
        result.domain,
        "→".color(*DIM_COLOR),
        status
    )
}

/// Format a domain error with colors
pub fn format_domain_error(domain: &str, error: &str) -> String {
    format!(
        "{} {} {}",
        domain.color(*DIM_COLOR),
        "→".color(*DIM_COLOR),
        format!("ERROR: {error}").color(*ERROR_COLOR)
    )
}

/// Format a compact domain result for TLD checking
pub fn format_tld_result(result: &CheckResult, indent: bool, max_domain_width: usize) -> String {
    let prefix = if indent { "  " } else { "" };

    let (status, color) = if result.available {
        ("AVAILABLE", *AVAILABLE_COLOR)
    } else {
        ("TAKEN", *TAKEN_COLOR)
    };

    format!(
        "{}{:<width$} {} {}",
        prefix,
        result.domain,
        "→".color(*DIM_COLOR),
        status.color(color),
        width = max_domain_width
    )
}

/// Format a TLD error result
pub fn format_tld_error(domain: &str, error: &str, indent: bool, max_domain_width: usize) -> String {
    let prefix = if indent { "  " } else { "" };

    format!(
        "{}{:<width$} {} {}",
        prefix,
        domain,
        "→".color(*DIM_COLOR),
        format!("ERROR: {error}").color(*ERROR_COLOR),
        width = max_domain_width
    )
}

/// Create a progress bar for batch operations
pub fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg}\n{wide_bar:.cyan/blue} {pos}/{len} ({percent}%) {eta_precise}")
            .expect("hardcoded template is valid")
            .progress_chars("█▓▒░")
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a spinner for indeterminate progress
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("hardcoded template is valid")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Print statistics in a clean table format
pub fn print_statistics(stats: &dotchk::export::Stats) {
    if is_quiet() {
        return;
    }

    print_header("Statistics");

    // Summary section
    println!("{}", "Summary".color(*DIM_COLOR));
    println!(
        "  Total checked    {}",
        stats.total.to_string().color(*HEADER_COLOR).bold()
    );
    println!(
        "  Available        {} {}",
        stats.available.to_string().color(*AVAILABLE_COLOR).bold(),
        format!(
            "({:.1}%)",
            (stats.available as f64 / stats.total as f64) * 100.0
        )
        .color(*DIM_COLOR)
    );
    println!(
        "  Taken            {} {}",
        stats.unavailable.to_string().color(*TAKEN_COLOR),
        format!(
            "({:.1}%)",
            (stats.unavailable as f64 / stats.total as f64) * 100.0
        )
        .color(*DIM_COLOR)
    );

    if stats.errors > 0 {
        println!(
            "  Errors           {} {}",
            stats.errors.to_string().color(*ERROR_COLOR),
            format!(
                "({:.1}%)",
                (stats.errors as f64 / stats.total as f64) * 100.0
            )
            .color(*DIM_COLOR)
        );
    }

}

/// Print statistics in a clean table format for TLD command
pub fn print_tld_statistics(stats: &dotchk::export::Stats) {
    if is_quiet() {
        return;
    }

    print_header("Statistics");

    // Summary section
    println!("{}", "Summary".color(*DIM_COLOR));
    println!(
        "  Total checked    {}",
        stats.total.to_string().color(*HEADER_COLOR).bold()
    );
    println!(
        "  Available        {} {}",
        stats.available.to_string().color(*AVAILABLE_COLOR).bold(),
        format!(
            "({:.1}%)",
            (stats.available as f64 / stats.total as f64) * 100.0
        )
        .color(*DIM_COLOR)
    );
    println!(
        "  Taken            {} {}",
        stats.unavailable.to_string().color(*TAKEN_COLOR),
        format!(
            "({:.1}%)",
            (stats.unavailable as f64 / stats.total as f64) * 100.0
        )
        .color(*DIM_COLOR)
    );

    if stats.errors > 0 {
        println!(
            "  Errors           {} {}",
            stats.errors.to_string().color(*ERROR_COLOR),
            format!(
                "({:.1}%)",
                (stats.errors as f64 / stats.total as f64) * 100.0
            )
            .color(*DIM_COLOR)
        );
    }

    // Add the TLD-specific summary line
    println!();
    println!(
        "  {} {} {} {}",
        "Registered in".color(*DIM_COLOR),
        stats.unavailable.to_string().color(*TAKEN_COLOR).bold(),
        format!("of {} TLDs", stats.total).color(*DIM_COLOR),
        format!(
            "({:.1}%)",
            (stats.unavailable as f64 / stats.total as f64) * 100.0
        )
        .color(*HEADER_COLOR)
        .bold()
    );

}

/// Print a footer note
pub fn print_footer_note() {
    if is_quiet() {
        return;
    }
    println!("\n{}", 
        "Note: 'Available' indicates no NS records found. Always verify with WHOIS before purchasing."
            .color(*DIM_COLOR)
            .italic()
    );
}

/// Print export success message
pub fn print_export_success(filename: &str, count: usize) {
    print_success(&format!(
        "Exported {} results to {}",
        count.to_string().bold(),
        filename.color(*INFO_COLOR).underline()
    ));
}
