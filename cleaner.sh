#!/bin/bash

# Function to convert bytes to gigabytes
bytes_to_gb() {
  echo "scale=2; $1 / 1024 / 1024 / 1024" | bc
}

# Prompt the user for the directory containing the Rust projects
read -p "Enter the directory containing the Rust projects: " PROJECTS_DIR

# Check if the directory exists
if [ ! -d "$PROJECTS_DIR" ]; then
  echo "Directory $PROJECTS_DIR does not exist."
  exit 1
fi

# Calculate initial disk usage
initial_usage=$(du -sb "$PROJECTS_DIR" | awk '{print $1}')

# Iterate over all subdirectories in the projects directory
for dir in "$PROJECTS_DIR"/*/; do
  if [ -d "$dir" ]; then
    echo "Cleaning project in directory: $dir"
    (cd "$dir" && cargo clean)
  fi
done

# Calculate final disk usage
final_usage=$(du -sb "$PROJECTS_DIR" | awk '{print $1}')

# Calculate freed space
freed_space=$(($initial_usage - $final_usage))
freed_space_gb=$(bytes_to_gb $freed_space)

# Display the freed space
echo "Total freed space: $freed_space_gb GB"

