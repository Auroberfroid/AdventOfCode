import os
import json
from copy import deepcopy

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def raw_data_input_to_data_input(raw_data_input):
    """
        Returns data_input from raw_data_input. data_input is a dict like:
        {
            seeds:[seed_id, location_id], #location_id set to None for now
            seed_to_soil:[
                            {
                                src_start:int,
                                dst_start:int,
                                range:int,
                            },
                            {...},
                         ]
            ...
        }
    """
    result = {}
    # Get input seeds
    result["seeds"] = [[int(x), None] for x in raw_data_input[0].split(":")[1].split(" ") if x != ""]
    # Define our mapp names [0] and mapp names from the raw_data_input [1]
    _mapp_names = [("seed_soil","seed-to-soil map:"), ("soil_fertilizer", "soil-to-fertilizer map:"),\
                   ("fertilizer_water", "fertilizer-to-water map:"), ("water_light", "water-to-light map:"),\
                   ("light_temperature", "light-to-temperature map:"), ("temperature_humidity", "temperature-to-humidity map:"),\
                   ("humidity_location", "humidity-to-location map:")]
    offset = 2
    for mapp_name in _mapp_names:
        for index_line, line in enumerate(raw_data_input[offset:]):
            if line == "": # Break in case we finish this mapping to go the next
                offset +=  index_line + 1
                break
            elif mapp_name[1] in line: # Create key for the mapping if first line of mapping
                result[mapp_name[0]] = {}
                result[mapp_name[0]]["mapping_rules"] = []
            else: # Append required data to the result
                dico = {}
                dico["dst_start"], dico["src_start"], dico["range"] = (int(x) for x in line.split(" ") if x != "")
                result[mapp_name[0]]["mapping_rules"].append(dico)
    return result

def full_load_input(file_path):
    return raw_data_input_to_data_input(load_input(file_path))

def get_serializable_data_input(data_input):
    """
        Make the data_input serializable
        HARD CODED...
    """
    def _replace_range_obj(range_obj):
        return f'range(start:{range_obj.start}, stop:{range_obj.stop}, step:{range_obj.step})'
    copy_data_input = deepcopy(data_input)
    # Define places where we have unserializable objects
    for mapp_name in copy_data_input.keys():
        if mapp_name == "seeds":
            continue
        else:
            for vect_dict in copy_data_input[mapp_name]["mapping_vectors"]:
                vect_dict["src"] = _replace_range_obj(vect_dict["src"])
                vect_dict["dst"] = _replace_range_obj(vect_dict["dst"])
    return copy_data_input

class RicoMappingVector:
    """
        Stores data about vector bounds from both side of the mapping
    """

    def __init__(self, mapping_rules):
        """
            vectors_dicts is a list of dict containing range objects for src and dst, they are of the same size
            The full processing of mapping rules is done on class instanciation
            vectors_dicts format: [{src:range, dst:range}, {...}] in a ascending order based on src_start
        """
        self.vectors_dicts = []
        for mapping_rule in mapping_rules:
            insertion_index = self.process_a_mapping_rule(mapping_rule)

    def process_a_mapping_rule(self, mapping_rule):
        """
        Process a single mapping_rule, and updates vectors accordingly
        mapping_rule format: 
            {
                "dst_start": XXXX,
                "src_start": XXXX,
                "range": XXXX
            }
        Range object are inserted with a ascending order (based on src value)
        Returns the index of insertion
        """
        SS = mapping_rule["src_start"]
        SD = mapping_rule["dst_start"]
        R = mapping_rule["range"]
        range_dico = {"src":range(SS, SS + R), "dst":range(SD, SD + R)}
        index_vect = -1
        if len(self.vectors_dicts) == 0: # Init the vect
            self.vectors_dicts.append(range_dico)
            return index_vect
        for index_vect, vect_dict in enumerate(self.vectors_dicts):
            if SS > vect_dict["src"].start: # If higher means that we need to insert inplace of the previous value <=> the current index
                if index_vect == len(self.vectors_dicts) - 1: # We reached the end of the vectors_dicts list, so we append, not insert
                    self.vectors_dicts.append(range_dico)
                    return index_vect
            else: # Insert at the current index, shifting other vect_dicts to the right of the list
                self.vectors_dicts.insert(index_vect, range_dico)
                return index_vect
        self.vectors_dicts.insert(0, range_dico) # If we didn't returned during the for loop, it means SS < all current vect_dict[src], so insert it as first vect_dict
        return index_vect

def create_all_mappingvectors(data_input):
    """
        Update the data_input dict, adding the list of vector dicts to each mapp_name
    """
    for mapp_name in data_input.keys():
        if mapp_name == 'seeds': # Skip seeds key
            continue
        mapping_vector = RicoMappingVector(data_input[mapp_name]["mapping_rules"])
        data_input[mapp_name]["mapping_vectors"] = mapping_vector.vectors_dicts

def get_mapped_object_id(object_id, vectors_dicts):
    """
        Returns the mapped(object_id) of an object_id based on vectors_dicts
    """
    for vect_dict in vectors_dicts:
        if vect_dict["src"].start <= object_id < vect_dict["src"].stop: # The object_id is within a mapping_rule
            mapp_index = object_id - vect_dict["src"].start
            return vect_dict["dst"][mapp_index]
    return object_id # The result isn't affected by any mapping_rule

def process_all_seeds(data_input):
    """
        Add the location_id for each input seeds using the data_input vectors
    """
    for seed in data_input["seeds"]:
        seed_id = seed[0]
        object_id = seed_id
        for mapp_name in data_input.keys():
            if mapp_name == "seeds": # Skip the seeds key
                continue
            object_id = get_mapped_object_id(object_id, data_input[mapp_name]["mapping_vectors"])
        seed[1] = object_id

input_file_path = os.path.join(os.getcwd(), "input.txt")
data_input = full_load_input(input_file_path)
create_all_mappingvectors(data_input)
process_all_seeds(data_input)
result = None
for seed in data_input["seeds"]:
    if result is None:
        result = seed[1]
    elif result > seed[1]:
        result = seed[1]
print(result)

data_input_file_path = os.path.join(os.getcwd(), "data_input.json")
with open(data_input_file_path, 'w') as fd:
    json.dump(get_serializable_data_input(data_input), fd)
