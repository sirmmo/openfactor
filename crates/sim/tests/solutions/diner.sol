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

# Chapter "Salad Days"

puzzle firstOrder
    crate 3 5 E
    board 4 5 E
    bowl 5 5 E
    table 6 5 E
    till 7 5 E
end

puzzle halfSet
    belt 5 4 E
    belt 7 4 E
    till 9 4 E
end

puzzle tinyPantry
    crate 3 5 E
    board 4 5 E
    splitter 5 5 E
    bowl 6 5 E
    table 7 5 E
    till 8 5 E
    bowl 5 4 N
    table 5 3 N
    till 5 2 N
end

puzzle sortingSchool
    crate 3 5 E
    board 4 5 E
    bowl 5 5 E
    table 6 5 E
    till 7 5 E
    research sorting
end

puzzle acrossTheHall
    crate 1 1 E
    board 2 1 E
    bowl 3 1 E
    belt 4..=17 1 E
    belt 18 1..=9 S
end

puzzle catering
    crate 3 5 E
    board 4 5 E
    bowl 5 5 E
    splitter 6 5 E
    table 7 5 E
    till 8 5 E
    belt 6..=7 4 E
    belt 8 4 S
end

puzzle backToFront
    remove 4 6
    board 4 6 E
    remove 5 6
    bowl 5 6 E
    remove 6 6
    table 6 6 E
end

puzzle connectTheDots
    belt 4 3..=2 N
    belt 4..=6 1 E
    belt 7 2..=5 S
    belt 7..=9 6 E
    belt 11..=13 6 E
    belt 14 6..=4 N
    belt 14 2..=1 N
end

puzzle terrace
    crate 3 4 E
    board 4 4 E
    splitter 5 4 E
    bowl 5 3 E
    bowl 5 5 E
    belt 6..=9 3 E
    belt 6..=9 5 E
    crate 3 8 E
    board 4 8 E
    splitter 5 8 E
    bowl 5 7 E
    bowl 5 9 E
    belt 6..=9 7 E
    belt 6..=9 9 E
end

puzzle rainyDay
    crate 3 1 E
    board 4 1 E
    bowl 5 1 E
    table 6 1 E
    till 7 1 E
    crate 3 3 E
    board 4 3 E
    bowl 5 3 E
    table 6 3 E
    till 7 3 E
    crate 3 5 E
    board 4 5 E
    bowl 5 5 E
    table 6 5 E
    till 7 5 E
    crate 3 7 E
    board 4 7 E
    bowl 5 7 E
    table 6 7 E
    till 7 7 E
end

# Chapter "Fire and Buns"

puzzle firstBurger
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    table 8 6 E
    till 9 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
end

puzzle bakery
    shelf 3 5 E
    mixer 4 5 E
    oven 5 5 E
    till 6 5 E
    gasPipe 5 4 E
    gasMain 4 4 E
end

puzzle cookingSchool
    crate 3 5 E
    board 4 5 E
    bowl 5 5 E
    till 6 5 E
    research grilling
end

puzzle cellarMain
    coldRoom 3 5 E
    grill 4 5 E
    till 5 5 E
    gasPipe 4 6..=12 E
    gasPipe 5..=19 12 E
end

puzzle burgerBudget
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    table 8 6 E
    till 9 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
end

puzzle finishBar
    gasPipe 4..=5 5 E
    belt 7 4..=5 S
    belt 5..=6 6 E
    pass 7 6 E
    belt 7 8..=7 N
end

puzzle threeTiles
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    table 8 6 E
    till 9 6 E
    crate 3 8 E
    board 4 8 E
    splitter 5 8 E
    belt 6 8 E
    belt 7 8..=7 N
    bowl 5 9 S
    table 5 10 S
    till 5 11 S
end

puzzle noCounter
    crate 2 6 E
    board 3 6 E
    pass 4 6 E
    table 5 6 E
    till 6 6 E
    coldRoom 3 5 E
    grill 4 5 S
    shelf 2 7 E
    mixer 3 7 E
    oven 4 7 N
    gasPipe 5..=7 5 E
    gasPipe 7 6..=7 E
    gasPipe 5..=6 7 E
    gasMain 8 6 E
end

puzzle oneOven
    shelf 3 6 E
    mixer 4 6 E
    oven 5 6 E
    splitter 6 6 E
    belt 6..=7 5 E
    belt 8 5 N
    belt 6..=7 7 E
    belt 8 7 S
    gasPipe 4..=5 5 E
    gasPipe 4..=5 7 E
    gasMain 3 5 E
    coldRoom 3 4 E
    grill 4 4 E
    belt 5..=7 4 E
    pass 8 4 E
    table 9 4 E
    till 10 4 E
    crate 3 2 E
    board 4 2 E
    belt 5..=7 2 E
    belt 8 2..=3 S
    coldRoom 3 8 E
    grill 4 8 E
    belt 5..=7 8 E
    pass 8 8 E
    table 9 8 E
    till 10 8 E
    crate 3 10 E
    board 4 10 E
    belt 5..=7 10 E
    belt 8 10..=9 N
end

puzzle climbTheMenu
    crate 3 5 E
    board 4 5 E
    bowl 5 5 E
    till 6 5 E
    research grilling
    crate 3 0 E
    board 4 0 E
    bowl 5 0 E
    table 6 0 E
    till 7 0 E
    remove 6 5
    table 6 5 E
    till 7 5 E
    shelf 3 7 E
    mixer 4 7 E
    oven 5 7 E
    belt 6 7 E
    belt 7 7..=8 S
    gasPipe 4..=5 8 E
    gasMain 6 8 E
    coldRoom 3 9 E
    grill 4 9 E
    belt 5..=6 9 E
    pass 7 9 E
    till 8 9 E
    crate 3 11 E
    board 4 11 E
    belt 5..=6 11 E
    belt 7 11..=10 N
    research baking
end

# Chapter "Stone Oven"

puzzle firstPizza
    crate 3 4 E
    board 4 4 E
    belt 5 4 S
    shelf 3 5 E
    mixer 4 5 E
    bench 5 5 E
    oven 6 5 E
    table 7 5 E
    till 8 5 E
    gasPipe 6 4 E
    gasMain 7 4 E
    fridge 3 6 E
    belt 4 6 E
    belt 5 6 N
end

puzzle benchJam
    remove 5 7
    remove 4 7
    remove 3 7
    crate 3 4 E
    board 4 4 E
    belt 5 4 S
end

puzzle takeawayOven
    shelf 3 5 E
    mixer 4 5 E
    splitter 5 5 E
    oven 6 5 E
    till 7 5 E
    gasPipe 6 6 E
    gasMain 7 6 E
    bench 5 4 E
    belt 6 4 S
    crate 3 4 E
    board 4 4 E
    fridge 3 3 E
    belt 4 3 E
    belt 5 3 S
end

puzzle doubleOven
    crate 3 4 E
    board 4 4 E
    belt 5 4 S
    shelf 3 5 E
    mixer 4 5 E
    bench 5 5 E
    splitter 6 5 E
    oven 7 5 E
    table 8 5 E
    till 9 5 E
    fridge 3 6 E
    belt 4 6 E
    belt 5 6 N
    oven 6 4 N
    table 6 3 N
    till 6 2 N
    gasPipe 7 4 E
    gasMain 8 4 E
end

puzzle gasTap
    crate 3 1 E
    board 4 1 E
    belt 5 1 S
    shelf 3 2 E
    mixer 4 2 E
    bench 5 2 E
    belt 6..=11 2 E
    oven 12 2 E
    table 13 2 E
    till 14 2 E
    fridge 3 3 E
    belt 4 3 E
    belt 5 3 N
end

puzzle bakingLicence
    remove 8 6
    splitter 8 6 E
    table 8 5 N
    till 8 4 N
    research baking
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    table 7 10 E
    till 8 10 E
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
end

puzzle scatteredPantry
    crate 0 7 E
    board 1 7 E
    belt 2..=7 7 E
    shelf 8 0 S
    mixer 8 1 S
    belt 8 2..=6 S
    fridge 8 14 N
    belt 8 13..=8 N
    bench 8 7 E
    oven 9 7 E
    table 10 7 E
    till 11 7 E
    gasPipe 9 6 E
    gasMain 10 6 E
end

puzzle changeOfMenu
    remove 5 4
    remove 6 4
    remove 7 4
    remove 7 5
    remove 4 5
    remove 5 5
    remove 4 6
    remove 5 6
    remove 6 6
    remove 7 6
    remove 6 5
    remove 8 6
    remove 9 6
    remove 3 8
    remove 4 8
    remove 5 8
    remove 6 8
    remove 7 8
    remove 7 7
    bench 5 4 E
    oven 6 4 E
    table 7 4 E
    till 8 4 E
    gasPipe 6 3 E
    gasMain 7 3 E
    crate 3 3 E
    board 4 3 E
    belt 5 3 S
    fridge 3 5 E
    belt 4 5 E
    belt 5 5 N
end

puzzle pizzaFund
    crate 3 4 E
    board 4 4 E
    belt 5 4 S
    shelf 3 5 E
    mixer 4 5 E
    bench 5 5 E
    oven 6 5 E
    table 7 5 E
    till 8 5 E
    gasPipe 6 4 E
    gasMain 7 4 E
    fridge 3 6 E
    belt 4 6 E
    belt 5 6 N
    crate 3 8 E
    board 4 8 E
    belt 5 8 S
    shelf 3 9 E
    mixer 4 9 E
    bench 5 9 E
    oven 6 9 E
    table 7 9 E
    till 8 9 E
    gasPipe 6 8 E
    gasMain 7 8 E
    fridge 3 10 E
    belt 4 10 E
    belt 5 10 N
end

puzzle fridayRush
    crate 3 3 E
    board 4 3 E
    belt 5 3 S
    shelf 3 4 E
    mixer 4 4 E
    bench 5 4 E
    splitter 6 4 E
    oven 7 4 E
    table 8 4 E
    till 9 4 E
    fridge 3 5 E
    belt 4 5 E
    belt 5 5 N
    oven 6 3 N
    table 6 2 N
    till 6 1 N
    gasPipe 7 3 E
    gasMain 8 3 E
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    splitter 6 10 E
    oven 7 10 E
    table 8 10 E
    till 9 10 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
    oven 6 9 N
    table 6 8 N
    till 6 7 N
    gasPipe 7 9 E
    gasMain 8 9 E
end

# Chapter "Order Up"

puzzle takeawayWindow
    table 12 6 E
    till 13 6 E
    research sorting
    wait 300
    table 12 7 S
    till 12 8 S
    remove 12 6
    sorter 12 6 E
    set 12 6 burger
end

puzzle coinsInTheSalad
    remove 13 5
    sorter 13 5 E
    set 13 5 salad
    till 13 6 S
end

puzzle misEnPlace
    coldRoom 3 6 E
    grill 4 6 E
    belt 5 6 E
    till 6 6 E
    gasPipe 4 5 E
    gasMain 3 5 E
    shelf 3 4 E
    mixer 4 4 E
    belt 5 4 E
    belt 6 4..=5 S
    crate 3 8 E
    board 4 8 E
    belt 5 8 E
    belt 6 8..=7 N
    fridge 3 10 E
    belt 4..=6 10 E
    belt 7 10..=7 N
    belt 7 6 W
end

puzzle tableForOne
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    belt 7..=9 10 E
    belt 10 10..=7 N
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    belt 8..=9 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    crate 3 1 E
    board 4 1 E
    bowl 5 1 E
    belt 6..=9 1 E
    belt 10 1..=5 S
end

puzzle twoWayOven
    shelf 3 5 E
    mixer 4 5 E
    splitter 5 5 E
    oven 6 5 E
    sorter 7 5 E
    set 7 5 pizza
    table 8 5 E
    till 9 5 E
    bench 5 4 E
    belt 6 4 S
    crate 3 4 E
    board 4 4 E
    fridge 3 3 E
    belt 4 3 E
    belt 5 3 S
    gasPipe 6 6 E
    gasMain 6 7 E
    belt 7 6..=8 S
    pass 7 9 E
    table 8 9 E
    till 9 9 E
    coldRoom 3 9 E
    grill 4 9 E
    belt 5..=6 9 E
    gasPipe 4..=6 8 E
    crate 3 11 E
    board 4 11 E
    belt 5..=6 11 E
    belt 7 11..=10 N
    wait 600
    set 6 5 bakeBuns
end

puzzle inspectionDay
    gasPipe 4 5 E
    remove 7 5
    belt 7 5 S
    remove 4 8
    board 4 8 E
    remove 8 6
    table 8 6 E
end

puzzle deliveryVan
    crate 0 0 S
    coldRoom 1 0 S
    belt 0..=22 1 E
    sorter 23 1 E
    set 23 1 meat
    grill 24 1 E
    gasPipe 24 0 E
    gasMain 25 0 E
    board 23 2 S
    bowl 23 3 S
end

puzzle seedMoney
    crate 3 1 E
    board 4 1 E
    bowl 5 1 E
    table 6 1 E
    till 7 1 E
    crate 3 4 E
    board 4 4 E
    belt 5 4 S
    shelf 3 5 E
    mixer 4 5 E
    bench 5 5 E
    fridge 3 6 E
    belt 4 6 E
    belt 5 6 N
    table 7 5 E
    till 8 5 E
    gasMain 7 4 E
    gasPipe 6 4 E
    oven 6 5 E
end

puzzle oneOfEach
    shelf 3 6 E
    mixer 4 6 E
    belt 5 6 E
    splitter 6 6 E
    belt 6 5..=4 N
    oven 6 3 E
    belt 7..=8 3 E
    pass 9 3 E
    table 10 3 E
    till 11 3 E
    coldRoom 3 1 E
    grill 4 1 E
    belt 5..=8 1 E
    belt 9 1..=2 S
    gasPipe 4..=5 2 E
    gasPipe 5 3 E
    gasMain 3 2 E
    belt 6 7..=8 S
    bench 6 9 E
    oven 7 9 E
    table 8 9 E
    till 9 9 E
    gasPipe 7 8 E
    gasMain 8 8 E
    fridge 3 9 E
    belt 4..=5 9 E
    crate 3 11 E
    board 4 11 E
    splitter 5 11 E
    belt 6 11..=10 N
    belt 5..=9 12 E
    belt 10 12..=5 N
    belt 10 4 W
    belt 9 4 N
end

puzzle mixedDelivery
    crate 0 0 S
    shelf 1 0 S
    coldRoom 2 0 S
    belt 0..=24 1 E
    splitter 25 1 E
    belt 26..=27 1 E
    splitter 28 1 E
    belt 29..=30 1 E
    splitter 31 1 E
    belt 32..=33 1 E
    till 34 1 E
    grill 25 2 S
    gasPipe 24 2 E
    gasMain 23 2 E
    belt 25 3..=5 S
    belt 25..=27 6 E
    board 28 2 S
    belt 28 3..=5 S
    mixer 31 2 S
    oven 31 3 S
    gasPipe 32 3 E
    gasMain 33 3 E
    belt 31 4..=6 S
    belt 31..=29 7 W
    belt 28 7 N
    pass 28 6 E
end

# Chapter "Dinner Rush"

puzzle spillOver
    remove 5 6
    splitter 5 6 N
    till 6 6 E
end

puzzle mainLine
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasPipe 3..=1 5 E
    gasPipe 1 4..=0 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    table 8 6 E
    till 9 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    table 7 10 E
    till 8 10 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
    gasPipe 6 11..=12 E
    gasPipe 5..=1 12 E
    gasPipe 1 11..=6 E
end

puzzle longShelf
    coldRoom 2 0 S
    grill 2 1 S
    belt 2 2..=3 S
    pass 2 4 S
    table 2 5 S
    till 2 6 S
    shelf 4 0 S
    mixer 4 1 S
    oven 4 2 S
    belt 4 3 S
    belt 4..=3 4 W
    gasPipe 3 1..=2 E
    gasMain 3 3 E
    crate 0 0 S
    board 0 1 S
    belt 0 2..=3 S
    belt 0..=1 4 E
    fridge 5 0 S
    shelf 6 0 S
    crate 7 0 S
    board 7 1 S
    belt 7 2 W
    mixer 6 1 S
    bench 6 2 S
    oven 6 3 S
    table 6 4 S
    till 6 5 S
    gasPipe 7 3 E
    gasMain 7 4 E
    belt 5 1 S
    belt 5 2 E
end

puzzle stocktake
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    splitter 8 6 E
    table 9 6 E
    till 10 6 E
    till 8 5 N
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    splitter 7 10 E
    table 8 10 E
    till 9 10 E
    till 7 11 S
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
end

puzzle marketCart
    crate 0 0 S
    shelf 1 0 S
    fridge 2 0 S
    belt 0..=19 1 E
    splitter 20 1 E
    belt 21..=22 1 E
    splitter 23 1 E
    belt 24..=25 1 E
    sorter 26 1 E
    set 26 1 cheese
    splitter 27 1 E
    till 26 2 S
    till 27 2 S
    board 20 2 S
    belt 20 3..=5 S
    belt 20..=22 6 E
    mixer 23 2 S
    belt 23 3..=5 S
    bench 23 6 E
    oven 24 6 E
    gasPipe 24 7 E
    gasMain 25 7 E
    belt 28 1..=7 S
    belt 28..=24 8 W
    belt 23 8..=7 N
end

puzzle oneTill
    crate 3 1 E
    board 4 1 E
    bowl 5 1 E
    table 6 1 E
    belt 7..=11 1 E
    belt 12 1..=5 S
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    table 8 6 E
    belt 9..=11 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    table 7 10 E
    belt 8..=11 10 E
    belt 12 10..=7 N
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
end

puzzle tastingMenu
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    table 8 6 E
    till 9 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    table 7 10 E
    till 8 10 E
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
end

puzzle fromScratch
    crate 3 0 E
    board 4 0 E
    bowl 5 0 E
    table 6 0 E
    till 7 0 E
    crate 3 1 E
    board 4 1 E
    bowl 5 1 E
    till 6 1 E
    research grilling
    crate 3 2 E
    board 4 2 E
    bowl 5 2 E
    table 6 2 E
    till 7 2 E
    remove 6 1
    table 6 1 E
    till 7 1 E
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    till 8 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    research baking
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    table 7 10 E
    till 8 10 E
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
end

puzzle officeLunch
    crate 3 1 E
    board 4 1 E
    bowl 5 1 E
    belt 6..=8 1 E
    belt 9 1..=5 S
    till 9 6 E
    shelf 3 4 E
    mixer 4 4 E
    oven 5 4 E
    belt 6 4 E
    belt 7 4..=5 S
    gasPipe 4..=5 5 E
    gasMain 6 5 E
    coldRoom 3 6 E
    grill 4 6 E
    belt 5..=6 6 E
    pass 7 6 E
    belt 8 6 E
    crate 3 8 E
    board 4 8 E
    belt 5..=6 8 E
    belt 7 8..=7 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    oven 6 10 E
    belt 7..=8 10 E
    belt 9 10..=7 N
    gasPipe 6 9 E
    gasMain 7 9 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
end

puzzle closingTime
    crate 3 0 E
    board 4 0 E
    bowl 5 0 E
    table 6 0 E
    till 7 0 E
    shelf 3 1 E
    mixer 4 1 E
    oven 5 1 E
    belt 6 1 E
    belt 7 1..=2 S
    gasPipe 4..=5 2 E
    gasMain 6 2 E
    coldRoom 3 3 E
    grill 4 3 E
    belt 5..=6 3 E
    pass 7 3 E
    table 8 3 E
    till 9 3 E
    crate 3 5 E
    board 4 5 E
    belt 5..=6 5 E
    belt 7 5..=4 N
    crate 3 9 E
    board 4 9 E
    belt 5 9 S
    shelf 3 10 E
    mixer 4 10 E
    bench 5 10 E
    splitter 6 10 E
    oven 7 10 E
    table 8 10 E
    till 9 10 E
    fridge 3 11 E
    belt 4 11 E
    belt 5 11 N
    oven 6 9 N
    table 6 8 N
    till 6 7 N
    gasPipe 7 9 E
    gasMain 8 9 E
    crate 3 6 E
    board 4 6 E
    bowl 5 6 E
    table 6 6 E
    till 7 6 E
end
