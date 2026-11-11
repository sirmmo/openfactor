# Solutions for the diner's puzzles: see ../puzzles.rs.

puzzle saturday
    # Burgers: patties from the grill, buns from the oven, chopped veg from below.
    coldRoom 3 6 E
    belt 4..=5 6 E
    grill 6 6 E
    belt 7 6 E
    pass 8 6 E
    table 9 6 E
    till 10 6 E
    shelf 3 4 E
    mixer 4 4 E
    belt 5 4 E
    oven 6 4 E
    belt 7 4 E
    belt 8 4 S
    belt 8 5 S
    gasMain 5 5 E
    gasPipe 6 5 E
    crate 3 8 E
    board 4 8 E
    belt 5..=7 8 E
    belt 8 8 N
    belt 8 7 N
    # Pizza: the bench takes dough, cheese and chopped veg; the oven bakes it.
    shelf 3 10 E
    mixer 4 10 E
    belt 5 10 E
    bench 6 10 E
    oven 7 10 E
    table 8 10 E
    till 9 10 E
    gasMain 8 9 E
    gasPipe 7 9 E
    fridge 3 11 E
    belt 4..=5 11 E
    belt 6 11 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 E
    belt 6 9 S
end
