#!/bin/bash

# Array of machine names (edit this list)
MACHINES=("robot-123e4567-e89b-12d3-a456-426614174000" "robot-550e8400-e29b-41d4-a716-446655440000" "robot-6ba7b810-9dad-11d1-80b4-00c04fd430c8")

# Local source file
SRC_FILE="metrics.csv"
TOTAL_FILE="all_metrics.csv"
DATE=$(date +%Y%m%d)
DATE_TIME=$(date +%Y%m%d_%H%M%S)
ls -l ./*${SRC_FILE}  # List existing metrics files for reference
# Loop: SCP from each machine to local
for machine in "${MACHINES[@]}"; do
    RANDOM_NUM=$((1 + RANDOM % 1000))  # Generate random number for demo
    echo "Fetching $SRC_FILE from $machine..."
    touch "./${machine}_${DATE_TIME}_${SRC_FILE}"  # Create empty file to simulate scp
    echo  "${machine},${DATE_TIME},${RANDOM_NUM}" >>"./${machine}_${DATE_TIME}_${SRC_FILE}"  # Simulate fetching file (replace with actual scp command)
    if [ $? -eq 0 ]; then
        echo "✓ Success from $machine"
    else
        echo "✗ Failed from $machine"
    fi
    cat "./${machine}_${DATE}_${SRC_FILE}" >> "./${TOTAL_FILE}"  # Append to combined file
done
ls -l ./*${SRC_FILE}
echo "All metrics collected into ${TOTAL_FILE}"
echo "Contents of ${TOTAL_FILE}:"
cat "${TOTAL_FILE}"