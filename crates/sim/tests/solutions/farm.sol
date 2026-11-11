# Solutions for the farm pack's puzzles: see ../puzzles.rs.

puzzle theSpring
    pump 4 4 E
    pipe 5 4 E
    farm 6 4 E
    belt 7..=9 4 E
end

puzzle farWater
    pump 24 3 E
    pipe 6..=23 3 E
    pipe 6 2 E
    pipe 6 4..=5 S
    farm 5 2 W
    farm 5 3 W
    farm 5 4 W
    farm 5 5 W
    belt 4 2..=7 S
end

puzzle fromStumps
    woodcutter 2 1 E
    sawmill 3 1 E
    pump 12 1 E
    pipe 11 0..=2 S
    farm 10 0 W
    farm 10 1 W
    farm 10 2 W
    belt 9 0..=3 S
    belt 9..=5 4 W
    belt 4 4..=2 N
end

puzzle snugPlot
    pump 5 4 E
    pipe 5 5 E
    farm 4 5 S
    farm 6 5 S
    farm 5 6 W
end

puzzle crookedRows
    remove 6 3
    remove 8 3
    remove 7 4
    pump 9 3 E
    pipe 6..=8 3 E
    farm 7 4 S
    remove 5 3
    farm 8 4 S
    remove 1 1
    remove 2 1
    remove 3 1
    remove 4 1
    farm 6 4 S
end

puzzle acrossTheTrack
    pump 10 2 E
    pipe 1..=9 2 E
    pipe 1 3..=7 S
    pipe 2..=12 7 E
    farm 8 8 S
    farm 9 8 S
    farm 10 8 S
    farm 11 8 S
    farm 12 8 S
    belt 8..=19 9 E
    belt 20 9..=6 N
end

puzzle twoOrders
    market 7 2 E
    woodcutter 5 2 E
    sawmill 6 2 E
    woodcutter 5 1 E
    belt 6 1 E
    sawmill 7 1 S
    woodcutter 5 3 E
    belt 6 3 E
    sawmill 7 3 N
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=9 1 W
    belt 8 1 S
    belt 8 2 W
    farm 17 2 N
    farm 16 2 N
    farm 15 2 N
    farm 14 2 N
    farm 13 2 N
    farm 12 2 N
    farm 11 2 N
end

puzzle fullBarns
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=9 5 W
    farm 17 4 S
    farm 16 4 S
    farm 15 4 S
    farm 14 4 S
    farm 13 4 S
end

puzzle twoSprings
    pump 1 5 E
    pump 18 5 E
    pipe 2..=17 5 E
    belt 2..=17 3 E
    belt 2..=17 7 E
    farm 17 4 N
    farm 17 6 S
    farm 16 4 N
    farm 16 6 S
    farm 15 4 N
    farm 15 6 S
    farm 14 4 N
    farm 14 6 S
    farm 13 4 N
    farm 13 6 S
    farm 12 4 N
    farm 12 6 S
    farm 11 4 N
    farm 11 6 S
    farm 10 4 N
    farm 10 6 S
    farm 9 4 N
    farm 9 6 S
    farm 8 4 N
    farm 8 6 S
end

puzzle learnMilling
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=10 5 W
    belt 9 5 W
    farm 17 4 S
    farm 16 4 S
    farm 15 4 S
    farm 14 4 S
    research milling
    remove 9 5
    mill 9 5 W
end

puzzle firstLoaf
    pump 8 1 E
    pipe 7 0..=2 S
    farm 6 0 W
    farm 6 1 W
    farm 6 2 W
    belt 5 0..=2 S
    mill 5 3 S
    bakery 5 4 S
    woodcutter 1 1 S
    belt 1 2..=3 S
    belt 1..=4 4 E
end

puzzle handToHand
    pump 2 5 E
    pipe 3 4..=6 S
    pipe 4 4 E
    pipe 4 6 E
    farm 4 5 E
    farm 5 4 S
    farm 5 6 N
    mill 5 5 E
    bakery 6 5 E
    woodcutter 6 4 S
end

puzzle oneTree
    woodcutter 3 3 E
    splitter 4 3 E
    sawmill 5 3 E
    belt 6 3 E
    belt 4 4 S
    belt 4..=6 5 E
    pump 14 5 E
    pipe 13 4..=6 S
    belt 11 4 S
    belt 11 6 N
    belt 11..=9 5 W
    farm 12 4 W
    farm 12 5 W
    farm 12 6 W
    mill 8 5 W
    bakery 7 5 N
    belt 7 4 N
end

puzzle jammedOven
    remove 6 3
    remove 6 4
    remove 6 5
    belt 6..=8 3 E
    belt 9 3..=5 S
end

puzzle flourAndBread
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=11 5 W
    mill 10 5 W
    splitter 9 5 W
    bakery 9 6 W
    farm 17 4 S
    farm 16 4 S
    farm 15 4 S
    farm 14 4 S
    woodcutter 5 3 E
    belt 6 3 E
    belt 7 3..=6 S
    belt 7..=8 7 E
    belt 9 7 N
end

puzzle bakeSale
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=11 5 W
    mill 10 5 W
    bakery 9 5 W
    farm 17 4 S
    farm 16 4 S
    farm 15 4 S
    woodcutter 5 3 E
    belt 6 3 E
    belt 7 3..=6 S
    belt 7..=8 7 E
    belt 9 7..=6 N
end

puzzle longStudies
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=9 5 W
    farm 17 4 S
    farm 16 4 S
    farm 15 4 S
    research milling
    remove 10 5
    mill 10 5 W
    remove 9 5
    bakery 9 5 W
    woodcutter 5 3 E
    belt 6 3 E
    belt 7 3..=6 S
    belt 7..=8 7 E
    belt 9 7..=6 N
    research dairying
end

puzzle overTheHill
    pump 27 1 E
    pipe 26 0..=3 S
    farm 25 0 W
    farm 25 1 W
    farm 25 2 W
    farm 25 3 W
    belt 24 0..=3 S
    mill 24 4 W
    belt 23..=5 4 W
    bakery 4 4 S
    woodcutter 2 2 E
    belt 3 2 E
    belt 4 2..=3 S
end

puzzle allThree
    pump 3 1 E
    pipe 4..=11 1 E
    belt 6 3..=7 S
    belt 6 8 E
    belt 5 3 E
    farm 5 2 S
    farm 6 2 S
    belt 8 3 W
    belt 7 3..=4 S
    mill 7 5 S
    belt 7 6..=7 S
    farm 7 2 S
    farm 8 2 S
    belt 9 3 E
    belt 10 3..=4 S
    mill 10 5 S
    bakery 10 6 S
    belt 10 7 S
    belt 10..=8 8 W
    farm 9 2 S
    farm 10 2 S
    woodcutter 13 6 W
    belt 12..=11 6 W
end

puzzle bakersRow
    pump 1 2 E
    pipe 2..=11 2 E
    belt 2..=11 7 E
    farm 2 3 S
    farm 3 3 S
    belt 2 4 E
    belt 3 4 S
    mill 3 5 S
    bakery 3 6 S
    woodcutter 2 6 E
    farm 4 3 S
    farm 5 3 S
    belt 4 4 E
    belt 5 4 S
    mill 5 5 S
    bakery 5 6 S
    woodcutter 4 6 E
    farm 6 3 S
    farm 7 3 S
    belt 6 4 E
    belt 7 4 S
    mill 7 5 S
    bakery 7 6 S
    woodcutter 6 6 E
    farm 8 3 S
    farm 9 3 S
    belt 8 4 E
    belt 9 4 S
    mill 9 5 S
    bakery 9 6 S
    woodcutter 8 6 E
    farm 10 3 S
    farm 11 3 S
    belt 10 4 E
    belt 11 4 S
    mill 11 5 S
    bakery 11 6 S
    woodcutter 10 6 E
end

puzzle firstPress
    picker 2 1 E
    press 3 1 E
    belt 4..=5 1 E
    picker 2 0 E
    belt 3 0 S
    picker 2 2 E
    belt 3 2 N
end

puzzle orchardSquare
    press 5 4 S
    picker 5 3 S
    picker 4 4 E
    press 6 5 W
    picker 7 5 W
    picker 6 4 S
    press 5 6 N
    picker 5 7 N
    picker 6 6 W
    press 4 5 E
    picker 3 5 E
    picker 4 6 N
end

puzzle firstMilk
    pump 0 5 E
    pipe 1 1..=6 S
    farm 2 1 E
    farm 2 2 E
    belt 3 1..=4 S
    belt 3 5 W
    belt 2 5 S
    ranch 2 6 E
    belt 3..=4 6 E
end

puzzle farmyard
    pump 3 5 E
    pipe 4 3..=7 S
    farm 5 3 S
    ranch 5 4 S
    dairy 5 5 E
    farm 5 7 N
    ranch 5 6 N
end

puzzle sortingGate
    remove 8 3
    sorter 8 3 E
    set 8 3 apple
    remove 9 3
    press 9 3 E
    mill 8 4 S
    belt 8..=14 5 E
    belt 15 5..=4 N
end

puzzle cowsCostBread
    pump 9 3 E
    pipe 4..=8 3 E
    belt 7..=3 1 W
    mill 2 1 S
    bakery 2 2 S
    woodcutter 1 2 E
    farm 4 2 N
    farm 5 2 N
    farm 6 2 N
    farm 7 2 N
    farm 8 4 W
    ranch 7 4 W
    dairy 6 4 S
    belt 6..=3 5 W
    belt 2 5..=4 N
end

puzzle fanOut
    belt 0..=3 0 E
    belt 4 0..=2 S
    belt 0..=4 3 E
    splitter 5 3 E
    press 6 3 E
    press 5 2 E
    belt 6 2 E
    belt 7 2 S
    press 5 4 E
    belt 6 4 E
    belt 7 4 N
    picker 0 1 N
    picker 1 1 N
    picker 2 1 N
    picker 0 2 S
    picker 1 2 S
    picker 2 2 S
end

puzzle breakfast
    pump 1 2 E
    pipe 2..=5 2 E
    belt 3..=9 7 E
    farm 2 3 S
    farm 3 3 S
    belt 2 4 E
    belt 3 4 S
    mill 3 5 S
    bakery 3 6 S
    woodcutter 2 6 E
    farm 4 3 E
    ranch 5 3 S
    dairy 5 4 S
    belt 5 5..=6 S
    press 7 6 S
    picker 7 5 S
    picker 6 6 E
    picker 8 6 W
end

puzzle threeOnACart
    remove 6 7
    sorter 6 7 E
    set 6 7 apple
    remove 7 7
    press 7 7 E
    belt 6 8 S
    belt 6 9 E
    sorter 7 9 E
    set 7 9 wood
    splitter 8 9 E
    mill 7 10 E
    bakery 8 10 E
    belt 9 10 N
    belt 9..=16 9 E
    belt 17 9..=8 N
end

puzzle herdAndOrchard
    pump 19 3 E
    pipe 11..=18 3 E
    belt 16..=11 5 W
    mill 10 5 W
    bakery 9 5 W
    farm 16 4 S
    farm 15 4 S
    woodcutter 5 3 E
    belt 6 3 E
    belt 7 3..=6 S
    belt 7..=8 7 E
    belt 9 7..=6 N
    research orchards
    market 7 12 E
    press 6 12 E
    picker 5 12 E
    picker 5 10 E
    picker 5 11 E
    belt 6 10..=11 S
    research dairying
    farm 17 4 E
    belt 18 4..=9 S
    pipe 19 4..=10 S
    market 17 11 E
    dairy 17 10 S
    ranch 18 10 W
end

puzzle lastTrees
    remove 3 2
    belt 3 2..=6 S
    belt 3..=6 7 E
    belt 7 7 N
    woodcutter 2 6 E
    pump 11 4 E
    pipe 10 4..=5 S
    farm 9 4 W
    farm 9 5 W
    belt 8 4..=5 S
    mill 8 6 W
    bakery 7 6 N
    belt 7 5..=3 N
    belt 7 2 W
    belt 6..=5 2 W
end

puzzle salvage
    remove 13 5
    remove 12 5
    remove 10 8
    remove 14 3
    remove 12 1
    remove 13 1
    pipe 2 2 E
    pipe 2 4 E
    farm 3 2 E
    farm 3 3 E
    farm 3 4 E
    belt 4 2..=5 S
    mill 4 6 E
    bakery 5 6 E
    belt 6 6 E
    woodcutter 8 1 S
    belt 8 2..=4 S
    belt 8..=6 5 W
    belt 5 5 S
end

puzzle twoMouths
    pump 0 3 E
    pipe 1 2..=3 S
    farm 2 2 E
    farm 2 3 E
    belt 3 2..=3 S
    splitter 3 4 S
    mill 3 5 E
    bakery 4 5 S
    woodcutter 5 5 W
    belt 4 6..=7 S
    belt 4 8 E
    belt 4..=5 4 E
    belt 6 4 S
    pump 6 7 E
    pipe 6 6 E
    ranch 6 5 E
    dairy 7 5 S
    belt 7 6..=7 S
    belt 7..=6 8 W
end

puzzle fourFaults
    pipe 3 5 E
    remove 5 5
    mill 5 5 E
    remove 6 4
    woodcutter 6 4 S
    remove 10 4
    press 10 4 S
end

puzzle stonyGround
    pump 0 3 E
    pipe 1..=8 3 E
    belt 2..=8 1 E
    belt 9 1..=2 S
    belt 2..=8 5 E
    belt 9 5..=4 N
    farm 2 2 N
    farm 4 2 N
    farm 6 2 N
    farm 2 4 S
    farm 4 4 S
    farm 6 4 S
end

puzzle scaffolding
    pump 4 4 E
    pipe 5 3..=5 S
    farm 6 3 S
    farm 6 5 N
    mill 6 4 E
    bakery 7 4 E
    woodcutter 7 3 S
    wait 1200
    remove 7 3
    remove 7 4
    remove 6 4
    remove 6 3
    research orchards
    remove 6 5
    remove 4 4
    remove 5 3
    remove 5 4
    remove 5 5
    press 9 4 W
    picker 10 4 W
end

puzzle oldTrack
    pump 0 3 E
    pipe 1..=5 3 E
    farm 2 4 S
    farm 3 4 S
    farm 4 4 S
    farm 5 4 S
    remove 10 5
    mill 10 5 E
    remove 12 5
    bakery 12 5 E
    woodcutter 12 4 S
end

puzzle theRing
    pump 4 4 E
    pipe 3..=5 3 E
    pipe 3 4 E
    pipe 5 4 E
    pipe 3..=5 5 E
    belt 3..=7 1 E
    belt 7 5..=2 N
    belt 1..=7 7 E
    belt 1 3..=6 S
    farm 3 2 N
    farm 4 2 N
    farm 5 2 N
    farm 6 3 E
    farm 6 4 E
    farm 6 5 E
    farm 3 6 S
    farm 4 6 S
    farm 2 3 W
    farm 2 4 W
end

puzzle packUp
    press 6 4 S
    picker 6 3 S
    picker 5 4 E
    picker 7 4 W
    pump 4 7 E
    pipe 5 7..=8 S
    farm 6 8 N
    ranch 6 7 N
    dairy 6 6 N
    wait 2400
    remove 6 3
    remove 5 4
    remove 7 4
    remove 6 4
    remove 6 8
    remove 6 7
    remove 6 6
    remove 4 7
    remove 5 7
    remove 5 8
end

puzzle hardWinter
    market 8 6 E
    woodcutter 5 2 E
    belt 6..=7 2 E
    sawmill 8 2 S
    belt 8 3..=5 S
    pump 19 3 E
    pipe 11..=18 3 E
    belt 17..=9 5 W
    farm 17 4 S
    farm 16 4 S
    farm 15 4 S
    research milling
    remove 10 5
    mill 10 5 W
    remove 9 5
    bakery 9 5 W
    woodcutter 5 3 E
    belt 6 3 E
    belt 7 3..=6 S
    belt 7..=8 7 E
    belt 9 7..=6 N
end

puzzle firstBasket
    kitchen 10 5 E
    belt 11..=13 5 E
end

puzzle halfBuilt
    pump 19 1 E
    pipe 18 0..=1 S
    farm 17 0 S
    ranch 17 1 W
    dairy 16 1 W
    belt 15..=11 1 W
    belt 10 1..=4 S
end

puzzle picnicLicence
    press 10 7 N
    belt 10 6 N
    picker 9 7 E
    picker 11 7 W
    picker 10 8 N
    pump 12 2 E
    pipe 11 1..=2 S
    farm 10 1 S
    ranch 10 2 S
    dairy 10 3 S
    belt 10 4 S
    research picnics
end

puzzle picnicInABox
    pump 6 10 E
    pipe 6..=7 9 E
    pipe 6..=7 11 E
    farm 7 10 E
    farm 8 9 S
    farm 8 11 N
    mill 8 10 E
    bakery 9 10 E
    woodcutter 9 9 S
    press 10 11 N
    picker 9 11 E
    picker 11 11 W
    picker 10 12 N
    pump 12 8 E
    pipe 11 7..=8 S
    farm 10 7 S
    ranch 10 8 S
    dairy 10 9 S
    kitchen 10 10 E
end

puzzle villageFair
    market 12 8 E
    market 12 9 E
    pump 19 3 E
    pipe 13..=18 3 E
    belt 16..=13 5 W
    belt 12 5 S
    mill 12 6 S
    bakery 12 7 S
    farm 16 4 S
    farm 15 4 S
    farm 14 4 S
    farm 13 4 S
    woodcutter 5 4 S
    belt 5 5..=6 S
    belt 5..=11 7 E
    belt 6 11..=9 N
    belt 6..=10 8 E
    press 11 8 E
    picker 5 9 E
    picker 5 10 E
    picker 5 11 E
    farm 17 4 E
    belt 18 4..=9 S
    belt 18..=14 10 W
    pipe 19 4..=11 S
    pipe 18..=13 11 W
    belt 13 9 N
    dairy 13 8 W
    ranch 13 10 N
    wait 600
    remove 12 8
    kitchen 12 8 S
end

puzzle sortTheHamper
    remove 13 5
    sorter 13 5 E
    set 13 5 bread
    remove 15 5
    splitter 15 5 E
    belt 15 6 S
    kitchen 15 7 E
    belt 13 6 S
    sorter 13 7 S
    set 13 7 cheese
    splitter 13 8 S
    belt 14 8 E
    belt 15 8 N
    belt 14 7 E
    belt 12 7..=9 S
    belt 13 9 S
    belt 12..=21 10 E
    belt 22 10..=6 N
    belt 16..=21 7 E
end

puzzle longRoad
    pump 6 10 E
    pipe 6..=7 9 E
    pipe 6..=7 11 E
    farm 7 10 E
    farm 8 9 S
    farm 8 11 N
    mill 8 10 E
    bakery 9 10 E
    woodcutter 9 9 S
    market 10 10 E
    research orchards
    press 10 11 N
    picker 9 11 E
    picker 11 11 W
    picker 10 12 N
    research dairying
    pump 12 8 E
    pipe 11 7..=8 S
    farm 10 7 S
    ranch 10 8 S
    dairy 10 9 S
    research picnics
    wait 900
    remove 10 10
    kitchen 10 10 E
end

puzzle soldOut
    remove 8 5
    splitter 8 5 E
    belt 8 4 N
    market 8 3 E
    remove 10 6
    splitter 10 6 N
    market 11 6 E
end

puzzle thinHarvest
    pump 3 6 E
    pipe 4 6 E
    farm 4 5 E
    splitter 5 5 E
    mill 6 5 E
    bakery 7 5 E
    woodcutter 7 4 S
    belt 5 4 N
    belt 5..=7 3 E
    pump 10 3 E
    pipe 9 3 E
    ranch 8 3 S
    dairy 8 4 S
    press 8 6 N
    picker 7 6 E
    picker 9 6 W
    picker 8 7 N
    kitchen 8 5 E
end

puzzle grandPicnic
    pump 6 10 E
    pipe 6..=7 9 E
    pipe 6..=7 11 E
    farm 7 10 E
    farm 8 9 S
    farm 8 11 N
    mill 8 10 E
    bakery 9 10 E
    woodcutter 9 9 S
    press 10 11 N
    picker 9 11 E
    picker 11 11 W
    picker 10 12 N
    pump 12 8 E
    pipe 11 7..=8 S
    farm 10 7 S
    ranch 10 8 S
    dairy 10 9 S
    kitchen 10 10 E
    pump 14 10 E
    pipe 14..=15 9 E
    pipe 14..=15 11 E
    farm 15 10 E
    farm 16 9 S
    farm 16 11 N
    mill 16 10 E
    bakery 17 10 E
    woodcutter 17 9 S
    press 18 11 N
    picker 17 11 E
    picker 19 11 W
    picker 18 12 N
    pump 20 8 E
    pipe 19 7..=8 S
    farm 18 7 S
    ranch 18 8 S
    dairy 18 9 S
    kitchen 18 10 E
end
