import os
import json
from copy import copy

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_input(raw_data_input):
    """
        Returns a list of lists
    """
    result = []
    for line in raw_data_input:
        result.append([int(x) for x in line.split(" ")])
    return result

def check_sequence(seq):
    """
        Returns True if all objects of seq are 0
    """
    for obj in seq:
        if obj != 0:
            return False
    return True


input_file_path = os.path.join(os.getcwd(), "input.txt")
data_input = parse_input(load_input(input_file_path))
result = 0
for to_process_sequence in data_input:
    sequences = []
    sequences.append(copy(to_process_sequence))
    # Substract each int from the the previous until a list of only 0 appears
    while not check_sequence(sequences[-1]):
        to_append_sequence = [sequences[-1][i]-sequences[-1][i-1] for i in range(1, len(sequences[-1]))]
        sequences.append(to_append_sequence)
    # Insert a 0 at the begining of the list of 0
    sequences[-1].insert(0, 0)
    # Compute last int of the the second to last list --> Insert the value to its left at the begining
    sequences[-2].insert(0, sequences[-2][-1])
    # Compte all lasts int of the upper sequences --> Append the value to its left + the last value of the previous sequence
    n = len(sequences)
    print(sequences)
    for index_sequence in range(-3, -(n+1), -1):
        sequences[index_sequence].insert(0, sequences[index_sequence][0] - sequences[index_sequence+1][0])
        print(sequences[index_sequence])
    # Add the (computed) next int of each sequence to the result
    result += sequences[0][0]
print(sequences)
print(result)

