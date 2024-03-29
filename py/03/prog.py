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
    for index_line, line in enumerate(lines):
        # Get the int value
        last_char_was_int = False
        dico = {}
        len_line = len(line)
        for index_char, char in enumerate(line):
            try:
                # Get the int value in the dict
                char_as_int = int(char)
                if last_char_was_int:
                    dico["value"] = 10 * dico["value"] + char_as_int
                    last_char_was_int = True
                else:
                    dico["value"] = char_as_int
                    dico["line_nb"] = index_line
                    dico["char_index"] = index_char
                    dico["id"] = id
                    id += 1
                    last_char_was_int = True
                if index_char + 1 == len_line:
                    len_int = len(str(dico["value"]))
                    dico["right"] = "."
                    dico["left"] = line[index_char - len_int]
                    if index_line == 0:
                        dico["top"] = "." * (len_int + 2)
                        dico["bot"] = lines[index_line + 1][index_char - (len_int + 1):index_char] + '.'
                    elif index_line == nb_lines - 1:
                        dico["bot"] = "." * (len_int + 2)
                        dico["top"] = lines[index_line - 1][index_char - (len_int + 1):index_char] + '.'
                    else:
                        dico["bot"] = lines[index_line + 1][index_char - (len_int + 1):index_char] + '.'
                        dico["top"] = lines[index_line - 1][index_char - (len_int + 1):index_char] + '.'
                    result.append(dico)
            except:
                # Get the surroundings if last char_was_int but current isn't
                if last_char_was_int:
                    # Handle and fetch left & right
                    # Check we use valid index
                    len_int = len(str(dico["value"]))
                    if index_char - len_int <= 0:
                        dico["left"] = "."
                        dico["right"] = line[index_char]
                    else:
                        dico["right"] = line[index_char]
                        dico["left"] = line[index_char - (len_int + 1)]
                    # Handle and fetch top and bot
                    if index_line == 0:
                        dico["top"] = "." * (len_int + 2)
                        dico["bot"] = lines[index_line + 1][index_char - (len_int + 1):index_char + 1]
                    elif index_line == nb_lines - 1:
                        dico["bot"] = "." * (len_int + 2)
                        dico["top"] = lines[index_line - 1][index_char - (len_int + 1):index_char + 1]
                    else:
                        if index_char - len_int <= 0:
                            dico["bot"] = '.' + lines[index_line + 1][index_char - (len_int):index_char + 1]
                            dico["top"] = '.' + lines[index_line - 1][index_char - (len_int):index_char + 1]
                        else:
                            dico["bot"] = lines[index_line + 1][index_char - (len_int + 1):index_char + 1]
                            dico["top"] = lines[index_line - 1][index_char - (len_int + 1):index_char + 1]
                    result.append(dico)
                    dico = {}
                last_char_was_int = False 
    return result
            
input_file_path = os.path.join(os.getcwd(), "input.txt")

schema_lines = load_input(input_file_path)
data_dico_list = parse_surroundings_lines(schema_lines)


output_file_path = os.path.join(os.getcwd(), "output.json")
with open(output_file_path, 'w') as fd:
    json.dump(data_dico_list, fd)

# Question 1
'''
result = 0
for dico in data_dico_list:
    if not (len(dico["top"]) * "." == dico["top"] and len(dico["bot"]) * "." == dico["bot"] and dico["left"] == "." and dico["right"] == "."):
        #print(dico["value"])
        result = result + dico["value"]
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