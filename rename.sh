#!/usr/bin/env bash
# Run this from the root of the rustodo project
# Usage: bash rename.sh

set -e
cd src/commands

# Rename task files
mv add.rs        task_add.rs
mv edit.rs       task_edit.rs
mv done.rs       task_done.rs
mv undone.rs     task_undone.rs
mv remove.rs     task_remove.rs
mv list.rs       task_list.rs
mv info.rs       task_info.rs
mv deps.rs       task_deps.rs
mv clear.rs      task_clear.rs
mv clear_recur.rs task_clear_recur.rs
mv recur.rs      task_recur.rs

# Remove legacy file (replaced by project_list.rs + project_show.rs)
rm projects.rs

echo "Done. Don't forget to update main.rs imports."
