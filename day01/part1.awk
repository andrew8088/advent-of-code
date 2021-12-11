BEGIN {
    count=-1;
    prev=0
}
{
    if ($0 > prev) count++;
    prev=$0
}
END {
    print count
}
