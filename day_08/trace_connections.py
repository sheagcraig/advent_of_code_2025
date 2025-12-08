import math

boxes = [
    (162,817,812),
    (57,618,57),
    (906,360,560),
    (592,479,940),
    (352,342,300),
    (466,668,158),
    (542,29,236),
    (431,825,988),
    (739,650,466),
    (52,470,668),
    (216,146,977),
    (819,987,18),
    (117,168,530),
    (805,96,715),
    (346,949,466),
    (970,615,88),
    (941,993,340),
    (862,61,35),
    (984,92,344),
    (425,690,689)
]

def distance_squared(b1, b2):
    return (b1[0]-b2[0])**2 + (b1[1]-b2[1])**2 + (b1[2]-b2[2])**2

def distance(b1, b2):
    return math.sqrt(distance_squared(b1, b2))

# Find all pairs and their distances
pairs = []
for i in range(len(boxes)):
    for j in range(i+1, len(boxes)):
        dist = distance(boxes[i], boxes[j])
        pairs.append((dist, i, j, boxes[i], boxes[j]))

pairs.sort()

print("First 15 closest pairs:")
for i, (dist, idx1, idx2, b1, b2) in enumerate(pairs[:15]):
    print(f"{i+1}. Distance: {dist:.2f} - Box {idx1} {b1} <-> Box {idx2} {b2}")
