#!/bin/bash
# This is an example script that can be executed as part of the Fedbox CLI application.

echo "Running example_script.sh"
# Add your commands below
# For example, you can list files in the current directory
ls -la

# You can also add more complex logic as needed
# Example: Check if a specific file exists
if [ -f "some_file.txt" ]; then
    echo "some_file.txt exists."
else
    echo "some_file.txt does not exist."
fi

# End of script