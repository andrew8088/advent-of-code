/forward/ { x += $2; y += a*$2 }
/down/ { a += $2 }
/up/ { a -= $2 }
END { print x*y }
