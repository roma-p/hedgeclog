import os
import json
import copy

TILES_ORIG_FILE = "assets/asset_src/tiles.gltf"
TILES_DST_DIR = "assets/tiles/"


def main():
    global TILES_DST_DIR
    global TILES_ORIG_FILE

    try:
        with open(TILES_ORIG_FILE, 'r') as file:
            gltf_src_data = json.load(file)
    except json.JSONDecodeError:
        print("Error: Invalid JSON data in the file.")

    node_id_in_scene_0 = gltf_src_data["scenes"][0]["nodes"]
    nodes_id_to_names = {}

    for node in gltf_src_data["nodes"]:
        nodes_id_to_names[node["mesh"]] = node["name"]
        node["translation"] = [0, 0, 0]

    for node_id in node_id_in_scene_0:
        gltf_copied_data = copy.deepcopy(gltf_src_data)
        node_name = nodes_id_to_names[node_id]
        gltf_copied_data["scenes"][0]["nodes"] = [node_id]
        export_path = os.path.join(TILES_DST_DIR, "{}.gltf".format(node_name))
        try:
            with open(export_path, 'w', encoding='utf-8') as f:
                json.dump(gltf_copied_data, f, ensure_ascii=False, indent=4)
        except Exception as e:
            print(f"Error writing to file: {e}")


if __name__ == "__main__":
    main()
