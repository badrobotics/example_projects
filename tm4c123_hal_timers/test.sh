#!/usr/bin/env bash

set -euf -o pipefail

serial_port="${1}"
test_time="${2:-5}" # Default to 5 seconds of test time if the second argument isn't given

stty -F "$serial_port" 115200

(
    tt="$test_time"
    while (( $tt > 0 )); do
        echo -n -e  "Gathering $tt seconds of data...\r"
        sleep 1
        tt=$(( $tt - 1 ))
    done
) &

(cat "$serial_port" & pid=$!; sleep "$test_time"; kill $pid) | gawk '
    BEGIN {
        PROCINFO["sorted_in"] = "@ind_num_asc"
    }

    /Timer/ {
        counts[$2] += 1
    }

    END {
        largest_count = -1
        for (timer in counts) {
            c = counts[timer]
            largest_count = c > largest_count ? c : largest_count
        }

        smallest_count = largest_count
        for (timer in counts) {
            c = counts[timer]
            smallest_count = c < smallest_count ? c : smallest_count
        }

        printf("\n")
        for (timer in counts) {
            printf("Timer %d: counts=%d, ratio=%f\n", timer, counts[timer], counts[timer]/smallest_count)
        }
    }
'

