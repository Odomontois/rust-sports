fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().map(|&(i, o)| i - o).sum()
}
