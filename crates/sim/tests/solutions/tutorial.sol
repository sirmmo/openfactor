# Solutions for the tutorial pack's puzzles: see ../puzzles.rs.

# Groundwork

puzzle hello
    belt 3..=6 1 E
end

puzzle roundTheBend
    drill 13 3 S
    belt 13 4..=5 S
    belt 13..=9 6 W
end

puzzle pocketMoney
    depot 2 10 E
    drill 2 9 S
end

puzzle fourSides
    drill 4 3 S
    drill 3 4 E
    drill 5 4 W
    drill 4 5 N
end

puzzle threeOres
    depot 6 5 E
    drill 4 3 S
    belt 4 4 S
    belt 4..=5 5 E
    drill 4 7 N
    belt 4 6 N
    drill 12 3 S
    belt 12 4 S
    belt 12..=7 5 W
end

puzzle combSeam
    belt 4 0..=7 S
    drill 3 5 E
    drill 3 4 E
    drill 3 3 E
    drill 3 2 E
    drill 3 1 E
    drill 3 0 E
end

puzzle everyLastOre
    depot 4 4 E
    drill 3 4 E
    drill 5 4 W
    drill 4 3 S
    wait 300
    remove 3 4
    drill 4 5 N
end

puzzle longHaul
    drill 30 4 W
    belt 29..=1 4 W
end

puzzle meetInTheMiddle
    drill 1 1 E
    belt 2..=9 1 E
    belt 10 1..=5 S
    drill 20 12 W
    belt 19..=11 12 W
    belt 10 12..=7 N
end

puzzle quarry
    depot 2 4 E
    belt 2 0..=3 S
    drill 1 3 E
    drill 3 3 W
    drill 1 2 E
    drill 3 2 W
    drill 1 1 E
    drill 3 1 W
end

# Fire and coal

puzzle firstFire
    drill 2 7 N
    belt 2 6 N
end

puzzle cheekToCheek
    drill 2 2 E
    furnace 3 2 E
    drill 3 3 N
    depot 4 2 E
end

puzzle copperGlow
    drill 12 3 S
    furnace 12 4 E
    depot 13 4 E
    drill 3 7 E
    belt 4..=11 7 E
    belt 12 7..=5 N
end

puzzle payday
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    depot 3 5 E
    drill 2 7 N
    belt 2 6 N
end

puzzle shareTheCoal
    drill 5 5 N
    splitter 5 4 N
    furnace 4 4 N
    furnace 6 4 N
    drill 3 4 E
    drill 7 4 W
    depot 4 3 E
    depot 6 3 E
end

puzzle twoOresOneFire
    drill 2 5 E
    belt 3..=4 5 E
    drill 5 2 S
    belt 5 3..=4 S
    drill 5 8 N
    belt 5 7..=6 N
    depot 6 5 E
end

puzzle everyLump
    furnace 5 4 S
    depot 5 5 E
    drill 4 3 S
    belt 4 4 E
    belt 5 1..=3 S
    drill 6 1 W
    drill 6 2 W
    drill 6 3 W
end

puzzle whereToSmelt
    drill 0 6 E
    belt 1..=9 6 E
    furnace 10 6 N
    drill 20 6 W
    belt 19..=11 6 W
    belt 10 5..=1 N
end

puzzle hotStreak
    drill 0 3 S
    furnace 0 4 S
    depot 0 5 E
    drill 2 3 S
    furnace 2 4 S
    depot 2 5 E
    drill 1 7 N
    belt 1 5..=6 N
    splitter 1 4 N
    drill 4 3 S
    furnace 4 4 S
    depot 4 5 E
    drill 3 7 N
    belt 3 5..=6 N
    belt 3 4 E
end

puzzle foundry
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    depot 3 5 E
    drill 2 7 N
    belt 2 6 N
    drill 12 3 S
    furnace 12 4 E
    depot 13 4 E
    drill 3 7 E
    belt 4..=11 7 E
    belt 12 7..=5 N
end

# Workshop

puzzle firstGear
    workshop 4 5 E
end

puzzle chainGang
    drill 2 2 E
    furnace 3 2 E
    drill 3 3 N
    workshop 4 2 E
    depot 5 2 E
end

puzzle shareThePlates
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    drill 2 7 N
    belt 2 6 N
    splitter 3 5 E
    depot 4 5 E
    workshop 3 6 E
    belt 4 6 N
end

puzzle oneOfEverything
    drill 2 5 E
    belt 3..=4 5 E
    drill 5 2 S
    belt 5 3..=4 S
    drill 5 8 N
    belt 5 7..=6 N
    furnace 5 5 E
    workshop 6 5 E
    depot 7 5 E
end

puzzle hungryWorkshop
    workshop 2 5 S
    depot 2 6 E
    drill 1 3 S
    belt 1 4 S
    furnace 1 5 E
    drill 1 7 N
    belt 1 6 N
    drill 3 3 S
    belt 3 4 S
    furnace 3 5 W
    drill 3 7 N
    belt 3 6 N
end

puzzle gearsAndWire
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    workshop 3 5 E
    depot 4 5 E
    drill 2 7 N
    belt 2 6 N
    drill 12 3 S
    furnace 12 4 E
    workshop 13 4 E
    depot 14 4 E
    drill 3 7 E
    belt 4..=11 7 E
    belt 12 7..=5 N
end

puzzle savingUp
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    belt 3 5 E
    depot 4 5 E
    drill 2 7 N
    belt 2 6 N
    wait 600
    remove 3 5
    splitter 3 5 E
    workshop 3 6 E
    belt 4 6 N
end

puzzle connectTheDots
    belt 2..=6 2 E
    belt 7 2..=4 S
    belt 1..=5 8 E
    belt 6 8..=6 N
    belt 6 5 E
    belt 7..=9 6 E
    belt 10 6..=3 N
    belt 10..=13 1 E
    belt 14 1..=5 S
end

puzzle wireWorks
    drill 2 6 N
    splitter 2 5 N
    drill 1 2 S
    belt 1 3..=4 S
    furnace 1 5 W
    workshop 0 5 S
    depot 0 6 E
    drill 3 2 S
    belt 3 3..=4 S
    furnace 3 5 E
    workshop 4 5 S
    depot 4 6 E
end

puzzle bigOrder
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    depot 3 5 E
    drill 2 7 N
    belt 2 6 N
    drill 12 3 S
    furnace 12 4 E
    workshop 13 4 E
    depot 14 4 E
    drill 3 7 E
    belt 4..=11 7 E
    belt 12 7..=5 N
    drill 0 3 S
    belt 0 4 S
    furnace 0 5 S
    drill 1 7 N
    belt 1 6 N
    belt 1 5 W
    workshop 0 6 W
    depot -1 6 E
end

# Mend and finish

puzzle wrongWay
    remove 4 5
    belt 4 5 E
end

puzzle turnAround
    remove 12 3
    drill 12 3 S
    remove 12 6
    drill 12 6 N
end

puzzle potholes
    belt 4 2 E
    belt 9 5 N
    belt 11 2 E
end

puzzle scrapyard
    remove 6 1
    remove 7 1
    remove 8 1
    remove 9 1
    remove 6 8
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    drill 2 7 N
    belt 2 6 N
end

puzzle trafficJam
    remove 2 5
    remove 2 6
    remove 2 7
    drill 4 7 N
    belt 4 5..=6 N
end

puzzle dryWell
    drill 8 2 S
    belt 8 3 S
    belt 8..=3 4 W
end

puzzle leakyLine
    remove 3 5
    belt 3 5 N
end

puzzle changeOfPlan
    remove 2 2
    drill 3 3 N
    remove 4 2
    workshop 4 2 E
    depot 5 2 E
end

puzzle starvingFurnace
    remove 3 4
    splitter 3 4 N
end

puzzle overhaul
    remove 2 4
    belt 2 4 S
    remove 12 3
    drill 12 3 S
    workshop 13 4 E
    belt 12 5 N
end

# Graduation

puzzle seedMoney
    drill 2 2 E
    furnace 3 2 E
    drill 3 3 N
    depot 4 2 E
    drill 6 2 E
    furnace 7 2 E
    drill 7 3 N
    depot 8 2 E
end

puzzle rationing
    depot 4 5 E
    belt 3 5 E
    furnace 2 5 E
    drill 2 7 N
    belt 2 6 N
    belt 2 4 S
    drill 2 3 S
    wait 560
    remove 3 5
    workshop 3 5 E
end

puzzle beltStrike
    drill 2 3 N
    splitter 2 2 N
    furnace 1 2 N
    furnace 3 2 N
    drill 0 2 E
    drill 4 2 W
    workshop 1 1 N
    depot 1 0 E
    workshop 3 1 N
    depot 3 0 E
end

puzzle outpost
    drill 24 0 W
    belt 23..=3 0 W
    furnace 2 0 W
    workshop 1 0 W
    drill 1 12 E
    belt 2 12..=1 N
end

puzzle mineClosure
    furnace 3 4 S
    depot 3 5 E
    drill 2 4 E
    belt 7..=4 4 W
    drill 5 3 S
    drill 5 5 N
    wait 2000
    remove 5 3
    drill 7 3 S
    remove 5 5
    drill 7 5 N
end

puzzle stockpile
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    depot 3 5 E
    drill 2 7 N
    belt 2 6 N
    drill 4 3 S
    belt 4 4 S
    furnace 4 5 W
    drill 4 7 N
    belt 4 6 N
    drill 0 3 S
    belt 0 4 S
    furnace 0 5 S
    drill 1 7 N
    belt 1 6 N
    belt 1 5 W
    workshop 0 6 W
    depot -1 6 E
end

puzzle oneCoal
    drill 5 6 N
    splitter 5 5 N
    furnace 4 5 W
    workshop 3 5 W
    depot 2 5 E
    drill 2 2 S
    belt 2..=3 3 E
    belt 4 3..=4 S
    furnace 6 5 E
    workshop 7 5 E
    depot 8 5 E
    drill 8 2 S
    belt 8..=7 3 W
    belt 6 3..=4 S
end

puzzle cleanSlate
    remove 12 3
    remove 12 4
    remove 13 4
    remove 14 4
    remove 3 7
    remove 4 7
    remove 5 7
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    workshop 3 5 E
    depot 4 5 E
    drill 2 7 N
    belt 2 6 N
end

puzzle grandTour
    drill 2 1 E
    furnace 3 1 E
    workshop 4 1 E
    depot 5 1 E
    drill 15 15 N
    belt 15 14..=3 N
    belt 15..=4 2 W
    belt 3 2 N
    drill 30 1 W
    furnace 29 1 W
    workshop 28 1 W
    depot 27 1 E
    drill 17 15 N
    belt 17 14..=3 N
    belt 17..=28 2 E
    belt 29 2 N
end

puzzle finalExam
    drill 2 3 S
    belt 2 4 S
    furnace 2 5 E
    depot 3 5 E
    drill 2 7 N
    belt 2 6 N
    drill 4 3 S
    belt 4 4 S
    furnace 4 5 W
    drill 4 7 N
    belt 4 6 N
    drill 12 3 S
    furnace 12 4 E
    workshop 13 4 E
    depot 14 4 E
    drill 4 9 E
    belt 5..=11 9 E
    belt 12 9..=5 N
    drill 0 3 S
    belt 0 4 S
    furnace 0 5 S
    drill 1 7 N
    belt 1 6 N
    belt 1 5 W
    workshop 0 6 W
    depot -1 6 E
end
