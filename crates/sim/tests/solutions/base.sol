# Solutions for the base pack's puzzles: see ../puzzles.rs.

puzzle firstFire
    drill 2 1 E
    furnace 3 1 E
    depot 4 1 E
    drill 2 4 E
    belt 3 4..=2 N
end

puzzle nightShift
    belt 9 6..=3 N
    belt 14..=19 2 E
end

puzzle gearTrain
    remove 15 2
    workshop 15 2 E
end

puzzle shoulder
    drill 0 2 E
    furnace 1 2 E
    drill 1 3 N
    depot 2 2 E
end

puzzle longHaul
    furnace 1 0 W
    drill 20 0 W
    belt 19..=2 0 W
    drill 1 20 N
    belt 1 19..=1 N
end

puzzle thinSeam
    drill 0 0 S
    belt 0..=6 1 E
    furnace 7 1 E
    depot 8 1 E
    drill 7 3 N
    belt 7 2 N
    wait 360
    remove 0 0
    drill 1 0 S
    wait 480
    remove 1 0
    drill 2 0 S
    wait 480
    remove 2 0
    drill 3 0 S
    wait 480
    remove 3 0
    drill 4 0 S
    wait 480
    remove 4 0
    drill 5 0 S
end

puzzle sevenGears
    drill 1 3 E
    belt 2 3..=2 N
    furnace 2 1 E
    workshop 3 1 E
    depot 4 1 E
    wait 600
    remove 1 3
    drill 1 1 E
    drill 1 3 E
end

puzzle copperRush
    drill 2 1 E
    furnace 3 1 E
    depot 4 1 E
    drill 2 5 E
    belt 3 5..=2 N
    research copperWorking
    drill 8 6 W
    furnace 7 6 N
    drill 2 6 E
    belt 3..=6 6 E
    workshop 7 5 N
    depot 7 4 N
end

puzzle rainyDay
    drill 1 1 E
    belt 2..=3 1 E
    furnace 4 1 N
    depot 4 0 E
    drill 8 1 W
    belt 7..=5 1 W
    drill 1 4 E
    belt 2..=3 4 E
    furnace 4 4 N
    depot 4 3 E
    drill 8 4 W
    belt 7..=5 4 W
    drill 1 7 E
    belt 2..=3 7 E
    furnace 4 7 N
    depot 4 6 E
    drill 8 7 W
    belt 7..=5 7 W
end

puzzle orderBook
    drill 1 1 S
    drill 1 3 N
    furnace 1 2 E
    depot 2 2 E
    drill 4 1 S
    drill 4 3 N
    furnace 4 2 E
    workshop 5 2 E
    depot 6 2 E
    drill 1 4 S
    drill 1 6 N
    furnace 1 5 E
    depot 2 5 E
    drill 8 4 S
    drill 8 6 N
    furnace 8 5 E
    depot 9 5 E
end

puzzle forkRoad
    drill 0 0 E
    furnace 1 0 E
    depot 2 0 E
    drill 0 4 E
    furnace 1 4 E
    depot 2 4 E
    drill 0 2 E
    splitter 1 2 E
    belt 1 1 N
    belt 1 3 S
end

puzzle crossroads
    drill 0 0 E
    belt 1..=11 0 E
    furnace 12 0 E
    drill 12 1 N
    tunnel 6 1 S
    tunnel 6 -1 S
    drill 6 -10 S
    belt 6 -9..=-2 S
    belt 6 2..=9 S
end

puzzle canyon
    belt 1..=3 0 E
    tunnel 4 0 E
    tunnel 8 0 E
    belt 9..=11 0 E
end

puzzle rubble
    remove 16 0
    sorter 16 0 E
    set 16 0 ironOre
    belt 16 -1 E
    belt 17 -1 E
    belt 18 -1 S
end

puzzle sharePlate
    drill 0 1 E
    furnace 1 1 E
    drill 1 2 N
    splitter 2 1 E
    depot 3 1 E
    workshop 2 0 E
    belt 3 0 S
end

puzzle wallIn
    drill 3 3 E
    furnace 4 3 E
    drill 4 6 N
    belt 4 5..=4 N
    belt 5..=8 3 E
    hoist 9 3 E
    hoist 9 3 E 1
    belt 10 3 E 1
    chute 11 3 E 1
    chute 11 3 E
    belt 12..=13 3 E
end

puzzle doubleDuty
    drill 0 0 S
    drill 0 2 N
    belt 0..=1 1 E
    furnace 2 1 E
    drill 2 2 N
    sorter 3 1 E
    set 3 1 copperPlate
    depot 3 0 E
    workshop 4 1 E
    depot 5 1 E
end

puzzle manifold
    drill -20 -1 S
    belt -20..=9 0 E
    splitter 10 0 E
    belt 10 1 S
    furnace 10 2 E
    depot 11 2 E
    drill -20 5 N
    belt -20..=9 4 E
    splitter 10 4 E
    belt 10 3 N
    drill -19 -1 S
    belt 11..=12 0 E
    splitter 13 0 E
    belt 13 1 S
    furnace 13 2 E
    depot 14 2 E
    belt 11..=12 4 E
    splitter 13 4 E
    belt 13 3 N
    drill -18 -1 S
    drill -19 5 N
    belt 14..=15 0 E
    belt 16 0..=1 S
    furnace 16 2 E
    depot 17 2 E
    belt 14..=15 4 E
    belt 16 4..=3 N
end

puzzle mineCart
    sorter 20 0 E
    set 20 0 ironOre
    furnace 21 0 E
    drill 21 1 N
    depot 22 0 E
    belt 20 -1..=-2 N
    furnace 20 -3 N
    drill 19 -3 E
    depot 20 -4 N
end

puzzle expressOrder
    drill 0 3 E
    belt 1 3..=2 N
    furnace 1 1 E
    drill 0 1 E
    workshop 2 1 E
    depot 3 1 E
    remove 1 3
    splitter 1 3 E
    belt 1 4 S
    furnace 1 5 E
    drill 0 5 E
    workshop 2 5 E
    depot 3 5 E
    research expressBelts
end

puzzle grandJunction
    drill 15 -1 S
    drill 15 1 N
    belt 15..=3 0 W
    splitter 2 0 W
    drill -15 0 E
    belt -14..=-3 0 E
    furnace -2 0 E
    belt -1 0 E
    drill 0 -15 S
    belt 0 -14..=-3 S
    furnace 0 -2 S
    belt 0 -1 S
    tunnel -1 -4 W
    tunnel 1 -4 W
    belt 2 -1 N
    splitter 2 -2 N
    belt 1 -2 W
    drill 0 15 N
    belt 0 14..=3 N
    furnace 0 2 N
    belt 0 1 N
    belt 2 1 S
    belt 2 2 W
    belt 1 2 W
    belt 2 -3 N
    belt 2 -4 W
    belt -2 -4..=-1 S
end

puzzle firstSpark
    drill 1 0 E
    arc 2 0 E
    depot 3 0 E
    drill 1 4 E
    belt 2 4 E
    steam 3 4 E
    pipe 4 4 E
    pipe 5 4 E
    pump 6 4 E
    pole 3 2 E
end

puzzle poleLine
    pole 5 0 E
    pole 10 0 E
    pole 15 0 E
    pole 20 0 E
    pole 22 3 E
end

puzzle blackout
    remove 5 4
    belt 5 4 E
    pipe 8 4 E
    pole 3 1 E
end

puzzle twoJobs
    drill 0 0 E
    arc 1 0 E
    depot 2 0 E
    drill 1 1 N
    drill 3 3 E
    steam 4 3 E
    pipe 5 3 E
    pump 6 3 E
    pole 3 1 E
end

puzzle sunshine
    arc 2 1 E
    depot 3 1 E
    drill 1 1 E
    drill 2 0 S
    drill 2 2 N
    pole 4 1 E
    solar 5 1 E
    solar 5 0 E
    solar 5 2 E
    solar 6 1 E
    solar 5 -1 E
    solar 5 3 E
end

puzzle kickStart
    pole 1 2 E
    solar 2 2 E
    powerDrill 0 4 E
    steam 1 4 E
    pipe 2 4 E
    pump 3 4 E
    powerDrill 0 0 E
    arc 1 0 E
    depot 2 0 E
end

puzzle threeIslands
    pole 0 5 E
    pole 0 10 E
    pole -5 10 E
    pole 5 10 E
    pole 0 15 E
end

puzzle coalForTwo
    drill 0 0 E
    furnace 1 0 E
    depot 2 0 E
    drill 1 2 N
    splitter 1 1 N
    steam 2 1 E
    pipe 2 2 E
    pump 2 3 E
    drill 4 1 E
    arc 5 1 E
    depot 6 1 E
    pole 4 2 E
end

puzzle lightsOn
    drill 1 1 S
    drill 1 3 N
    furnace 1 2 E
    depot 2 2 E
    drill 1 4 S
    drill 1 6 N
    furnace 1 5 E
    depot 2 5 E
    drill 4 4 S
    drill 4 6 N
    furnace 4 5 E
    workshop 5 5 E
    depot 6 5 E
    drill 8 4 S
    drill 8 6 N
    furnace 8 5 E
    depot 9 5 E
    research electricity
    research metallurgy
end

puzzle powerStation
    drill 6 1 W
    steam 5 1 E
    pipe 5 2 E
    pump 4 2 E
    pole 3 3 E
    powerDrill 1 0 E
    arc 2 0 E
    depot 3 0 E
    drill 6 3 W
    steam 5 3 E
    powerDrill 1 2 E
    arc 2 2 E
    depot 3 2 E
    powerDrill 1 4 E
    arc 2 4 E
    depot 3 4 E
end

puzzle firstAssembly
    drill 1 1 E
    arc 2 1 E
    assembly 3 1 E
    depot 4 1 E
    drill 5 4 E
    steam 6 4 E
    pipe 7 4 E
    pump 8 4 E
    pole 4 3 E
    drill 2 0 S
    drill 2 2 N
end

puzzle everyLump
    drill 0 0 E
    furnace 1 0 E
    workshop 2 0 E
    depot 3 0 E
    drill 0 2 E
    belt 1 2..=1 N
    drill 0 5 E
    furnace 1 5 E
    workshop 2 5 E
    depot 3 5 E
    drill 0 3 E
    splitter 1 3 E
    belt 1 4 S
end

puzzle wireDrawing
    powerDrill 0 0 E
    arc 1 0 E
    assembly 2 0 E
    depot 3 0 E
    drill 3 3 E
    steam 4 3 E
    pipe 5 3 E
    pump 6 3 E
    pole 2 2 E
end

puzzle rooftop
    drill 0 0 E
    drill 1 1 N
    furnace 1 0 E
    workshop 2 0 E
    belt 3 0 E
    belt 4 0..=1 S
    hoist 4 2 E
    hoist 4 2 E 2
    drill 0 4 E
    drill 1 5 N
    furnace 1 4 E
    workshop 2 4 E
    belt 3 4 E
    belt 4 4..=3 N
end

puzzle automationRace
    drill 1 1 S
    drill 1 3 N
    furnace 1 2 E
    workshop 2 2 E
    depot 3 2 E
    drill 4 1 S
    drill 4 3 N
    furnace 4 2 E
    workshop 5 2 E
    depot 6 2 E
    research automation
    drill 10 4 S
    steam 10 5 E
    pipe 11 5 E
    pump 11 6 E
    pole 6 5 E
    drill 2 6 N
    arc 2 5 E
    assembly 3 5 E
    depot 4 5 E
end

puzzle badAir
    remove 1 1
    steam 2 2 E
    belt 1 1 E
    belt 2 1 S
end

puzzle noBelts
    powerDrill 2 5 E
    arc 3 5 E
    assembly 4 5 E
    powerDrill 8 5 W
    arc 7 5 W
    assembly 6 5 W
    drill 5 8 N
    steam 5 7 E
    pipe 6 7 E
    pump 7 7 E
    pole 5 3 E
end

puzzle loadTest
    drill 6 1 E
    steam 7 1 E
    pipe 7 2 E
    pump 8 2 E
    pole 4 3 E
    drill 1 1 E
    arc 2 1 E
    assembly 3 1 E
    depot 4 1 E
    drill 1 6 E
    arc 2 6 E
    depot 3 6 E
    drill 2 0 S
    drill 2 2 N
    drill 2 5 S
    drill 2 7 N
end

puzzle busyWeek
    drill 7 3 E
    steam 8 3 E
    pipe 9 3 E
    pump 10 3 E
    pole 4 4 E
    drill 1 1 E
    arc 2 1 E
    assembly 3 1 E
    depot 4 1 E
    drill 1 6 E
    arc 2 6 E
    assembly 3 6 E
    depot 4 6 E
    drill 1 10 E
    furnace 2 10 E
    depot 3 10 E
    drill 2 11 N
    drill 2 0 S
    drill 2 2 N
    drill 2 5 S
    drill 2 7 N
    drill 1 12 E
    furnace 2 12 E
    depot 3 12 E
    drill 2 13 N
end

puzzle motorWorks
    drill 7 4 E
    steam 8 4 E
    pipe 8 5 E
    pump 9 5 E
    pole 5 3 E
    drill 1 1 E
    arc 2 1 E
    assembly 3 1 E
    depot 4 1 E
    drill 2 0 S
    drill 2 2 N
    drill 1 10 E
    arc 2 10 E
    assembly 3 10 E
    depot 4 10 E
    pole 5 8 E
    drill 7 6 E
    steam 8 6 E
    drill 2 9 S
    drill 2 11 N
    drill 1 5 E
    arc 2 5 E
    assembly 3 5 E
    depot 4 5 E
    drill 2 4 S
    drill 2 6 N
    research motors
end

puzzle hardRock
    powerDrill 0 0 E
    arc 1 0 E
    depot 2 0 E
    drill 1 1 N
    drill 3 3 E
    steam 4 3 E
    pipe 5 3 E
    pump 6 3 E
    pole 2 2 E
end

puzzle blackGold
    derrick 0 0 E
    pipe 1 0 E
    refinery 2 0 E
    depot 3 0 E
    depot 2 -1 E
    pipe 2 1 E
    pump 2 2 E
    drill 4 1 W
    steam 3 1 E
    pole 1 2 E
end

puzzle wasteNot
    pole 0 1 E
    solar 1 3 E
    derrick 0 0 E
    pipe 1 0 E
    refinery 2 0 E
    depot 3 0 E
    splitter 2 -1 N
    burner 2 -2 N
    depot 1 -1 E
    pipe 2 1 E
    pump 2 2 E
end

puzzle onePipe
    drill 2 2 W
    steam 1 2 E
    refinery 0 1 N
    belt -1 1 N
    belt -1 0 E
    pole 2 0 E
    pipe 1..=30 1 E
    derrick 30 0 E
    pump 30 2 E
    pole 7 0 E
    pole 12 0 E
    pole 17 0 E
    pole 22 0 E
    pole 27 0 E
end

puzzle pipeBridge
    drill 5 2 E
    furnace 6 2 E
    drill 6 3 N
    belt 7..=9 2 E
    pipe 10 1 E 1
    pipe 10 2 E 1
    pipe 10 3 E 1
    remove 10 2
    belt 10..=13 2 E
end

puzzle carbideRun
    drill 8 1 W
    steam 7 1 E
    pipe 7 2 E
    pump 6 2 E
    pole 4 2 E
    pole 1 3 E
    powerDrill 0 0 E
    belt 1 0 E
    splitter 2 0 E
    arc 3 0 E
    depot 4 0 E
    drill 3 -1 S
    arc 2 -1 N
    depot 2 -2 E
    drill 1 -1 E
    drill 8 3 W
    steam 7 3 E
    arc 2 1 S
    depot 2 2 E
    drill 1 1 E
end

puzzle leftHand
    remove 2 0
    refinery 2 0 S
    belt 2 1 E
    belt 3 1 N
end

puzzle fuelSwitch
    derrick 5 8 E
    pipe 6 8 E
    pump 6 9 E
    refinery 7 8 E
    depot 8 8 E
    burner 7 7 E
    pole 6 6 E
end

puzzle saveTheCoal
    pole 4 1 E
    solar 4 2 E
    derrick 0 0 E
    pipe 1 0 E
    pump 1 -1 E
    refinery 2 0 E
    depot 3 0 E
    burner 2 -1 E
    refinery 1 1 S
    depot 1 2 E
    burner 2 1 E
    powerDrill 6 0 E
    arc 7 0 E
    depot 8 0 E
    drill 7 1 N
end

puzzle robotics
    drill 1 0 E
    steam 2 0 E
    pipe 3 0 E
    pump 4 0 E
    pole 6 1 E
    pole 11 1 E
    derrick 15 0 E
    pipe 14 0 E
    pump 14 1 E
    refinery 13 0 W
    depot 12 0 E
    depot 13 1 E
    pole -3 0 E
    pole -8 0 E
    pole -12 1 E
    powerDrill -15 0 E
    arc -14 0 E
    depot -13 0 E
    drill -14 1 N
    research robotics
end
