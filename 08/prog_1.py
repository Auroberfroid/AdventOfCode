import os
import json

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_input(raw_data_input):
    """
        Returns a dict like {"Instructions":[0,1,0,...], "NODE_CHAR":("NODE_CHAR", "NODE_CHAR"), ...}
    """
    result = {}
    mapp = {'L':0, 'R':1}
    result["instructions"] = [mapp[x] for x in raw_data_input[0]]
    for node in raw_data_input[2:]:
        key = node.split(' = ')[0]
        values = tuple(node.split(' = ')[1].replace("(","").replace(")","").replace(" ", "").split(','))
        result[key] = values
    return result

def process_node(node, instruct):
    """
        Navigates through one node
    """
    node = node[instruct]

def check_end_reached(node):
    """
        Checks if we reached the end (eg. all nodes last char == 'Z')
    """
    if node != 'ZZZ':
            return False
    return True

input_file_path = os.path.join(os.getcwd(), "input.txt")
data_input = parse_input(load_input(input_file_path))

START = 'AAA'
END = 'ZZZ'
result = 0
found = False
node = START
while not found:
    for instruct in data_input['instructions']:
        if check_end_reached(node):
            found = True
            break
        process_node(node, instruct)
        result += 1
print(result)
