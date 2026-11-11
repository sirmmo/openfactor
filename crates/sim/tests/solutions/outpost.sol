# Solutions for the outpost's puzzles: see ../puzzles.rs.

puzzle dustStorm
    dock 11 1 E
    electrolyser 12 1 W
    melter 13 1 W
    drill 14 1 W
    pylon 12 3 E
    panel 12 4 E
end

puzzle groundBreak
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 6 4 E
    panel 7 4 E
end

puzzle looseWire
    pylon 8 4 E
    pylon 11 6 E
    pylon 14 7 E
end

puzzle backwards
    remove 5 2
    excavator 5 2 E
    remove 6 2
    remove 7 2
    belt 6..=7 2 E
    remove 8 2
    smelter 8 2 E
    remove 9 2
    remove 10 2
    belt 9..=10 2 E
    remove 8 8
    pylon 8 5 E
end

puzzle farPlain
    smelter 4 12 W
    excavator 22 12 W
    belt 5..=21 12 W
end

puzzle narrowPlain
    excavator 2 2 N
    excavator 3 2 N
    belt 2..=6 1 E
    splitter 7 1 E
    smelter 8 1 S
    smelter 7 2 E
    dock 8 2 E
    pylon 9 3 E
    panel 10 3 E
    panel 9 4 E
end

puzzle thinSeam
    belt 3 0..=5 S
    smelter 3 6 S
    dock 3 7 S
    pylon 5 6 E
    panel 5 7 E
    excavator 2 5 E
    excavator 2 4 E
    wait 600
    remove 2 5
    remove 2 4
    excavator 2 3 E
    excavator 2 2 E
    excavator 2 1 E
    excavator 2 0 E
end

puzzle nestEgg
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 8 3 E
    panel 8 4 E
    excavator 5 1 E
    belt 6 1 S
    excavator 5 3 E
    smelter 6 3 E
    belt 7 3 N
    excavator 5 4 E
    belt 6 4 N
end

puzzle blueprints
    excavator 5 1 E
    belt 6 1 S
    research fabrication
end

puzzle twoOrders
    excavator 5 3 E
    belt 6 3 N
    excavator 5 0 E
    excavator 5 1 E
    belt 6..=7 0 E
    belt 6 1 N
    smelter 8 0 E
    workshop 9 0 E
    belt 10 0..=1 S
end

puzzle foundry
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 8 3 E
    panel 8 4 E
    excavator 5 1 E
    belt 6 1 S
    excavator 5 3 E
    smelter 6 3 E
    belt 7 3 N
    excavator 5 4 E
    belt 6 4 N
    research fabrication
    remove 7 3
    workshop 7 3 N
end

puzzle firstThaw
    drill 14 1 W
    melter 13 1 W
    dock 12 1 W
    pylon 13 3 E
    panel 13 4 E
end

puzzle iceFloes
    drill 17 1 S
    belt 14..=18 2 W
    melter 13 2 W
    belt 12 2 W
    pylon 13 4 E
    drill 14 0 S
    belt 14 1 S
    drill 15 4 N
    belt 15 3 N
    drill 18 4 N
    belt 18 3 N
end

puzzle onePanel
    remove 8 2
    drill 14 1 W
    melter 13 1 W
    belt 12 1 S
    belt 12 2 W
    pylon 13 4 E
    drill 14 2 W
    melter 13 2 W
end

puzzle handToHand
    dock 15 2 E
    drill 15 0 S
    melter 15 1 S
    pylon 17 3 E
    panel 18 3 E
    drill 14 1 S
    melter 14 2 E
    drill 16 1 S
    melter 16 2 W
    panel 17 4 E
end

puzzle lifePlans
    drill 14 1 W
    melter 13 1 W
    belt 12 1 S
    belt 12 2 W
    pylon 13 4 E
    excavator 5 1 E
    belt 6 1 S
    research lifeSupport
end

puzzle icePocket
    drill 14 0 W
    drill 14 1 W
    belt 13 0..=1 S
    melter 13 2 S
    belt 13..=14 3 E
    dock 15 3 E
    pylon 15 5 E
    panel 15 6 E
    drill 15 0 E
    drill 15 1 E
    belt 16 0..=1 S
    melter 16 2 S
    belt 16 3 W
end

puzzle sharedSun
    drill 14 1 W
    melter 13 1 W
    belt 12 1 S
    belt 12 2 W
    pylon 13 4 E
    excavator 5 1 E
    belt 6 1 S
    drill 14 2 W
    melter 13 2 W
end

puzzle coldRoad
    pylon 11 4 E
    melter 12 2 W
    belt 12 3..=14 N
    belt 13..=25 14 W
    drill 26 14 W
end

puzzle lastIce
    melter 12 0 S
    belt 12 1 S
    belt 12 2 W
    pylon 11 4 E
    belt 13..=16 0 W
    drill 14 1 N
    drill 15 1 N
    drill 16 1 N
    excavator 5 1 E
    belt 6 1 S
    research lifeSupport
end

puzzle springThaw
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 6 4 E
    panel 7 4 E
    excavator 5 1 E
    belt 6 1 S
    dock 15 2 E
    drill 15 0 S
    melter 15 1 S
    pylon 17 3 E
    panel 18 3 E
    drill 14 1 S
    melter 14 2 E
    drill 16 1 S
    melter 16 2 W
    panel 17 4 E
end

puzzle firstBreath
    drill 14 1 W
    melter 13 1 W
    electrolyser 12 1 W
    dock 11 1 W
    pylon 12 3 E
    panel 12 4 E
end

puzzle greenThumb
    drill 14 1 W
    melter 13 1 W
    greenhouse 12 1 S
    belt 12 2 W
    pylon 12 4 E
end

puzzle plansAndAir
    drill 14 1 W
    melter 13 1 W
    belt 12 1 S
    belt 12 2 W
    pylon 13 4 E
    excavator 5 1 E
    belt 6 1 S
    research lifeSupport
    remove 12 1
    electrolyser 12 1 S
end

puzzle iceWorld
    drill 14 1 W
    melter 13 1 W
    splitter 12 1 W
    electrolyser 11 1 S
    greenhouse 12 2 W
    dock 11 2 E
    pylon 12 4 E
    panel 13 4 E
    panel 12 5 E
end

puzzle repairCrew
    remove 14 1
    drill 14 1 W
    remove 12 1
    electrolyser 12 1 W
    remove 12 6
    pylon 12 4 E
end

puzzle glassAndSteel
    remove 10 2
    workshop 10 2 E
    drill 14 1 W
    melter 13 1 W
    pylon 12 4 E
    belt 12 2 W
    wait 600
    remove 10 2
    belt 10 2 E
    greenhouse 12 1 S
end

puzzle thinIce
    melter 12 0 S
    belt 12 1 S
    belt 12 2 W
    pylon 11 4 E
    belt 13..=15 0 W
    drill 14 1 N
    drill 15 1 N
    wait 900
    remove 12 1
    electrolyser 12 1 S
end

puzzle greenhouseRow
    drill 14 1 W
    melter 13 1 W
    splitter 12 1 W
    pylon 12 4 E
    greenhouse 11 1 S
    greenhouse 12 2 W
    greenhouse 12 0 N
    belt 12 -1 W
    belt 11 -1 W
    belt 10 -1..=1 S
end

puzzle airRibbon
    excavator 5 1 E
    belt 6 1 S
    drill 14 2 W
    melter 13 2 W
    electrolyser 12 2 W
    pylon 12 4 E
    drill 14 1 W
    melter 13 1 W
    electrolyser 12 1 W
    belt 11 1 S
    panel 13 4 E
    drill 14 0 W
    melter 13 0 W
    electrolyser 12 0 W
    belt 11 0 S
    panel 12 5 E
end

puzzle selfSufficient
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 6 4 E
    panel 7 4 E
    excavator 5 1 E
    belt 6 1 S
    drill 14 1 W
    melter 13 1 W
    splitter 12 1 W
    belt 11 1 W
    dock 10 1 E
    pylon 12 4 E
    panel 13 4 E
    research lifeSupport
    remove 11 1
    electrolyser 11 1 W
    panel 12 5 E
    greenhouse 12 2 W
    belt 11 2 W
    belt 10 2 N
end

puzzle moveInDay
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    electrolyser 15 7 S
    habitat 15 8 S
    greenhouse 16 6 S
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    pylon 13 7 E
    panel 12 7 E
    panel 13 8 E
end

puzzle halfBuilt
    pylon 10 6 E
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    habitat 15 8 S
end

puzzle jammed
    remove 16 8
    belt 16 8 S
    belt 16 9 W
    belt 14 6..=8 S
    belt 14 9 E
end

puzzle noSplitters
    drill 15 4 S
    melter 15 5 S
    electrolyser 15 6 S
    belt 15 7 S
    pylon 13 7 E
    panel 12 7 E
    habitat 15 8 S
    drill 16 4 S
    melter 16 5 S
    greenhouse 16 6 S
    belt 16 7 S
    belt 16 8 W
    panel 13 8 E
    drill 14 4 S
    melter 14 5 S
    belt 14 6..=7 S
    belt 14 8 E
end

puzzle homeMade
    excavator 5 1 E
    belt 6 1 S
    remove 10 2
    workshop 10 2 E
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    electrolyser 15 7 S
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    pylon 13 7 E
    panel 12 7 E
    panel 13 8 E
    greenhouse 16 6 S
    habitat 15 8 S
end

puzzle ration
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    habitat 15 7 S
    electrolyser 14 6 S
    belt 14 7 E
    greenhouse 16 6 S
    belt 16 7 W
    pylon 13 7 E
    panel 12 7 E
    panel 13 8 E
end

puzzle longCommute
    drill 15 4 S
    melter 15 5 S
    pylon 13 6 E
    panel 12 6 E
    belt 15 6..=17 S
    splitter 15 18 S
    electrolyser 15 19 S
    belt 15 20 S
    greenhouse 16 18 S
    belt 16 19..=20 S
    belt 16 21 W
    belt 14 18..=20 S
    belt 14 21 E
    pylon 13 18 E
    panel 12 18 E
    panel 12 19 E
end

puzzle airToSpare
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    electrolyser 15 7 S
    habitat 15 8 S
    greenhouse 16 6 S
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    pylon 13 7 E
    panel 12 7 E
    panel 13 8 E
    drill 14 1 W
    melter 13 1 W
    electrolyser 12 1 W
    dock 11 1 W
    pylon 12 3 E
end

puzzle crewRace
    excavator 5 1 E
    belt 6 1 S
    drill 14 1 W
    melter 13 1 W
    belt 12 1 S
    belt 12 2 W
    pylon 13 4 E
    research lifeSupport
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    electrolyser 15 7 S
    habitat 15 8 S
    greenhouse 16 6 S
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    pylon 13 7 E
    panel 12 7 E
    panel 13 8 E
end

puzzle colony
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 8 3 E
    panel 8 4 E
    excavator 5 1 E
    belt 6 1 S
    excavator 5 3 E
    smelter 6 3 E
    belt 7 3 N
    excavator 5 4 E
    belt 6 4 N
    drill 14 1 W
    melter 13 1 W
    dock 12 1 W
    pylon 13 3 E
    panel 12 3 E
    research lifeSupport
    research fabrication
    remove 7 3
    workshop 7 3 N
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    electrolyser 15 7 S
    habitat 15 8 S
    greenhouse 16 6 S
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    pylon 13 7 E
    panel 12 7 E
    panel 13 8 E
end

puzzle sortingRoom
    smelter 20 5 E
    melter 19 6 E
    belt 20 6 E
    belt 21 6 N
    sorter 19 5 E
    set 19 5 regolith
    excavator 0 4 E
    belt 1 4 S
    drill 0 5 E
    belt 1..=18 5 E
end

puzzle crystalPlans
    excavator 5 3 E
    belt 6 3 N
    excavator 5 0 E
    excavator 5 1 E
    belt 6..=7 0 E
    belt 6 1 N
    smelter 8 0 E
    workshop 9 0 E
    belt 10 0..=1 S
    research crystals
end

puzzle cuttingEdge
    cutter 5 13 N
    dock 5 12 E
    cutter 6 13 N
    belt 6 12 W
end

puzzle blackout
    cutter 5 13 N
    reactor 5 12 N
    pylon 7 8 E
    pylon 6 11 E
end

puzzle fullPress
    excavator 2 2 N
    excavator 3 2 N
    belt 2..=4 1 E
    smelter 5 1 E
    belt 6 1..=2 S
    workshop 6 3 E
    dock 7 3 E
    pylon 7 2 E
    panel 8 2 E
    excavator 2 3 S
    excavator 3 3 S
    belt 2..=4 4 E
    smelter 5 4 E
    belt 6 4 N
end

puzzle fuelOrFreight
    cutter 5 13 N
    splitter 5 12 N
    reactor 5 11 N
    dock 6 12 E
    pylon 7 8 E
    pylon 6 10 E
    excavator 5 1 E
    belt 6 1 S
end

puzzle fuelLine
    pylon 8 9 E
    pylon 8 14 E
    pylon 8 19 E
    pylon 8 24 E
    pylon 7 28 E
    reactor 5 29 N
    cutter 5 30 N
end

puzzle supplyRun
    excavator 5 3 E
    belt 6 3 N
    cutter 5 13 N
    dock 5 12 E
    cutter 6 13 N
    belt 6 12 W
    excavator 5 0 E
    excavator 5 1 E
    belt 6..=7 0 E
    belt 6 1 N
    smelter 8 0 E
    workshop 9 0 E
    belt 10 0..=1 S
end

puzzle darkSide
    cutter 5 13 N
    reactor 5 12 N
    pylon 6 11 E
    pylon 9 9 E
    pylon 12 8 E
    drill 15 4 S
    melter 15 5 S
    splitter 15 6 S
    electrolyser 15 7 S
    greenhouse 16 6 S
    belt 16 7 S
    belt 16 8 W
    belt 14 6..=7 S
    belt 14 8 E
    pylon 13 7 E
    habitat 15 8 S
end

puzzle longNight
    excavator 5 2 E
    smelter 6 2 E
    dock 7 2 E
    pylon 8 3 E
    panel 8 4 E
    excavator 5 1 E
    belt 6 1 S
    excavator 5 3 E
    smelter 6 3 E
    belt 7 3 N
    excavator 5 4 E
    belt 6 4 N
    excavator 5 5 E
    smelter 6 5 E
    belt 7 4..=5 N
    panel 9 3 E
    research fabrication
    remove 7 3
    workshop 7 3 N
    research crystals
    cutter 5 13 N
    dock 5 12 E
    cutter 6 13 N
    belt 6 12 W
end
