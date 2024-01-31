import networkx as nx
import matplotlib.pyplot as plt

def visualize_graph_from_file(file_path):
    # Initialize an empty graph
    G = nx.Graph()

    # Open and read the file
    with open(file_path, 'r') as file:
        for line in file:
            # Assume each line contains two node identifiers separated by space
            node1, node2 = line.strip().split()
            G.add_edge(node1, node2)

    # Identify cycles (if any)
    try:
        cycles = list(nx.find_cycle(G, orientation='ignore'))
    except nx.NetworkXNoCycle:
        cycles = []

    # Extract nodes and edges from cycles for coloring
    cycle_nodes = set(node for edge in cycles for node in edge[:2])
    cycle_edges = set((u, v) for u, v, _ in cycles)

    # Coloring nodes and edges, distinguishing cycle parts
    node_colors = ['skyblue' if node not in cycle_nodes else 'lightgreen' for node in G.nodes()]
    edge_colors = ['black' if (u, v) not in cycle_edges and (v, u) not in cycle_edges else 'red' for u, v in G.edges()]

    # Use the spring layout for better visualization
    pos = nx.spring_layout(G, k=0.4, iterations=1500)

    # Draw the graph
    nx.draw(G, pos, with_labels=True, node_color=node_colors, edge_color=edge_colors, node_size=700)
    plt.show()

# Example usage
file_path = 'graph_input.txt'  # Make sure to replace this with the actual file path
visualize_graph_from_file(file_path)




"""
new_lines = []
with open("./problem_inputs/problem_8_puzzle_input.txt") as f:
    lines = f.readlines()
    for line in lines:
        for match in re.finditer(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)", line.strip()):
            new_lines.append(f"{match.group(1)} {match.group(2)}")
            new_lines.append(f"{match.group(1)} {match.group(3)}")



with open("./graph_input.txt", "w") as f:
    f.write("\n".join(new_lines))
"""
