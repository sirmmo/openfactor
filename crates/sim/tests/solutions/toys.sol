# Solutions for the toy workshop's puzzles: see ../puzzles.rs.

puzzle brickRush
    shop 5 10 E
    belt 5 9 S
    hopper 3 9 E
    mould 4 9 E
    belt 5 8 S
    hopper 3 8 E
    mould 4 8 E
    belt 5 7 S
    hopper 3 7 E
    mould 4 7 E
    belt 5 6 S
    hopper 3 6 E
    mould 4 6 E
    belt 5 5 S
    hopper 3 5 E
    mould 4 5 E
    belt 5 4 S
    hopper 3 4 E
    mould 4 4 E
    belt 5 3 S
    hopper 3 3 E
    mould 4 3 E
end

puzzle pitStop
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
end

puzzle giftWrap
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
    hopper 3 8 E
    mould 4 8 E
    boxer 5 8 E
    shop 6 8 E
    hopper 3 9 E
    mould 4 9 E
    belt 5 9 N
end

puzzle handful
    hopper 3 2 E
    mould 4 2 E
    shop 5 2 E
end

puzzle acrossTheRoom
    hopper 3 7 E
    mould 4 7 E
    belt 5..=13 7 E
end

puzzle wheelPress
    hopper 3 2 E
    press 4 2 E
    shop 5 2 E
end

puzzle notOneToSpare
    hopper 3 4 E
    mould 4 4 E
end

puzzle oneHopper
    hopper 2 4 E
    splitter 3 4 E
    mould 4 4 E
    shop 5 4 E
    press 3 5 E
    belt 4 5 E
    belt 5 5 N
end

puzzle twoCorners
    hopper 0 0 E
    mould 1 0 E
    belt 2..=5 0 E
    belt 6 0..=5 S
    hopper 0 12 E
    mould 1 12 E
    belt 2..=5 12 E
    belt 6 12..=7 N
end

puzzle savings
    hopper 3 2 E
    mould 4 2 E
    shop 5 2 E
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 4 E
    mould 4 4 E
    belt 5 4 N
end

puzzle wheelWorks
    shop 5 4 E
    hopper 3 4 E
    press 4 4 E
    hopper 3 3 E
    press 4 3 E
    belt 5 3 S
    hopper 3 5 E
    press 4 5 E
    belt 5 5 N
    hopper 3 2 E
    press 4 2 E
    belt 5 2 S
    hopper 3 6 E
    press 4 6 E
    belt 5 6 N
end

puzzle mixedBag
    shop 5 4 E
    hopper 3 4 E
    mould 4 4 E
    hopper 3 3 E
    press 4 3 E
    belt 5 3 S
    hopper 3 5 E
    mould 4 5 E
    belt 5 5 N
end

puzzle rushHour
    shop 3 3 E
    belt 3 -1..=2 S
    hopper 1 0 E
    mould 2 0 E
    hopper 1 1 E
    mould 2 1 E
    hopper 0 0 N
    mould 0 -1 E
    belt 1..=2 -1 E
    hopper 0 1 S
    mould 0 2 E
    belt 1..=2 2 E
end

puzzle paintShop
    hopper 3 2 E
    shaper 4 2 S
    shop 4 3 E
    tap 14 2 W
    belt 13..=5 2 W
end

puzzle fourWheels
    hopper 3 6 E
    press 4 6 E
    belt 5..=7 6 E
    belt 8 6 N
end

puzzle farPaint
    hopper 3 9 E
    shaper 4 9 E
    assembler 5 9 E
    shop 6 9 E
    hopper 3 8 E
    belt 4 8 E
    press 5 8 S
    tap 24 12 W
    belt 23..=5 12 W
    belt 4 12..=10 N
end

puzzle pitCrew
    shop 5 4 E
    hopper 3 4 E
    mould 4 4 E
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 S
    hopper 3 5 E
    mould 4 5 E
    belt 5 5 N
    research racing
    hopper 3 7 E
    shaper 4 7 E
    assembler 5 7 E
    shop 6 7 E
    hopper 3 8 E
    belt 4 8 E
    press 5 8 N
    tap 14 2 S
    belt 14 3..=5 S
    belt 14..=5 6 W
    belt 4 6 S
end

puzzle oneDrop
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    hopper 3 8 E
    shaper 4 8 E
    assembler 5 8 E
    shop 6 8 E
    hopper 3 7 E
    belt 4 7 E
    press 5 7 S
    tap 8 0 S
    belt 8 1..=3 S
    splitter 8 4 S
    belt 7..=5 4 W
    belt 4 4 S
    belt 8 5..=9 S
    belt 8..=5 10 W
    belt 4 10..=9 N
end

puzzle everyPellet
    hopper 2 4 E
    splitter 3 4 E
    shaper 4 4 N
    press 3 3 E
    assembler 4 3 E
    shop 5 3 E
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
end

puzzle spareParts
    hopper 3 5 E
    shaper 4 5 E
    splitter 5 5 E
    assembler 6 5 E
    shop 7 5 E
    shop 5 4 E
    hopper 3 7 E
    belt 4..=5 7 E
    belt 6 7 N
    press 6 6 N
    hopper 3 8 E
    press 4 8 E
    shop 5 8 E
    tap 14 2 W
    belt 13..=5 2 W
    belt 4 2..=4 S
end

puzzle backToFront
    remove 4 5
    shaper 4 5 E
    remove 5 6
    press 5 6 N
    remove 14 2
    tap 14 2 S
end

puzzle showroom
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
end

puzzle grandPrix
    hopper 1 1 E
    shaper 2 1 E
    assembler 3 1 E
    shop 4 1 E
    hopper 1 2 E
    belt 2 2 E
    press 3 2 N
    tap 10 6 N
    belt 10 5..=1 N
    belt 10..=3 0 W
    belt 2 0 S
    hopper 1 13 E
    shaper 2 13 E
    assembler 3 13 E
    shop 4 13 E
    hopper 1 12 E
    belt 2 12 E
    press 3 12 S
    tap 10 7 S
    belt 10 8..=13 S
    belt 10..=3 14 W
    belt 2 14 N
end

puzzle firstBox
    hopper 3 2 E
    mould 4 2 E
    boxer 5 2 E
    shop 6 2 E
end

puzzle pocketMoney
    hopper 3 2 E
    mould 4 2 E
    shop 5 2 E
    hopper 3 4 E
    mould 4 4 E
    boxer 5 4 E
    belt 6 4..=3 N
    belt 6 2 W
end

puzzle boxingDay
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    research boxing
    hopper 3 5 E
    mould 4 5 E
    boxer 5 5 E
    shop 6 5 E
    hopper 3 6 E
    mould 4 6 E
    belt 5 6 N
end

puzzle threeSides
    boxer 2 2 E
    shop 3 2 E
    hopper 0 2 E
    mould 1 2 E
    hopper 2 0 S
    mould 2 1 S
    hopper 2 4 N
    mould 2 3 N
end

puzzle halfAndHalf
    hopper 2 4 E
    mould 3 4 E
    splitter 4 4 E
    boxer 5 4 N
    shop 5 3 E
    belt 4 3 E
end

puzzle tradeIn
    remove 5 5
    remove 4 5
    mould 4 5 E
    boxer 5 5 E
    remove 5 6
    remove 4 6
    mould 4 6 E
    belt 5 6 N
    hopper 3 4 E
    mould 4 4 E
    belt 5 4 S
end

puzzle fixedHoppers
    boxer 7 4 E
    mould 4 4 E
    belt 5..=6 4 E
    mould 2 8 E
    belt 3..=6 8 E
    belt 7 8..=5 N
    mould 2 0 N
    belt 2..=6 -1 E
    belt 7 -1..=3 S
    mould 0 4 N
    belt 0 3..=0 N
    belt 0..=1 -1 E
end

puzzle giftAndSavings
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 6 E
    mould 4 6 E
    boxer 5 6 E
    shop 6 6 E
    hopper 3 5 E
    mould 4 5 E
    belt 5 5 S
    hopper 3 7 E
    mould 4 7 E
    belt 5 7 N
end

puzzle nightShift
    shop 5 1 E
    hopper 3 1 E
    mould 4 1 E
    hopper 3 0 E
    mould 4 0 E
    belt 5 0 S
    hopper 3 2 E
    mould 4 2 E
    belt 5 2 N
    research boxing
    hopper 3 4 E
    mould 4 4 E
    boxer 5 4 E
    shop 6 4 E
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 S
    hopper 3 5 E
    mould 4 5 E
    belt 5 5 N
    hopper 3 7 E
    mould 4 7 E
    boxer 5 7 E
    shop 6 7 E
    hopper 3 6 E
    mould 4 6 E
    belt 5 6 S
    hopper 3 8 E
    mould 4 8 E
    belt 5 8 N
    remove 5 1
    boxer 5 1 E
    shop 6 1 E
end

puzzle christmasRush
    shop 3 3 E
    belt 3 -1..=2 S
    hopper 1 0 E
    mould 2 0 E
    hopper 1 1 E
    mould 2 1 E
    hopper 0 0 N
    mould 0 -1 E
    belt 1..=2 -1 E
    hopper 0 1 S
    mould 0 2 E
    belt 1..=2 2 E
    belt 3 9..=12 S
    boxer 3 13 S
    shop 3 14 S
    hopper 1 10 E
    mould 2 10 E
    hopper 1 11 E
    mould 2 11 E
    hopper 0 10 N
    mould 0 9 E
    belt 1..=2 9 E
    hopper 0 11 S
    mould 0 12 E
    belt 1..=2 12 E
    remove 3 3
    boxer 3 3 S
    shop 3 4 S
end

puzzle underpass
    hopper 3 2 S
    press 3 3 S
    tunnel 3 4 S
    tunnel 3 6 S
    belt 3 7..=8 S
end

puzzle sortItOut
    sorter 11 5 E
    set 11 5 brick
    boxer 12 5 E
    shop 13 5 E
    shop 11 6 E
end

puzzle paperwork
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
    research logistics
end

puzzle mixedDelivery
    sorter 5 2 W
    set 5 2 paint
    splitter 4 2 W
    shop 3 2 W
    shaper 4 3 W
    belt 5 3 W
    belt 3 3 N
    mould 5 1 W
    belt 4 1 W
    belt 3 1 S
end

puzzle sixLanes
    hopper 3 0 S
    press 3 1 S
    tunnel 3 2 S
    tunnel 3 6 S
    tunnel 3 10 S
end

puzzle keyhole
    tap 14 2 W
    belt 13..=5 2 W
    tunnel 4 2 S
    tunnel 4 4 S
end

puzzle museumPiece
    research logistics
    hopper 0 0 E
    mould 1 0 E
    tunnel 2 0 E
    tunnel 6 0 E
end

puzzle fastLane
    hopper 0 0 E
    press 1 0 E
    hopper 0 1 E
    press 1 1 E
    hopper 0 2 E
    press 1 2 E
    hopper 0 3 E
    press 1 3 E
    hopper 0 4 E
    press 1 4 E
    hopper 0 5 E
    press 1 5 E
    fast 2 0..=6 S
    fast 2..=29 7 E
end

puzzle twinLines
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=12 4 W
    tunnel 11 4 W
    tunnel 7 4 W
    belt 6..=5 4 W
    belt 4 4 S
end

puzzle clearingHouse
    sorter 10 6 S
    set 10 6 brick
    boxer 10 7 S
    shop 10 8 S
    splitter 11 6 E
    assembler 12 6 E
    shop 13 6 E
    shop 11 5 E
end

puzzle everyPatent
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 0 E
    mould 4 0 E
    belt 5 0 S
    research racing
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
    research logistics
    research boxing
end

puzzle toyFair
    hopper 2 1 E
    shaper 3 1 E
    assembler 4 1 E
    shop 5 1 E
    hopper 2 2 E
    belt 3 2 E
    press 4 2 N
    tap 14 0 W
    belt 13..=4 0 W
    belt 3 0 S
    hopper 0 0 W
    mould -1 0 W
    hopper 0 1 W
    mould -1 1 W
    hopper 0 2 W
    mould -1 2 W
    belt -2 0..=2 S
    splitter -2 3 S
    boxer -2 4 S
    shop -2 5 S
    shop -1 3 E
    hopper 2 0 N
    press 2 -1 N
    shop 2 -2 N
end

puzzle beltFree
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 4 6 E
    press 5 6 N
    tap 4 -9 S
    tunnel 4 -8 S
    tunnel 4 -4 S
    tunnel 4 0 S
    tunnel 4 4 S
end

puzzle twoTiles
    hopper 2 4 E
    shaper 3 4 E
    assembler 4 4 E
    shop 5 4 E
    hopper 2 5 E
    splitter 3 5 E
    press 4 5 N
    tap 10 4 N
    belt 10..=4 3 W
    belt 3 3 S
    mould 3 6 E
    boxer 4 6 E
    shop 5 6 E
end

puzzle mondayMorning
    remove 5 5
    assembler 5 5 E
    belt 10 4 W
    remove 5 9
    sorter 5 9 N
    set 5 9 brick
    shop 6 9 E
end

puzzle islands
    hopper 0 0 S
    belt 0 1..=4 S
    belt 0..=5 5 E
    tap 6 6 N
    shaper 6 5 N
    assembler 6 4 N
    shop 6 3 N
    hopper 12 0 S
    belt 12 1..=3 S
    belt 12..=8 4 W
    press 7 4 W
    hopper 0 12 E
    mould 1 12 E
    belt 2..=5 12 E
    boxer 6 12 S
    shop 6 13 S
    hopper 12 12 W
    mould 11 12 W
    belt 10..=7 12 W
end

puzzle renovation
    remove 4 9
    remove 4 10
    remove 5 9
    remove 5 10
    remove 5 11
    remove 4 8
    remove 5 8
    remove 3 8
    shop 5 8 E
    shaper 4 9 E
    assembler 5 9 E
    shop 6 9 E
    belt 4 10 E
    press 5 10 N
    tap -6 8 E
    belt -5..=3 8 E
    belt 4 8 S
    remove 5 8
    boxer 5 8 E
    shop 6 8 E
end

puzzle productionRun
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 0 E
    mould 4 0 E
    belt 5 0 S
    research racing
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
    hopper 3 8 E
    shaper 4 8 E
    assembler 5 8 E
    shop 6 8 E
    hopper 3 7 E
    belt 4 7 E
    press 5 7 S
    tap 16 2 S
    belt 16 3..=8 S
    belt 16..=5 9 W
    belt 4 9 N
    research boxing
    remove 5 2
    boxer 5 2 E
    shop 6 2 E
end

puzzle closingTime
    hopper 0 1 E
    shaper 1 1 E
    assembler 2 1 E
    shop 3 1 E
    hopper 0 2 E
    belt 1 2 E
    press 2 2 N
    tap 14 0 W
    belt 13..=2 0 W
    belt 1 0 S
    hopper 0 3 E
    mould 1 3 E
    boxer 2 3 E
    shop 3 3 E
    wait 2400
    remove 2 1
    remove 1 1
    remove 2 2
    remove 14 0
    remove 2 3
    remove 1 3
    remove 0 1
    remove 0 2
    remove 0 3
    remove 3 1
    remove 3 3
    remove 1 2
    remove 1 0
end

puzzle toyBox
    shop 5 2 E
    hopper 3 2 E
    mould 4 2 E
    hopper 3 1 E
    mould 4 1 E
    belt 5 1 S
    hopper 3 3 E
    mould 4 3 E
    belt 5 3 N
    hopper 3 0 E
    mould 4 0 E
    belt 5 0 S
    hopper 3 5 E
    shaper 4 5 E
    assembler 5 5 E
    shop 6 5 E
    hopper 3 6 E
    belt 4 6 E
    press 5 6 N
    tap 14 2 S
    belt 14 3 S
    belt 14..=5 4 W
    belt 4 4 S
    hopper 3 7 E
    press 4 7 E
    belt 5..=6 7 E
    shop 7 7 E
    hopper 3 8 E
    mould 4 8 E
    boxer 5 8 E
    shop 6 8 E
    hopper 3 9 E
    mould 4 9 E
    belt 5 9 N
    research logistics
end
