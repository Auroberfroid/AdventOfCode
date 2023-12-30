import os
import json
from threading import Thread
from datetime import datetime

###################################################################
# BRUTE FORCE SOLUTION TAKES ~ 20k HOURS TO COMPUTE THE RESULT... #
#               CHECK LCM FOR REALISTIC SOLUTION                  #
###################################################################

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

def reindex_as_int_nodes(raw_nodes):
    """
        Reindex all the node in a list, storing start & end nodes indexes for better performance
        Return reindexed_nodes, start_nodes_indexes, end_nodes_indexes, nodes_indexes_mapp
    """
    start_nodes_indexes = [x for x in raw_nodes.keys() if x[2] == 'A']
    end_nodes_indexes = [x for x in raw_nodes.keys() if x[2] == 'Z']
    nodes_indexes_mapp = {}
    i = 0
    for key in raw_nodes.keys():
        if key =='instructions':
            continue
        nodes_indexes_mapp[key] = i
        i += 1
    reindexed_nodes = []
    for node_mapp in nodes_indexes_mapp.keys():
        reindexed_nodes.insert(nodes_indexes_mapp[node_mapp], (nodes_indexes_mapp[raw_nodes[node_mapp][0]], nodes_indexes_mapp[raw_nodes[node_mapp][1]]))
    for index_node, node in enumerate(start_nodes_indexes):
        start_nodes_indexes[index_node] = nodes_indexes_mapp[start_nodes_indexes[index_node]]
    for index_node, node in enumerate(end_nodes_indexes):
        end_nodes_indexes[index_node] = nodes_indexes_mapp[end_nodes_indexes[index_node]]
    return reindexed_nodes, start_nodes_indexes, end_nodes_indexes, nodes_indexes_mapp 

def check_end_reached(nodes):
    """
        Checks if we reached the end
    """
    for node in nodes:
        if not node in END_NODES:
            return False
    return True

input_file_path = os.path.join(os.getcwd(), "input.txt")
raw_nodes = parse_input(load_input(input_file_path))

reindexed_nodes, START_NODES, END_NODES, _ = reindex_as_int_nodes(raw_nodes)

instructions = raw_nodes["instructions"]

result = 0
found = False
current_nodes = START_NODES
start = datetime.now()
while not found:
    for instruct in instructions:
        if check_end_reached(current_nodes):
            found = True
            break
        for index_in_current_nodes, node in enumerate(current_nodes):
            current_nodes[index_in_current_nodes] = reindexed_nodes[node][instruct]
        result += 1
        if result%10000000 == 0:
            print('10M reached')
            found = True
            break
print((datetime.now() - start).total_seconds())
print(result)
