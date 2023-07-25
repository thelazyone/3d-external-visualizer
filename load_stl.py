from stl import mesh
from model_server import run_server
import numpy as np

def load_stl(filename):
    # Load the STL files and add the vectors to the plot
    your_mesh = mesh.Mesh.from_file(filename)
    vertices = your_mesh.vectors.reshape(-1, 3).tolist()  # Flatten the vertices array
    faces = [[i, i + 1, i + 2] for i in range(0, len(vertices), 3)]  # Each consecutive trio of vertices form a face

    return {"vertices": vertices, "faces": faces}

# Load an .stl file
model = load_stl('susan.stl')

# Send the model to the web app
print(model)
run_server(model)
