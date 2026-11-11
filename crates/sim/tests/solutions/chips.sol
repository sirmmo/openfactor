# Solutions for the chips pack's puzzles: see ../puzzles.rs.

# Chapter "Sand and sun"

puzzle firstMelt
    dredge 2 2 E
    smelter 3 2 E
    dock 4 2 E
end

puzzle farDock
    dredge 1 1 E
    smelter 2 1 E
    belt 3..=15 1 E
    belt 16 1..=5 S
end

puzzle sunlight
    dredge 1 2 E
    smelter 2 2 E
    slicer 3 2 E
    dock 4 2 E
    pole 3 3 E
    panel 2 3 E
end

puzzle nightShift
    remove 3 2
    slicer 3 2 E
    belt 7 2 E
    pole 3 4 E
end

puzzle longCable
    dredge 1 2 E
    smelter 2 2 E
    slicer 3 2 E
    dock 4 2 E
    pole 3 3 E
    pole 8 3 E
    pole 13 3 E
    pole 18 3 E
end

puzzle thinDunes
    smelter 4 0 N
    dock 4 -1 N
    dredge 0 0 E
    belt 1..=3 0 E
    dredge 8 0 W
    belt 7..=5 0 W
    dredge 4 6 N
    belt 4 5..=1 N
end

puzzle noBelts
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    tunnel 3 0 E
    tunnel 7 0 E
    tunnel 11 0 E
    pole 2 1 E
    panel 1 1 E
end

puzzle keepSome
    dredge 1 2 E
    smelter 2 2 E
    slicer 3 2 E
    dock 4 2 E
    pole 3 1 E
    panel 2 1 E
    dredge 1 3 E
    smelter 2 3 E
    dock 3 3 E
end

puzzle oneSun
    dredge 1 0 E
    smelter 2 0 E
    dredge 1 1 E
    smelter 2 1 E
    dredge 1 2 E
    smelter 2 2 E
    dock 4 1 E
    belt 4 0 S
    belt 4 2 N
    slicer 3 1 E
    slicer 3 0 E
    slicer 3 2 E
    pole 5 1 E
    panel 6 1 E
end

puzzle fromSand
    dredge 1 2 E
    smelter 2 2 E
    belt 3 2 E
    dock 4 2 E
    pole 3 3 E
    panel 2 3 E
    wait 300
    remove 3 2
    slicer 3 2 E
end

# Chapter "Copper country"

puzzle redGround
    drill 2 2 E
    smelter 3 2 E
    dock 4 2 E
end

puzzle spools
    drill 1 1 E
    smelter 2 1 E
    drawer 3 1 E
    dock 4 1 E
end

puzzle halfAndHalf
    drill 1 2 E
    smelter 2 2 E
    splitter 3 2 E
    drawer 4 2 E
    dock 5 2 E
    belt 3..=4 1 E
    belt 5 1 S
end

puzzle fence
    drill 1 4 E
    smelter 2 4 E
    drawer 3 4 E
    belt 4 4 E
    tunnel 5 4 E
    tunnel 7 4 E
    belt 8..=11 4 E
end

puzzle oneRoad
    drill 0 0 E
    smelter 1 0 E
    belt 2 0..=7 S
    belt 2..=15 8 E
    drill 0 1 E
    smelter 1 1 E
    drill 0 2 E
    smelter 1 2 E
    drill 0 3 E
    smelter 1 3 E
end

puzzle runningDry
    smelter 1 1 E
    drawer 2 1 E
    dock 3 1 E
    drill 1 0 S
    wait 900
    remove 1 0
    drill 0 1 E
    wait 900
    remove 0 1
    drill 1 2 N
end

puzzle sortItOut
    sorter 6 2 E
    set 6 2 sand
    smelter 7 2 E
    dock 8 2 E
    smelter 6 3 S
    drawer 6 4 S
    dock 6 5 S
end

puzzle hoard
    drill 0 1 E
    smelter 1 1 E
    dock 2 1 E
    drill 0 0 E
    smelter 1 0 E
    belt 2 0 S
    drill 0 2 E
    smelter 1 2 E
    belt 2 2 N
end

puzzle oneDock
    dredge 0 0 E
    smelter 1 0 E
    belt 2..=7 0 E
    belt 8 0..=2 S
    drill 0 6 E
    smelter 1 6 E
    belt 2..=7 6 E
    belt 8 6..=4 N
    drill 0 3 E
    smelter 1 3 E
    drawer 2 3 E
    belt 3..=7 3 E
end

puzzle copperBoom
    drill 0 0 E
    smelter 1 0 E
    dock 2 0 E
    drill 0 1 E
    smelter 1 1 E
    drawer 2 1 N
    drill 0 2 E
    smelter 1 2 E
    drawer 2 2 E
    dock 3 2 E
end

# Chapter "Etch room"

puzzle firstEtch
    etcher 3 0 E
    pole 2 1 E
end

puzzle longWire
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    dock 4 0 E
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    drill 20 10 W
    smelter 19 10 W
    drawer 18 10 W
    belt 17..=4 10 W
    belt 3 10..=1 N
end

puzzle threeEtchers
    dredge 0 1 E
    smelter 1 1 E
    slicer 2 1 E
    belt 3..=4 1 E
    splitter 5 1 E
    belt 6 1 E
    splitter 7 1 E
    belt 8 1 E
    belt 9 1 S
    drill 0 3 E
    smelter 1 3 E
    drawer 2 3 E
    belt 3..=4 3 E
    splitter 5 3 E
    belt 6 3 E
    splitter 7 3 E
    belt 8 3 E
    belt 9 3 N
    etcher 5 2 E
    dock 6 2 E
    etcher 7 2 W
    etcher 9 2 E
    dock 10 2 E
    pole 4 2 E
    panel 3 2 E
    panel 2 2 E
    panel 1 2 E
end

puzzle everyGrain
    drill 3 3 N
    smelter 3 2 N
    drawer 3 1 N
    etcher 3 0 E
    dock 4 0 E
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    slicer 2 0 E
    smelter 1 0 E
    dredge 0 0 E
end

puzzle blackout
    dredge 0 5 E
    smelter 1 5 E
    dock 2 5 E
    panel 3 2 E
    panel 2 2 E
end

puzzle stockroom
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    splitter 3 0 E
    etcher 4 0 E
    dock 5 0 E
    dock 3 1 E
    drill 4 4 N
    smelter 4 3 N
    drawer 4 2 N
    splitter 4 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
end

puzzle sharedSun
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    dock 4 0 E
    drill 3 3 N
    smelter 3 2 N
    drawer 3 1 N
    pole 2 1 E
    pole 7 1 E
    pole 12 1 E
    pole 17 1 E
    pole 22 1 E
    pole 26 1 E
    dredge 24 0 E
    smelter 25 0 E
    slicer 26 0 E
    etcher 27 0 E
    dock 28 0 E
    drill 27 3 N
    smelter 27 2 N
    drawer 27 1 N
end

puzzle jammed
    drill 4 5 N
    smelter 4 4 N
    drawer 4 3 N
    remove 4 1
    belt 4 1 E
    belt 5 1 N
end

puzzle mixedDelivery
    sorter 7 2 E
    set 7 2 wafer
    splitter 8 2 E
    dock 8 1 E
    etcher 9 2 E
    dock 10 2 E
    belt 7 3 S
    belt 7..=8 4 E
    splitter 9 4 N
    belt 9 3 N
    dock 10 4 E
    pole 10 3 E
    panel 11 3 E
end

puzzle bootstrap
    drill 4 3 N
    smelter 4 2 N
    belt 4 1 N
    dock 4 0 E
    drill 3 3 N
    smelter 3 2 N
    drawer 3 1 E
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    etcher 3 0 E
    remove 3 1
    drawer 3 1 N
end

# Chapter "Packaging"

puzzle firstChip
    packager 4 0 E
    dock 5 0 E
end

puzzle packagerFee
    wait 900
    remove 4 0
    packager 4 0 E
    dock 5 0 E
end

puzzle fullLine
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    packager 4 0 E
    dock 5 0 E
    drill 3 4 N
    smelter 3 3 N
    drawer 3 2 N
    splitter 3 1 N
    belt 4 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
end

puzzle fullSpeed
    dredge 4 7 N
    smelter 4 6 N
    slicer 4 5 N
    belt 4 4 N
    splitter 4 3 E
    etcher 4 2 E
    etcher 5 3 N
    packager 5 2 E
    dock 6 2 E
    drill 0 1 E
    smelter 1 1 E
    drawer 2 1 E
    belt 3 1 E
    splitter 4 1 E
    splitter 5 1 E
    belt 6 1 E
    belt 7 1..=2 S
    belt 7 3 W
    belt 6 3 W
    pole 3 3 E
    panel 2 3 E
    panel 3 4 E
    panel 2 4 E
end

puzzle spareParts
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    splitter 3 0 E
    etcher 4 0 E
    packager 5 0 E
    dock 6 0 E
    dock 3 -1 E
    dock 3 1 E
    drill 4 4 N
    smelter 4 3 N
    drawer 4 2 N
    splitter 4 1 N
    belt 5 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
end

puzzle lastHeap
    drill 3 4 N
    smelter 3 3 N
    drawer 3 2 N
    splitter 3 1 N
    belt 4 1 N
    etcher 3 0 E
    dock 4 0 E
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    slicer 2 0 E
    smelter 1 0 E
    dredge 0 0 E
    wait 600
    remove 4 0
    packager 4 0 E
    dock 5 0 E
end

puzzle downTheLine
    remove 3 1
    splitter 3 1 N
    belt 4..=18 1 E
    belt 19 1 N
    remove 19 0
    packager 19 0 E
    dock 20 0 E
    pole 18 2 E
    panel 17 2 E
end

puzzle fastLane
    drill 12 3 W
    smelter 11 3 W
    drawer 10 3 W
    splitter 9 3 W
    dock 9 4 W
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 6 0 E
    packager 7 0 E
    dock 8 0 E
    pole 4 1 E
    panel 3 1 E
    panel 4 2 E
    fast 3..=5 0 E
    fast 8..=7 3 W
    fast 6 3..=1 N
    fast 9..=8 2 W
    fast 7 2..=1 N
end

puzzle clearance
    remove 8 6
    remove 8 5
    remove 10 6
    remove 10 3
    remove 11 3
    remove 8 3
    remove 9 3
    remove 12 0
    remove 12 1
    remove 12 2
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    packager 4 0 E
    dock 5 0 E
    drill 3 4 N
    smelter 3 3 N
    drawer 3 2 N
    splitter 3 1 N
    belt 4 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
end

puzzle chipShop
    drill 5 4 N
    smelter 5 3 N
    belt 5 2..=1 N
    dock 5 0 E
    drill 3 4 N
    smelter 3 3 N
    drawer 3 2 N
    splitter 3 1 N
    belt 4 1 N
    belt 4 0 E
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    etcher 3 0 E
    wait 600
    remove 4 0
    packager 4 0 E
end

# Chapter "Board room"

puzzle firstBoard
    boarder 5 0 E
    dock 6 0 E
    drill 5 2 N
    smelter 5 1 N
end

puzzle chipsOrBoards
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    packager 4 0 E
    splitter 5 0 E
    dock 5 -1 E
    drill 3 4 N
    smelter 3 3 N
    drawer 3 2 N
    splitter 3 1 N
    belt 4 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    dock 7 0 E
    drill 6 2 N
    smelter 6 1 N
    boarder 6 0 E
end

puzzle oneSeam
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    packager 4 0 E
    boarder 5 0 E
    dock 6 0 E
    drill 4 4 N
    smelter 4 3 N
    splitter 4 2 N
    drawer 4 1 N
    drawer 3 2 N
    belt 3 1 N
    belt 5 2..=1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
end

puzzle mixUp
    remove 4 0
    packager 4 0 E
    remove 6 1
    remove 5 1
    smelter 5 1 N
    drill 5 2 N
end

puzzle handsOff
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    packager 4 0 E
    boarder 5 0 E
    dock 6 0 E
    drill 3 3 N
    smelter 3 2 N
    drawer 3 1 N
    drill 4 3 N
    smelter 4 2 N
    drawer 4 1 N
    drill 5 3 N
    smelter 5 2 N
    belt 5 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
end

puzzle underOnePole
    slicer 8 4 E
    etcher 9 4 E
    packager 10 4 E
    boarder 11 4 E
    dock 12 4 E
    panel 9 5 E
    panel 10 6 E
    dredge 0 4 E
    smelter 1 4 E
    belt 2..=7 4 E
    drill 0 1 E
    smelter 1 1 E
    drawer 2 1 E
    belt 3..=8 1 E
    belt 9 1..=2 S
    splitter 9 3 S
    belt 10 3 S
    drill 11 8 N
    smelter 11 7 N
    belt 11 6..=5 N
end

puzzle throughTraffic
    drill 11 0 W
    smelter 10 0 W
    belt 9 0 W
    tunnel 8 0 W
    tunnel 6 0 W
    boarder 5 0 N
    dock 5 -1 N
end

puzzle twinLines
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    etcher 3 0 E
    packager 4 0 E
    drill 3 4 N
    smelter 3 3 N
    drawer 3 2 N
    splitter 3 1 N
    belt 4 1 N
    pole 2 1 E
    panel 1 1 E
    panel 2 2 E
    boarder 5 0 E
    dock 6 0 E
    drill 5 2 N
    smelter 5 1 N
    dredge 5 -5 S
    smelter 5 -4 S
    slicer 5 -3 S
    etcher 5 -2 S
    packager 5 -1 S
    drill 9 -1 W
    smelter 8 -1 W
    drawer 7 -1 W
    splitter 6 -1 W
    belt 6 -2 W
    pole 4 -2 E
    panel 3 -2 E
    panel 4 -3 E
end

puzzle sampleCase
    dredge 0 0 E
    smelter 1 0 E
    slicer 2 0 E
    splitter 3 0 E
    etcher 4 0 E
    splitter 5 0 E
    packager 6 0 E
    splitter 7 0 E
    boarder 8 0 E
    dock 9 0 E
    dock 3 -1 E
    dock 5 -1 E
    dock 7 -1 E
    dock 3 1 E
    drill 4 4 N
    smelter 4 3 N
    drawer 4 2 N
    splitter 4 1 N
    belt 5 1 N
    drill 8 3 N
    smelter 8 2 N
    belt 8 1 N
    pole 5 -2 E
    panel 4 -2 E
    panel 6 -2 E
end

puzzle grandFab
    dredge 4 7 N
    smelter 4 6 N
    slicer 4 5 N
    belt 4 4 N
    splitter 4 3 E
    etcher 4 2 E
    etcher 5 3 N
    packager 5 2 E
    boarder 6 2 E
    dock 7 2 E
    drill 0 1 E
    smelter 1 1 E
    drawer 2 1 E
    belt 3 1 E
    splitter 4 1 E
    belt 5 1 S
    drill 5 7 N
    smelter 5 6 N
    drawer 5 5 N
    belt 5 4 N
    drill 6 -1 S
    smelter 6 0 S
    belt 6 1 S
    pole 3 3 E
    panel 2 3 E
    panel 3 4 E
    panel 2 4 E
end
