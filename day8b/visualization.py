import networkx as nx
import matplotlib.pyplot as plt

def plot_subpaths(G: {}, instructions: str):

    starting_points = []
    ending_points = []
    for key, value in G.items():
        if str(key).__contains__("A"):
            starting_points.append(key)
        elif str(key).__contains__("Z"):
            ending_points.append(key)

    current_nodes = starting_points.copy()
    index = 0
    while index < 100000:
        c = instructions[index % len(instructions)]
                
        current_nodes = [G[x][0] if c == 'L' else G[x][1] for x in current_nodes]
        
        reached_points = [(x, ind) for ind, x in enumerate(current_nodes) if x in ending_points]

        for x, i in reached_points:
            print("Reached point from -", starting_points[i], " - to - ", x , "in", index + 1, "steps")

        index += 1

def simulate_path(G: {}, instructions: str, starting_point = "AAA", max=1000000):
    
    current_node = starting_point
    index = 0
    hits = []
    while index < max:
        c = instructions[index % len(instructions)]
        current_node = G[current_node][0] if c == 'L' else G[current_node][1]

        if current_node.__contains__("Z"):
            hits.append(index + 1)
        
        index += 1
    
    # print("Hits:", hits)

    # Check if hits are spaced evenly
    diffs = [hits[i] - hits[i - 1] for i in range(1, len(hits))]
    diffs_true = [diffs[i] == diffs[i - 1] for i in range(1, len(diffs))]
    if all(diffs_true):
        print("Hits are evenly spaced by", diffs[0])
    else:
        print("Hits are not evenly spaced")
    

def plot_graph(G: nx.DiGraph):

    color_map = []
    for node in G:
        if node.__contains__("A"):
            color_map.append('red')
        elif node.__contains__("Z"):
            color_map.append('green')
        else:
            color_map.append('blue') 
    
    plt.figure(figsize=(15, 15))
    pos = nx.nx_agraph.graphviz_layout(G, prog='neato')
    nx.draw_networkx_nodes(G, pos, node_size=10, node_color=color_map)
    nx.draw_networkx_edges(G, pos, node_size=10, arrows=True)
    plt.show()

# open input file
# 8 starting locations that have A
with open('input.txt', encoding="utf-8") as f:
    instructions = f.readline().strip()

    print("Instructions:", instructions)

    f.readline() # skip first two lines

    G = nx.DiGraph()
    py_G = {}

    # create graph
    for line in f:
        split = line.split('=')
        node = split[0].strip()
        right = split[1].strip()

        children = [x.replace("(", ""). replace(")", "").strip() for x in right.split(", ")]
        
        G.add_node(node)
        G.add_node(children[0])
        G.add_node(children[1])
        G.add_edge(node, children[0])
        G.add_edge(node, children[1])

        py_G[node] = children

    for node in [x for x in py_G.keys() if x.__contains__("A")]:
        print(node)
        simulate_path(py_G, instructions, starting_point=node)
    # plot_subpaths(py_G, instructions)
    # plot_graph(G)