# looming

A simple deadline tracker for managing items with upcoming due dates.

## Overview

`looming` helps you track deadlines by storing items with associated dates, notes, and URLs. Deadlines can be marked as hard (firm) or soft (flexible), and you can quickly see what's coming up.

## Features

- Add items with deadlines (hard or soft)
- Optional notes and URLs for each item
- View upcoming items within a specified time window
- Calculate days remaining until deadlines

## Installation

```bash
cargo install --path .
```

## Building

```bash
cargo build --release
```

## Storage

Items are stored locally in TOML format.
