import os
import json
from math import sqrt, ceil

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_input(raw_data_input):
    """
        Returns a list of dict like:
        [{time_limit:int, distance_record:int}, ...]
    """
    result = []
    for time_limit, distance_record in zip([int(_time_limit) for _time_limit in raw_data_input[0].split(":")[1].split(" ") if _time_limit != ""], [int(_distance_record) for _distance_record in raw_data_input[1].split(":")[1].split(" ") if _distance_record != ""]):
        result.append({"time_limit":time_limit, "distance_record":distance_record})
    return result

def compute_solution_analytic(data_input):
    """
        Returns the (int) solutions of the equation: -h² + T*h - R = 0, with h:hold_duration, T:time_limit, R:distance_record. 0 <= h <= T
    """
    for dico in data_input:
        T = dico["time_limit"]
        R = dico["distance_record"]
        dico["nb_solution"] = 0
        delta = T**2 - 4*R
        h_1 = ceil((-T - sqrt(delta))/(-2))
        h_2 = ceil((-T + sqrt(delta))/(-2))
        if 0 < h_1 < T:
            dico["nb_solution"] += T - h_1
        if 0 < h_2 < T:
            dico["nb_solution"] += T - h_2

def compute_solution_numeric(data_input):
    """
    """
    for dico in data_input:
        T = dico["time_limit"]
        R = dico["distance_record"]
        dico["nb_solution"] = 0
        for h in range(0, T+1):
            equation_result = h*(T-h) - R
            if equation_result >= 0:
                dico["nb_solution"] += 1

input_file_path = os.path.join(os.getcwd(), "input.txt")
data_input = parse_input(load_input(input_file_path))
compute_solution_numeric(data_input)

result = 1
for dico in data_input:
    result = result * dico["nb_solution"]
print(result)

output_file_path = os.path.join(os.getcwd(), "data.json")
with open(output_file_path, 'w') as fd:
    json.dump(data_input, fd)