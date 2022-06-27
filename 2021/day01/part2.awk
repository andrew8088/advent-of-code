BEGIN {
    count=-1;
    prevSum=-1;
    prev1=-1;
    prev2=-1;
    prev3=-1;
}
{
    prev1=prev2
    prev2=prev3
    prev3=$0

    if (prev1 > -1 && prev2 > -1 && prev3 > -1) {
        sum = prev1 + prev2 + prev3
        if (sum > prevSum) count++;
        prevSum=sum;
    }
}
END {
    print count
}
