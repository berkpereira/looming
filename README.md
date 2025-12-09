# looming

A simple deadline tracker for managing items with upcoming due dates.

## Overview

`looming` helps you track deadlines by storing items with associated dates, notes, and URLs. Deadlines can be marked as hard (firm) or soft (flexible), and you can quickly see what's coming up.

## Platform Support

**macOS only** — This tool currently stores data in `~/Library/Application Support/looming/`, which is macOS-specific. Linux and Windows support would require platform-specific storage paths.

## Features

- Add items with deadlines (hard or soft)
- Optional notes and URLs for each item
- List all tracked deadlines
- Display individual item details
- Remove items by name
- Extend soft deadlines by a specified number of days
- View upcoming items within a specified time window (default: 14 days)
- Color-coded output based on deadline proximity

## Installation

```bash
cargo install --path .
```

## Building

```bash
cargo build --release
```

## Usage

```bash
# Add a new deadline
looming add --name "Project proposal" --deadline 25-12-2025 --type hard --note "Final submission"

# Add with optional URL
looming add --name "Conference" --deadline 15-01-2026 --url "https://example.com"

# List all items
looming list

# Display a specific item
looming display "Project proposal"

# Show items due in the next N days (default: 14)
looming in 30

# Extend a soft deadline by N days (default: 7)
looming extend --name "Draft review" --days 14

# Remove an item
looming remove "Project proposal"
```

## Storage

Items are stored locally in TOML format at `~/Library/Application Support/looming/looming-data.toml`.
