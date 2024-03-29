import os
import json
import pandas as pd
from copy import copy

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_input(raw_data_input):
    """
        Returns a dict containg 2 df, one storing the maze char, the other one only None, it will store shortest distances from the start tile.
    """
    return {"maze":pd.DataFrame(data=[list(x) for x in raw_data_input], columns=[x for x in range(0,len(raw_data_input[0]))], dtype=str),
            "dist":pd.DataFrame(data=[list(-1 for y in range(0,len(raw_data_input))) for x in raw_data_input], columns=[x for x in range(0,len(raw_data_input[0]))], dtype=int),}

def find_maze_start(df):
    """
        Returns coordinates of the S tile from the maze
    """
    for col in range(0, len(df.columns)):
        for row in range(0, len(df)):
            if df[col][row] == 'S':
                return [col, row]
            
def init_dataframes(dfs):
    """
        Initializes maze and dist dataframes
        Set start tile at a dist of 0, and connected surrounding tile at a dist of 1
        Replace the S with the proper pipe char
        Returns the start_coords and the 2 other pipes that the start is connected to
    """
    start_coords = find_maze_start(dfs["maze"])
    dfs["dist"][start_coords[0]][start_coords[1]] = 0
    arounds = {"north":[start_coords[0], start_coords[1]-1], 
               "south":[start_coords[0], start_coords[1]+1], 
               "east":[start_coords[0]+1, start_coords[1]], 
               "west":[start_coords[0]-1, start_coords[1]]}
    start_connected_to = []
    if dfs["maze"][arounds["north"][0]][arounds["north"][1]] in ["|", "7", "F"]:
        start_connected_to.append("north")
    if dfs["maze"][arounds["south"][0]][arounds["south"][1]] in ["|", "L", "J"]:
        start_connected_to.append("south")
    if dfs["maze"][arounds["east"][0]][arounds["east"][1]] in ["-", "J", "7"]:
        start_connected_to.append("east")
    if dfs["maze"][arounds["west"][0]][arounds["west"][1]] in ["-", "L", "F"]:
        start_connected_to.append("west")
    if start_connected_to == ["north", "south"]:
        start_char = "|"
    elif start_connected_to == ["east", "west"]:
        start_char = "-"
    elif start_connected_to == ["north", "east"]:
        start_char = "L"
    elif start_connected_to == ["north", "west"]:
        start_char = "J"
    elif start_connected_to == ["south", "west"]:
        start_char = "7"
    elif start_connected_to == ["south", "east"]:
        start_char = "F"
    else:
        raise Exception("Unknown surroundings of start_char")
    dfs["maze"][start_coords[0]][start_coords[1]] = start_char
    dfs["dist"][arounds[start_connected_to[0]][0]][arounds[start_connected_to[0]][1]] = 1
    dfs["dist"][arounds[start_connected_to[1]][0]][arounds[start_connected_to[1]][1]] = 1
    return start_coords, arounds[start_connected_to[0]], arounds[start_connected_to[1]]

def get_next_coordinates(maze_char, previous_coords, current_coords):
    """
        Returns current_cords & next_coords
    """
    last_move_vect = [current_coords[0]-previous_coords[0], current_coords[1]-previous_coords[1]]
    if maze_char == "|":
        if last_move_vect == [0, 1]:
            vect = [0, 1]
        elif last_move_vect == [0, -1]:
            vect = [0, -1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "-":
        if last_move_vect == [1, 0]:
            vect = [1, 0]
        elif last_move_vect == [-1, 0]:
            vect = [-1, 0]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "L":
        if last_move_vect == [0, 1]:
            vect = [1, 0]
        elif last_move_vect == [-1, 0]:
            vect = [0, -1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "J":
        if last_move_vect == [0, 1]:
            vect = [-1, 0]
        elif last_move_vect == [1, 0]:
            vect = [0, -1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "7":
        if last_move_vect == [1, 0]:
            vect = [0, 1]
        elif last_move_vect == [0, -1]:
            vect = [-1, 0]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "F":
        if last_move_vect == [0, -1]:
            vect = [1, 0]
        elif last_move_vect == [-1, 0]:
            vect = [0, 1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == ".":
        raise Exception("Error while getting the next coordinates. Ground tile found")
    elif maze_char == "S":
        raise Exception("Error while getting the next coordinates. Start tile found")
    else:
        raise Exception("Error while getting the next coordinates. Unknown tile found")
    next_coords = [current_coords[0]+vect[0], current_coords[1]+vect[1]]
    return current_coords, next_coords

def process_dataframes(dfs):
    """
        Navigate through the maze on both ends of the start tile
        Stops when a navigation agent reach a tile with a distance value already set
    """
    navig_agent_1 = {"previous":None, "current":None}
    navig_agent_2 = {"previous":None, "current":None}
    start_coords, navig_agent_1["current"], navig_agent_2["current"] = init_dataframes(dfs)
    navig_agent_1["previous"] = copy(start_coords)
    navig_agent_2["previous"] = copy(start_coords)
    loop_closed = False
    distance = 1
    while not loop_closed:
        distance += 1
        # Agent 1 Navigation
        navig_agent_1["previous"], navig_agent_1["current"] = get_next_coordinates(maze_char=dfs["maze"][navig_agent_1["current"][0]][navig_agent_1["current"][1]],
                                                                                   previous_coords=navig_agent_1["previous"],
                                                                                   current_coords=navig_agent_1["current"])
        # Agent 1 Distance Value Set
        dfs["dist"][navig_agent_1["current"][0]][navig_agent_1["current"][1]] = distance

        # Agent 2 Navigation
        navig_agent_2["previous"], navig_agent_2["current"] = get_next_coordinates(maze_char=dfs["maze"][navig_agent_2["current"][0]][navig_agent_2["current"][1]],
                                                                                   previous_coords=navig_agent_2["previous"],
                                                                                   current_coords=navig_agent_2["current"])
        # Agent 2 Distance Value Set
        dfs["dist"][navig_agent_2["current"][0]][navig_agent_2["current"][1]] = distance

        #Check if loop closed
        if navig_agent_1["current"] == navig_agent_2["current"]:
            loop_closed = True
    return distance


input_file_path = os.path.join(os.getcwd(), "input.txt")
data_frames = parse_input(load_input(input_file_path))

result = process_dataframes(data_frames)
print(result)

test_maze = data_frames["maze"].to_html()
test_dist = data_frames["dist"].to_html()
test_maze_file_path = os.path.join(os.getcwd(), "maze_df.html")
with open(test_maze_file_path, 'w') as f:
    f.write(test_maze)
test_dist_file_path = os.path.join(os.getcwd(), "dist_df.html")
with open(test_dist_file_path, 'w') as f:
    f.write(test_dist)