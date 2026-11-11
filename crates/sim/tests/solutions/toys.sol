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
