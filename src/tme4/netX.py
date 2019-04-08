import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph()

f = open("rand.txt", "r")


for x in f:
    tab =x.split(" ")
    source = tab[0]
    target = tab[1]
    
    G.add_edge(source, target)

nx.draw_networkx(G , )

plt.show()