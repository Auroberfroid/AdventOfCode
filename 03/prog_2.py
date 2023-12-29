import os
import json

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data
        
def parse_surroundings_lines(lines):
    """
        We take the full input lines list as input
        We get the int valuen and store all its surrounding characters in a dict (1 char left/right, and len(int) + 2 for top/bot
        dict_pattern = {"value":None,
                        "left":None,
                        "right":None,
                        "top":None,
                        "bot":None}
    """
    result = []
    nb_lines = len(lines)
    id = 0
    int_list_as_str = [str(x) for x in range(0,10)]
    for index_line, line in enumerate(lines):
        # Get the int value
        dico = {}
        len_line = len(line)
        for index_char, char in enumerate(line):
            # Get all * char and their surroundings
            if char == "*":
                dico["line_nb"] = index_line
                dico["char_index"] = index_char
                dico["id"] = id
                id += 1
                if index_char + 1 == len_line:
                    dico["right"] = "."
                    dico["left"] = line[index_char - 1]
                    if index_line == 0:
                        dico["top"] = "." * 3
                        dico["bot"] = lines[index_line + 1][index_char - 1:index_char] + '.'
                    elif index_line == nb_lines - 1:
                        dico["top"] = lines[index_line - 1][index_char - 1:index_char] + '.'
                        dico["bot"] = "." * 3
                    else:
                        dico["top"] = lines[index_line - 1][index_char - 1:index_char + 1] + '.'
                        dico["bot"] = lines[index_line + 1][index_char - 1:index_char + 1] + '.'
                elif index_char == 0:
                    dico["right"] = line[index_char + 1]
                    dico["left"] = "." 
                    if index_line == 0:
                        dico["top"] = "." * 3
                        dico["bot"] = '.' + lines[index_line + 1][index_char:index_char + 2]
                    elif index_line == nb_lines - 1:
                        dico["top"] = '.' + lines[index_line - 1][index_char:index_char + 2]
                        dico["bot"] = "." * 3
                    else:
                        dico["top"] = '.' + lines[index_line - 1][index_char:index_char + 2]
                        dico["bot"] = '.' + lines[index_line + 1][index_char:index_char + 2]
                else:
                    dico["right"] = line[index_char + 1]
                    dico["left"] = line[index_char - 1] 
                    if index_line == 0:
                        dico["top"] = "." * 3
                        dico["bot"] = lines[index_line + 1][index_char - 1:index_char + 2]
                    elif index_line == nb_lines - 1:
                        dico["top"] = lines[index_line - 1][index_char - 1:index_char + 2]
                        dico["bot"] = "." * 3
                    else:
                        dico["top"] = lines[index_line - 1][index_char - 1:index_char + 2]
                        dico["bot"] = lines[index_line + 1][index_char - 1:index_char + 2]
                # Check if numbers in the surroundings, and if yes, then get the full number (check id 43)
                # Left
                if dico["left"] in int_list_as_str:
                    if line[index_char - 2] in int_list_as_str:
                        dico["left"] = line[index_char - 2] + dico["left"]
                        if line[index_char - 3] in int_list_as_str:
                            dico["left"] = line[index_char - 3] + dico["left"]
                # Right
                if dico["right"] in int_list_as_str:
                    if line[index_char + 2] in int_list_as_str:
                        dico["right"] = dico["right"] + line[index_char + 2]
                        if line[index_char + 3] in int_list_as_str:
                            dico["right"] = dico["right"] + line[index_char + 3]
                # Bot
                if dico["bot"][-1] in int_list_as_str:
                    if lines[index_line + 1][index_char + 2] in int_list_as_str:
                        dico["bot"] = dico["bot"] + lines[index_line + 1][index_char + 2]
                        if lines[index_line + 1][index_char + 3] in int_list_as_str:
                            dico["bot"] = dico["bot"] + lines[index_line + 1][index_char + 3]
                if dico["bot"][0] in int_list_as_str:
                    if lines[index_line + 1][index_char - 2] in int_list_as_str:
                        dico["bot"] = lines[index_line + 1][index_char - 2] + dico["bot"]
                        if lines[index_line + 1][index_char - 3] in int_list_as_str:
                            dico["bot"] = lines[index_line + 1][index_char - 3] + dico["bot"]
                # Top
                if dico["top"][-1] in int_list_as_str:
                    if lines[index_line - 1][index_char + 2] in int_list_as_str:
                        dico["top"] = dico["top"] + lines[index_line - 1][index_char + 2]
                        if lines[index_line - 1][index_char + 3] in int_list_as_str:
                            dico["top"] = dico["top"] + lines[index_line - 1][index_char + 3]
                if dico["top"][0] in int_list_as_str:
                    if lines[index_line - 1][index_char - 2] in int_list_as_str:
                        dico["top"] = lines[index_line - 1][index_char - 2] + dico["top"]
                        if lines[index_line - 1][index_char - 3] in int_list_as_str:
                            dico["top"] = lines[index_line - 1][index_char - 3] + dico["top"]
                # Create a list with all numbers from the surroundings of asterix
                dico["numbers"] = []
                try:
                    dico["numbers"].append(int(dico["right"]))
                except:
                    pass
                try:
                    dico["numbers"].append(int(dico["left"]))
                except:
                    pass
                for int_char in dico["bot"].split("."):
                    try:
                        dico["numbers"].append(int(int_char))
                    except:
                        pass
                for int_char in dico["top"].split("."):
                    try:
                        dico["numbers"].append(int(int_char))
                    except:
                        pass
                result.append(dico)
                dico = {}
    return result
            
input_file_path = os.path.join(os.getcwd(), "input.txt")

schema_lines = load_input(input_file_path)
data_dico_list = parse_surroundings_lines(schema_lines)


output_file_path = os.path.join(os.getcwd(), "output.json")
with open(output_file_path, 'w') as fd:
    json.dump(data_dico_list, fd)

result = 0
for dico in data_dico_list:
    if len(dico["numbers"]) == 2:
        result += dico["numbers"][0] * dico["numbers"][1]
print(result)

'''
# Question 2 
# Get dicts with * in their surroundings
dico_with_asterix_list = []
for dico in data_dico_list:
    if '*' in dico["top"] or '*' in dico["bot"] or '*' in dico["left"] or '*' in dico["right"]:
        dico_with_asterix_list.append(dico)

# Transform it into a dict with the line_nb as key
line_nb_indexed_dico_list = {}
for dico in dico_with_asterix_list:
    if not dico["line_nb"] in line_nb_indexed_dico_list.keys():
        line_nb_indexed_dico_list[dico["line_nb"]] = []
        line_nb_indexed_dico_list[dico["line_nb"]].append(dico)
    else:
        line_nb_indexed_dico_list[dico["line_nb"]].append(dico)

# debug files
output_asterix_file_path = os.path.join(os.getcwd(), "output_asterix.json")
with open(output_asterix_file_path, 'w') as fd:
    dico_with_asterix_list_todump = {"data_wrapper":dico_with_asterix_list}
    json.dump(dico_with_asterix_list_todump, fd)

output_line_nb_indexed_file_path = os.path.join(os.getcwd(), "output_line_nb_indexed.json")
with open(output_line_nb_indexed_file_path, 'w') as fd:
    json.dump(line_nb_indexed_dico_list, fd)
'''