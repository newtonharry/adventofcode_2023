import networkx as nx
import matplotlib.pyplot as plt

def visualize_graph_from_file(file_path):
    # Initialize an empty graph
    G = nx.Graph()

    # Open and read the file
    with open(file_path, 'r') as file:
        for line in file:
            node1, node2 = line.strip().split()
            G.add_edge(node1, node2)

    # Dynamically adjust 'k' for spring layout to improve spread based on the graph size
    k_value = 0.4# Adjust the k value based on graph size
    pos = nx.spring_layout(G, k=k_value, iterations=2000)

    # Coloring nodes that end with 'A' differently
    node_colors = []
    for node in G.nodes():
        if node.endswith('A'):
            node_colors.append('lightgreen')  # Color for nodes ending in 'A'
        elif node.endswith('Z'):
            node_colors.append('lightblue')   # Color for nodes ending in 'Z'
        else:
            node_colors.append('gray')        # Default color for other nodes

    # Draw the graph
    nx.draw(G, pos, with_labels=True, node_color=node_colors, node_size=700)
    plt.show()

# Example usage
file_path = 'graph_input.txt'  # Replace with your file path
visualize_graph_from_file(file_path)
