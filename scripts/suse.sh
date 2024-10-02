#!/bin/bash

# This script is oriented towards the specific custom fields in SUSE's NetBox instance.
# It mainly collects information about the CPU, RAM and power draw.
#
# You can write your own collection script for whatever data you want in the "Custom Fields" section of your
# NetBox instance as long as it returns valid JSON.
#
# Note that you are responsible for the validity of the fields you collect as there is no way for us to check what
# you have configured.

# Declare an associative array to store system information
declare -A system_info

# Function to print usage instructions
usage() {
    echo "Usage: $0"
    exit 1
}

# Check if jq is installed (even though jq is not needed anymore, keeping this in case you plan to use jq elsewhere)
if ! command -v jq &> /dev/null; then
    echo "jq is required. Please install it."
    exit 1
fi

# Get CPU Architecture
system_info["CPU_Architecture"]=$(uname -m)

# Get Max Memory Capacity (TB) (This might not be available on all systems)
max_capacity=$(lscpu | grep "Max memory size:" | awk '{print $NF}')
system_info["Max_Capacity_TB"]="${max_capacity%T}"

# Get CPU Model Name
cpu_type=$(lscpu | grep "Model name:" | head -n 1 | cut -d':' -f2 | sed 's/^[[:space:]]*//')
system_info["CPU_Type"]="$cpu_type"

# Get Max Power Consumption (Watt) (This will only work if the path exists)
if [[ -f /sys/class/power_supply/BAT0/power_now ]]; then
    max_power=$(cat /sys/class/power_supply/BAT0/power_now)
    system_info["Max_Power_Watt"]="${max_power% }"
else
    system_info["Max_Power_Watt"]="N/A"
fi

# Get Total RAM Size (GB)
total_ram=$(free -b | grep Mem | awk '{print $2/1024^3}')
system_info["RAM_GB"]="${total_ram%.*}"

# Get CPU Sockets, CPU Cores, and CPU Threads from lscpu
cpu_sockets=$(lscpu | grep "Socket(s):" | awk '{print $2}')
cpu_cores=$(lscpu | grep "Core(s) per socket:" | awk '{print $4}')
cpu_threads=$(lscpu | grep "Thread(s) per core:" | awk '{print $4}')

# Store CPU configuration in an associative array
cpu_config="{\"CPU_Sockets\": \"$cpu_sockets\", \"CPU_Cores\": \"$cpu_cores\", \"CPU_Threads\": \"$cpu_threads\"}"
system_info["CPU_Configuration"]="$cpu_config"

# Print the final JSON output
echo "System Information:"
cat << EOF
{
  "CPU_Architecture": "${system_info["CPU_Architecture"]}",
  "Max_Capacity_TB": "${system_info["Max_Capacity_TB"]}",
  "CPU_Type": "${system_info["CPU_Type"]}",
  "Max_Power_Watt": "${system_info["Max_Power_Watt"]}",
  "RAM_GB": "${system_info["RAM_GB"]}",
  "CPU_Configuration": ${system_info["CPU_Configuration"]}
}
EOF
