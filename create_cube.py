import requests
import json


import json
import numpy as np

def create_large_cube(n):
    # Create an array to store the vertices
    vertices = []
    # Create an array to store the faces
    faces = []

    # Calculate the size of each grid cell
    cell_size = 1.0 / n

    # Create the faces and vertices for each face of the cube
    for i in range(6):
        for j in range(n):
            for k in range(n):
                # Calculate the base index for this grid cell
                base_index = len(vertices)

                if i == 0:  # top face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, 0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, 0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, 0.5, (k + 1) * cell_size - 0.5],
                        [(j + 0) * cell_size - 0.5, 0.5, (k + 1) * cell_size - 0.5]
                    ])
                elif i == 1:  # bottom face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, -0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, -0.5, (k + 0) * cell_size - 0.5],
                        [(j + 1) * cell_size - 0.5, -0.5, (k + 1) * cell_size - 0.5],
                        [(j + 0) * cell_size - 0.5, -0.5, (k + 1) * cell_size - 0.5]
                    ])
                elif i == 2:  # front face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5, 0.5],
                        [(j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5, 0.5],
                        [(j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5, 0.5],
                        [(j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5, 0.5]
                    ])
                elif i == 3:  # back face
                    vertices.extend([
                        [(j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5, -0.5],
                        [(j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5, -0.5],
                        [(j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5, -0.5],
                        [(j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5, -0.5]
                    ])
                elif i == 4:  # right face
                    vertices.extend([
                        [0.5, (j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [0.5, (j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [0.5, (j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5],
                        [0.5, (j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5]
                    ])
                elif i == 5:  # left face
                    vertices.extend([
                        [-0.5, (j + 0) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [-0.5, (j + 1) * cell_size - 0.5, (k + 0) * cell_size - 0.5],
                        [-0.5, (j + 1) * cell_size - 0.5, (k + 1) * cell_size - 0.5],
                        [-0.5, (j + 0) * cell_size - 0.5, (k + 1) * cell_size - 0.5]
                    ])

                # Add two faces for this grid cell
                faces.append([base_index, base_index + 1, base_index + 2])
                faces.append([base_index, base_index + 2, base_index + 3])

    # Return the vertices and faces as a dictionary
    return {'vertices': vertices, 'faces': faces}

# Generate a cube with 10 squares per side
large_cube = create_large_cube(5)


# Send the cube to the web app
print("sending cube")
response = requests.post('http://localhost:8000/model', json=large_cube)
print(response.status_code)