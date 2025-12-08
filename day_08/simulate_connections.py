import math

boxes = [
    (162,817,812),  # 0
    (57,618,57),    # 1
    (906,360,560),  # 2
    (592,479,940),  # 3
    (352,342,300),  # 4
    (466,668,158),  # 5
    (542,29,236),   # 6
    (431,825,988),  # 7
    (739,650,466),  # 8
    (52,470,668),   # 9
    (216,146,977),  # 10
    (819,987,18),   # 11
    (117,168,530),  # 12
    (805,96,715),   # 13
    (346,949,466),  # 14
    (970,615,88),   # 15
    (941,993,340),  # 16
    (862,61,35),    # 17
    (984,92,344),   # 18
    (425,690,689)   # 19
]

def distance(b1, b2):
    return math.sqrt((b1[0]-b2[0])**2 + (b1[1]-b2[1])**2 + (b1[2]-b2[2])**2)

# Create circuits - each box starts in its own circuit
circuits = [{i} for i in range(len(boxes))]

def find_circuit(box_idx):
    for i, circuit in enumerate(circuits):
        if box_idx in circuit:
            return i
    return None

def merge_circuits(idx1, idx2):
    if idx1 == idx2:
        print(f"  Already in same circuit!")
        return
    if idx1 > idx2:
        idx1, idx2 = idx2, idx1
    circuit2 = circuits.pop(idx2)
    circuits[idx1].update(circuit2)

# Find all pairs and sort by distance
pairs = []
for i in range(len(boxes)):
    for j in range(i+1, len(boxes)):
        dist = distance(boxes[i], boxes[j])
        pairs.append((dist, i, j))
pairs.sort()

print("Making 10 connections:\n")
connections_made = 0
for dist, idx1, idx2 in pairs:
    circuit1 = find_circuit(idx1)
    circuit2 = find_circuit(idx2)
    
    if circuit1 == circuit2:
        print(f"Connection {connections_made + 1}: Skip - boxes {idx1} and {idx2} already connected (distance {dist:.2f})")
        continue
    
    connections_made += 1
    print(f"Connection {connections_made}: Connect boxes {idx1} {boxes[idx1]} and {idx2} {boxes[idx2]} (distance {dist:.2f})")
    merge_circuits(circuit1, circuit2)
    print(f"  Circuits now: {len(circuits)} circuits")
    
    if connections_made >= 10:
        break

print(f"\nFinal circuits: {len(circuits)}")
sizes = sorted([len(c) for c in circuits])
print(f"Circuit sizes: {sizes}")
print(f"Three largest: {sizes[-3:]}")
print(f"Product: {sizes[-1] * sizes[-2] * sizes[-3]}")
