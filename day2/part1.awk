/forward/ { x += $2 }
/down/ { y += $2 }
/up/ { y -= $2 }
END { print x*y }
