import os
import json

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_data(lines):
    '''
        Returns a dict like:
        {id, win_list, got_list}
    '''
    result = []
    for line in lines:
        dico = {}
        dico["id"] = int(line.split(":")[0].split("Card")[1].strip().rstrip())
        dico["win_list"] = [x for x in line.split(":")[1].split("|")[0].split(" ") if x != '']
        dico["got_list"] = [x for x in line.split(":")[1].split("|")[1].split(" ") if x != '']
        for index in range(0, len(dico["win_list"])):
            dico["win_list"][index] =  int(dico["win_list"][index].strip().rstrip())
        for index in range(0, len(dico["got_list"])):
            dico["got_list"][index] =  int(dico["got_list"][index].strip().rstrip())
        dico["matches"] = 0 # Question 2
        dico["copies"] = 1 # Question 2
        result.append(dico)
    return result

def compute_score(data_dico):
    """
        Question 1
    """
    for dico in data_dico:
        dico["score"] = 0
        for gotten_int in dico["got_list"]:
            if gotten_int in dico["win_list"]:
                if dico["score"] == 0:
                    dico["score"] = 1
                else:
                    dico["score"] = dico["score"] * 2

def compute_copies(dico_list):
    """
        Question 2
    """
    for dico in dico_list:
        for gotten_int in dico["got_list"]:
            if gotten_int in dico["win_list"]:
                dico["matches"] += 1

    max_dico_index = len(dico_list)
    print(f'{max_dico_index=}')
    for dico_index, dico in enumerate(dico_list):
        print(f'{dico_index + 1=} ||  || {min(dico_index + 1 + dico["matches"], max_dico_index)}')
        for nb_copy in range(0, dico["copies"]):
            for i in range(dico_index + 1, min(dico_index + 1 + dico["matches"], max_dico_index)):
                dico_list[i]["copies"] += 1

input_file_path = os.path.join(os.getcwd(), "input.txt")
input_lines = load_input(input_file_path)

data_dico_list = parse_data(input_lines)

# Question 1
"""
compute_score(data_dico_list)
result = 0
for dico in data_dico_list:
    result += dico["score"]
print(result)
"""

# Question 2
compute_copies(data_dico_list)
result = 0
for dico in data_dico_list:
    result += dico["copies"]
print(result)

output_file_path = os.path.join(os.getcwd(), "output.json")
with open(output_file_path, 'w') as fd:
    json.dump(data_dico_list, fd)