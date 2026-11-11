# Solutions for the network pack's puzzles: see ../puzzles.rs.

# Localhost

puzzle ping
    gateway 1 4 E
    belt 2..=5 4 E
    internet 6 4 E
end

puzzle itWorks
    gateway 1 4 E
    belt 2..=4 4 E
    monolith 5 4 E
    internet 6 4 E
end

puzzle lastMile
    gateway 1 1 S
    belt 1 2..=11 S
    belt 1..=22 12 E
    monolith 23 12 E
end

puzzle switchedFabric
    switch 5 6 E
    switch 8 8 E
    switch 10 11 E
end

puzzle healthChecks
    internet 6 3 E
    gateway 0 3 E
    belt 1..=5 3 E
    gateway 0 2 E
    monolith 1 2 E
    belt 2..=5 2 E
    belt 6 2 S
    gateway 0 4 E
    monolith 1 4 E
    belt 2..=5 4 E
    belt 6 4 N
end

puzzle insideTheReach
    internet 8 9 E
    belt 8 4..=8 S
    gateway 1 6 E
    belt 2..=6 6 E
    monolith 7 6 E
    gateway 1 5 E
    belt 2..=6 5 E
    monolith 7 5 E
    gateway 1 7 E
    belt 2..=6 7 E
    monolith 7 7 E
    gateway 1 4 E
    belt 2..=6 4 E
    monolith 7 4 E
    gateway 1 8 E
    belt 2..=6 8 E
    monolith 7 8 E
end

puzzle chokePoint
    belt 4..=13 6 E
    switch 11 5 E
    switch 6 5 E
    gateway 1 6 E
    belt 2 6 E
    monolith 3 6 E
    belt 4 5 S
    gateway 1 5 E
    belt 2 5 E
    monolith 3 5 E
    belt 4 7 N
    gateway 1 7 E
    belt 2 7 E
    monolith 3 7 E
    belt 4 4 S
    gateway 1 4 E
    belt 2 4 E
    monolith 3 4 E
    belt 4 3 S
    gateway 1 3 E
    belt 2 3 E
    monolith 3 3 E
    switch 6 9 E
    belt 4 8 N
    gateway 1 8 E
    belt 2 8 E
    monolith 3 8 E
end

puzzle branchOffices
    rack 11 1 E
    switch 11 2 E
    internet 11 4 E
    belt 8..=10 4 E
    monolith 8 3 S
    belt 8 0..=2 S
    belt 1..=7 0 E
    gateway 0 0 E
    belt 14..=12 4 W
    monolith 14 3 S
    belt 14 0..=2 S
    belt 23..=15 0 W
    gateway 24 0 W
    monolith 11 5 N
    belt 11 14..=6 N
    belt 1..=10 14 E
    gateway 0 14 E
end

puzzle breakEven
    internet 3 6 E
    rack 5 3 E
    switch 4 3 E
    gateway 1 5 E
    monolith 2 5 E
    belt 3 5 S
    belt 3 0..=4 S
    gateway 1 4 E
    monolith 2 4 E
    gateway 1 3 E
    monolith 2 3 E
    gateway 1 2 E
    monolith 2 2 E
    gateway 1 1 E
    monolith 2 1 E
    gateway 1 0 E
    monolith 2 0 E
end

puzzle frontPage
    internet 2 10 E
    belt 2 0..=9 S
    rack 4 5 E
    switch 3 5 E
    gateway 0 5 E
    monolith 1 5 E
    gateway 0 4 E
    monolith 1 4 E
    gateway 0 6 E
    monolith 1 6 E
    gateway 0 3 E
    monolith 1 3 E
    gateway 0 7 E
    monolith 1 7 E
    gateway 0 2 E
    monolith 1 2 E
    gateway 0 8 E
    monolith 1 8 E
    switch 3 0 E
    gateway 0 1 E
    monolith 1 1 E
    gateway 0 0 E
    monolith 1 0 E
    switch 3 10 E
    gateway 0 9 E
    monolith 1 9 E
end

# Microservices

puzzle malformed
    logger 2 3 E
end

puzzle decomposed
    switch 6 4 E
    rack 6 5 E
    gateway 1 3 E
    parser 2 3 E
    logger 2 2 E
    router 3 3 E
    belt 4..=7 3 E
    database 8 3 E
    renderer 9 3 E
    internet 10 3 E
end

puzzle nowhereToLog
    rack 4 6 E
    switch 4 5 E
    internet 5 3 E
    belt 3..=4 3 E
    belt 2 2 E
    belt 3 2 S
    gateway 1 3 E
    parser 2 3 E
end

puzzle dataExport
    switch 6 5 E
    rack 7 5 E
    internet 10 4 E
    database 9 4 E
    belt 4..=8 4 E
    router 3 4 E
    logger 2 3 E
    parser 2 4 E
    gateway 1 4 E
    belt 9 6..=5 N
    belt 4..=8 6 E
    router 3 6 E
    logger 2 5 E
    parser 2 6 E
    gateway 1 6 E
end

puzzle outage
    remove 3 4
    router 3 4 E
    remove 5 4
    belt 5 4 E
    switch 7 6 E
end

puzzle hotAisle
    rack 6 1 E
    rack 6 9 E
end

puzzle roundTrip
    rack 22 10 E
    switch 22 9 E
    database 24 8 W
    belt 24 2..=7 S
    renderer 3 8 W
    belt 23..=4 8 W
    belt 4..=23 2 E
    router 3 2 E
    logger 2 1 E
    parser 2 2 E
    gateway 1 2 E
end

puzzle auditTrail
    internet 5 3 E
    belt 3..=4 3 E
    logger 2 2 E
    parser 2 3 E
    belt 1 3 E
    gateway 0 3 E
    belt 1 2 S
    gateway 0 2 E
    belt 1 4 N
    gateway 0 4 E
end

puzzle migration
    gateway 1 2 E
    monolith 2 2 E
    research microservices
    switch 5 6 E
    gateway 1 5 E
    parser 2 5 E
    logger 2 4 E
    router 3 5 E
    belt 4..=7 5 E
    belt 8 5 S
    database 8 6 E
    internet 9 6 E
end

puzzle production
    internet 12 12 E
    belt 12 1..=11 S
    switch 5 2 E
    switch 9 2 E
    rack 7 4 E
    gateway 1 1 E
    parser 2 1 E
    logger 2 0 E
    router 3 1 E
    belt 4..=9 1 E
    database 10 1 E
    renderer 11 1 E
    gateway 1 3 E
    parser 2 3 E
    logger 2 2 E
    router 3 3 E
    belt 4..=9 3 E
    database 10 3 E
    renderer 11 3 E
    switch 5 6 E
    switch 9 6 E
    rack 7 8 E
    gateway 1 5 E
    parser 2 5 E
    logger 2 4 E
    router 3 5 E
    belt 4..=9 5 E
    database 10 5 E
    renderer 11 5 E
    gateway 1 7 E
    parser 2 7 E
    logger 2 6 E
    router 3 7 E
    belt 4..=9 7 E
    database 10 7 E
    renderer 11 7 E
    switch 5 10 E
    switch 9 10 E
    rack 7 12 E
    gateway 1 9 E
    parser 2 9 E
    logger 2 8 E
    router 3 9 E
    belt 4..=9 9 E
    database 10 9 E
    renderer 11 9 E
    gateway 1 11 E
    parser 2 11 E
    logger 2 10 E
    router 3 11 E
    belt 4..=9 11 E
    database 10 11 E
    renderer 11 11 E
end

# Traffic control

puzzle roundRobin
    internet 5 5 E
    belt 1 5 E
    balancer 2 5 E
    monolith 3 5 E
    belt 4 5 E
    monolith 2 4 N
    belt 2..=3 3 E
    belt 4 3..=4 S
    monolith 2 6 S
    belt 2..=3 7 E
    belt 4 7..=6 N
end

puzzle crossedWires
    remove 4 4
    firewall 4 4 E
    set 4 4 json
    logger 4 5 E
end

puzzle renderFarm
    switch 8 2 E
    internet 12 4 E
    renderer 11 4 E
    balancer 10 4 E
    renderer 10 3 N
    belt 10..=11 2 E
    belt 12 2..=3 S
end

puzzle underTheWire
    rack 14 7 E
    switch 14 6 E
    switch 9 6 E
    switch 4 6 E
    internet 18 4 E
    renderer 17 4 E
    database 16 4 E
    belt 12..=15 4 E
    queue 11 4 E
    queue 9 4 E
    belt 4..=8 4 E
    router 3 4 E
    logger 2 3 E
    parser 2 4 E
    gateway 1 4 E
end

puzzle backlog
    research loadBalancing
    remove 6 4
    balancer 6 4 E
    belt 6 3 N
    belt 6..=7 2 E
    belt 8 2..=3 S
    wait 1800
    remove 6 4
    belt 6 4 E
    research messaging
end

puzzle multiplex
    rack 7 8 E
    switch 6 7 E
    internet 10 5 E
    renderer 9 5 E
    database 8 5 E
    belt 7 5 E
    router 6 5 E
    firewall 5 5 E
    set 5 5 json
    logger 5 4 E
    belt 2..=4 5 E
    belt 1 6 N
    belt 1 5 E
    parser 2 6 N
    belt 2 8..=7 N
    gateway 1 7 E
    gateway 1 8 E
end

puzzle apiAndSite
    switch 2 6 E
    switch 8 6 E
    internet 9 4 E
    renderer 8 4 E
    database 7 4 E
    belt 4..=6 4 E
    router 3 4 E
    belt 2..=8 5 E
    belt 9 5 N
    balancer 2 4 E
    logger 1 3 E
    parser 1 4 E
    gateway 0 4 E
end

puzzle serverRoom
    gateway 1 7 E
    parser 2 7 E
    logger 2 6 E
    router 3 7 E
    belt 4 7 E
    queue 5 7 E
    queue 11 7 E
    belt 12..=13 7 E
    database 14 7 S
    belt 14 8 S
    renderer 14 9 W
    belt 13..=12 9 W
    queue 11 9 W
    queue 5 9 W
    belt 4 9 W
    switch 6 10 E 1
    switch 10 9 E 1
    switch 12 8 E
end

puzzle fanInFanOut
    switch 6 8 E
    rack 6 9 E
    internet 9 5 E
    renderer 8 5 E
    database 7 5 E
    router 6 5 E
    balancer 5 5 E
    logger 4 4 E
    parser 4 5 E
    belt 0..=3 5 E
    belt 0 1..=4 S
    gateway 0 0 S
    belt 0 9..=6 N
    gateway 0 10 N
    router 5 6 S
    database 5 7 E
    renderer 6 7 E
    belt 7..=8 7 E
    belt 9 7..=6 N
    rack 3 9 E
    belt 2 11..=6 N
    gateway 2 12 N
end

puzzle peakHour
    rack 4 1 E
    switch 4 4 E
    internet 9 5 E
    belt 8 5 E
    gateway 0 5 E
    belt 1 5 E
    balancer 2 5 E
    monolith 7 5 E
    monolith 2 4 N
    belt 2..=7 3 E
    belt 8 3..=4 S
    monolith 2 6 S
    belt 2..=7 7 E
    belt 8 7..=6 N
    balancer 3 5 E
    monolith 3 4 N
    monolith 3 6 S
    balancer 4 5 E
    monolith 4 6 S
    gateway 0 6 E
    belt 1 6 N
    balancer 5 5 E
    monolith 5 4 N
    monolith 5 6 S
    balancer 6 5 E
    monolith 6 4 N
    monolith 6 6 S
end

# Keys and caches

puzzle cacheHit
    belt 4 4 E
    cache 5 4 E
    renderer 6 4 E
end

puzzle signedIn
    internet 8 4 E
    renderer 7 4 E
    balancer 6 4 E
    database 5 4 E
    belt 3..=4 4 E
    router 2 4 E
    auth 1 4 E
    gateway 0 4 E
    renderer 6 3 N
    belt 6..=7 2 E
    belt 8 2..=3 S
end

puzzle zeroCopy
    switch 2 5 E
    rack 2 6 E
    internet 5 4 E
    renderer 4 4 E
    cache 3 4 E
    router 2 4 E
    auth 1 4 E
    gateway 0 4 E
end

puzzle coldCache
    switch 3 2 E
    switch 8 2 E
    rack 6 2 E
    internet 13 1 E
    database 12 1 E
    belt 4..=11 1 E
    router 3 1 E
    logger 2 0 E
    parser 2 1 E
    gateway 1 1 E
    internet 6 3 E
    renderer 5 3 E
    router 3 3 E
    logger 2 2 E
    parser 2 3 E
    gateway 1 3 E
    cache 4 3 E
    internet 6 5 E
    renderer 5 5 E
    router 3 5 E
    logger 2 4 E
    parser 2 5 E
    gateway 1 5 E
    cache 4 5 E
    rack 8 4 E
    switch 4 6 E
    internet 6 7 E
    renderer 5 7 E
    router 3 7 E
    logger 2 6 E
    parser 2 7 E
    gateway 1 7 E
    cache 4 7 E
end

puzzle bottleneck
    remove 9 4
    cache 9 4 E
    remove 10 4
    balancer 10 4 E
    rack 9 8 E
    renderer 10 3 N
    belt 10..=11 2 E
    belt 12 2..=3 S
    renderer 10 5 S
    belt 10..=11 6 E
    belt 12 6..=5 N
end

puzzle edgeCache
    rack 4 2 E
    switch 3 4 E
    switch 3 9 E
    cache 2 7 E
    balancer 3 7 E
    renderer 4 7 E
    belt 5..=12 7 E
    belt 2 3..=6 S
    router 2 2 S
    logger 3 1 E
    parser 2 1 S
    gateway 2 0 S
    renderer 3 6 N
    belt 3..=4 5 E
    belt 5 5..=6 S
    belt 2 11..=8 N
    router 2 12 N
    logger 1 13 E
    parser 2 13 N
    gateway 2 14 N
    rack 22 10 E
    switch 22 9 E
    belt 19..=14 7 W
    renderer 20 7 W
    cache 21 7 W
    router 22 7 W
    logger 23 8 E
    parser 23 7 W
    gateway 24 7 W
end

puzzle sessionStore
    switch 5 2 E
    rack 5 3 E
    internet 10 1 E
    renderer 9 1 E
    database 8 1 E
    belt 4..=7 1 E
    router 3 1 E
    logger 2 0 E
    parser 2 1 E
    gateway 1 1 E
    internet 9 4 E
    database 8 4 E
    belt 4..=7 4 E
    router 3 4 E
    logger 2 3 E
    parser 2 4 E
    gateway 1 4 E
end

puzzle roadmap
    switch 4 6 E
    internet 4 7 E
    belt 3 7 E
    logger 2 6 E
    parser 2 7 E
    gateway 1 7 E
    internet 7 5 E
    database 6 5 E
    belt 4..=5 5 E
    router 3 5 E
    logger 2 4 E
    parser 2 5 E
    gateway 1 5 E
    research authentication
    research caching
end

puzzle thunderingHerd
    switch 3 1 E
    rack 2 0 E
    switch 6 8 E
    rack 6 9 E
    internet 8 4 E
    renderer 7 4 E
    balancer 6 4 E
    balancer 5 4 E
    cache 4 4 E
    belt 3 4 E
    belt 3 3 S
    router 2 4 E
    auth 1 4 E
    gateway 0 4 E
    renderer 5 3 N
    renderer 6 3 N
    belt 5..=7 2 E
    belt 8 2..=3 S
    renderer 5 5 S
    renderer 6 5 S
    belt 5..=7 6 E
    belt 8 6..=5 N
    router 2 3 E
    auth 1 3 E
    gateway 0 3 E
end

puzzle blackFriday
    switch 3 1 E
    rack 2 0 E
    switch 6 8 E
    rack 6 9 E
    internet 8 4 E
    renderer 7 4 E
    balancer 6 4 E
    balancer 5 4 E
    cache 4 4 E
    belt 3 4 E
    belt 3 3 S
    router 2 4 E
    auth 1 4 E
    gateway 0 4 E
    renderer 5 3 N
    renderer 6 3 N
    belt 5..=7 2 E
    belt 8 2..=3 S
    renderer 5 5 S
    renderer 6 5 S
    belt 5..=7 6 E
    belt 8 6..=5 N
    router 2 3 E
    auth 1 3 E
    gateway 0 3 E
    switch 3 11 E
    rack 2 10 E
    switch 6 18 E
    rack 6 19 E
    internet 8 14 E
    renderer 7 14 E
    balancer 6 14 E
    balancer 5 14 E
    cache 4 14 E
    belt 3 14 E
    belt 3 13 S
    router 2 14 E
    auth 1 14 E
    gateway 0 14 E
    renderer 5 13 N
    renderer 6 13 N
    belt 5..=7 12 E
    belt 8 12..=13 S
    renderer 5 15 S
    renderer 6 15 S
    belt 5..=7 16 E
    belt 8 16..=15 N
    router 2 13 E
    auth 1 13 E
    gateway 0 13 E
end

# Up the stack

puzzle secondFloor
    up 5 4 E
    up 5 4 E 1
    monolith 6 4 E 1
    internet 7 4 E 1
    switch 7 5 E 1
    gateway 4 4 E
end

puzzle rooftop
    up 6 5 E
    up 6 5 S 3
    belt 3 0..=4 S
    belt 3..=5 5 E
    gateway 1 5 E
    monolith 2 5 E
    gateway 1 4 E
    monolith 2 4 E
    gateway 1 3 E
    monolith 2 3 E
    gateway 1 2 E
    monolith 2 2 E
    gateway 1 1 E
    monolith 2 1 E
    gateway 1 0 E
    monolith 2 0 E
end

puzzle layerCake
    rack 5 6 E 1
    switch 5 5 E 1
    up 2 3 E
    up 2 3 E 1
    parser 3 3 E 1
    logger 3 2 E 1
    router 4 3 E 1
    down 5 3 E 1
    down 5 3 E
    database 6 3 E
    up 7 3 E
    up 7 3 E 1
    renderer 8 3 E 1
    internet 9 3 E 1
    gateway 1 3 E
end

puzzle walledGarden
    up 3 3 E
    up 3 3 E 1
    belt 4..=8 3 E 1
    down 9 3 E 1
    down 9 3 E
    parser 10 3 E
    logger 10 2 E
    router 11 3 E
    database 12 3 S
    renderer 12 4 W
    belt 11..=10 4 W
    up 9 4 W
    up 9 4 W 1
    belt 8..=5 4 W 1
    down 4 4 S 1
    down 4 4 S
    gateway 2 3 E
end

puzzle infrastructure
    research loadBalancing
    switch 4 6 E
    internet 7 5 E
    database 6 5 E
    belt 4..=5 5 E
    router 3 5 E
    logger 2 4 E
    parser 2 5 E
    gateway 1 5 E
    research fiberLinks
    research messaging
    research stack
end

puzzle overpass
    parser 2 3 E
    logger 2 2 E
    router 3 3 S
    belt 3 4..=14 S
    belt 3..=7 15 E
    up 8 15 E
    up 8 15 E 1
    belt 9..=11 15 E 1
    down 12 15 E 1
    down 12 15 E
    belt 13..=15 15 E
    database 16 15 E
    renderer 17 15 E
    internet 18 15 E
    switch 16 13 E
    rack 16 12 E
    gateway 1 3 E
end

puzzle expressLift
    rack 5 6 E 2
    switch 5 5 E 2
    up 2 3 E
    up 2 3 E 2
    parser 3 3 E 2
    logger 3 2 E 2
    router 4 3 E 2
    down 5 3 E 2
    down 5 3 E
    database 6 3 E
    up 7 3 E
    up 7 3 E 2
    renderer 8 3 E 2
    internet 9 3 E 2
    gateway 1 3 E
end

puzzle penthouse
    switch 7 6 E 3
    switch 9 6 E 3
    switch 7 8 E 3
    switch 9 8 E 3
    rack 4 4 E 3
    rack 12 10 E 3
end

puzzle quarterlyReport
    switch 5 2 E
    rack 6 4 E
    internet 10 1 E
    renderer 9 1 E
    database 8 1 E
    belt 4..=7 1 E
    router 3 1 E
    logger 2 0 E
    parser 2 1 E
    gateway 1 1 E
    internet 4 3 E
    belt 3 3 E
    logger 2 2 E
    parser 2 3 E
    gateway 1 3 E
    switch 5 6 E
    internet 9 5 E
    database 8 5 E
    belt 4..=7 5 E
    router 3 5 E
    logger 2 4 E
    parser 2 5 E
    gateway 1 5 E
end

puzzle hyperscale
    switch 3 1 E
    rack 2 0 E
    switch 6 8 E
    rack 6 9 E
    internet 8 4 E
    renderer 7 4 E
    balancer 6 4 E
    balancer 5 4 E
    cache 4 4 E
    belt 3 4 E
    belt 3 3 S
    router 2 4 E
    auth 1 4 E
    gateway 0 4 E
    renderer 5 3 N
    renderer 6 3 N
    belt 5..=7 2 E
    belt 8 2..=3 S
    renderer 5 5 S
    renderer 6 5 S
    belt 5..=7 6 E
    belt 8 6..=5 N
    router 2 3 E
    auth 1 3 E
    gateway 0 3 E
    switch 3 11 E
    rack 2 10 E
    switch 6 18 E
    rack 6 19 E
    internet 8 14 E
    renderer 7 14 E
    balancer 6 14 E
    balancer 5 14 E
    cache 4 14 E
    belt 3 14 E
    belt 3 13 S
    router 2 14 E
    auth 1 14 E
    gateway 0 14 E
    renderer 5 13 N
    renderer 6 13 N
    belt 5..=7 12 E
    belt 8 12..=13 S
    renderer 5 15 S
    renderer 6 15 S
    belt 5..=7 16 E
    belt 8 16..=15 N
    router 2 13 E
    auth 1 13 E
    gateway 0 13 E
    switch 3 21 E
    rack 2 20 E
    switch 6 28 E
    rack 6 29 E
    internet 8 24 E
    renderer 7 24 E
    balancer 6 24 E
    balancer 5 24 E
    cache 4 24 E
    belt 3 24 E
    belt 3 23 S
    router 2 24 E
    auth 1 24 E
    gateway 0 24 E
    renderer 5 23 N
    renderer 6 23 N
    belt 5..=7 22 E
    belt 8 22..=23 S
    renderer 5 25 S
    renderer 6 25 S
    belt 5..=7 26 E
    belt 8 26..=25 N
    router 2 23 E
    auth 1 23 E
    gateway 0 23 E
end
