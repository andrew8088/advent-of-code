BEGIN { FS = "" }
{
    for (i = 1; i <= NF; i++)
        if ($i == 0)
            c[i] += 1
}

END {
    for (i = 1; i <= NF; i++) {
        if (c[i] > NR - c[i]) {
            gamma = gamma "0"
            epsilon = epsilon "1"
        } else {
            gamma = gamma "1"
            epsilon = epsilon "0"
        }
    }
    # pipe this output to `bc` to calculate
    print "obase=10;ibase=2;" gamma "*" epsilon
}
