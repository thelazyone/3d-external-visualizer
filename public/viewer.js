let scene, camera, renderer, mesh;

async function init() {
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.z = 5;

    renderer = new THREE.WebGLRenderer();
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);

    const response = await fetch('/model');
    const modelData = await response.json();
    mesh = createMeshFromModelData(modelData);
    scene.add(mesh);

    animate();
}

function createMeshFromModelData(modelData) {
    const geometry = new THREE.BufferGeometry();

    // Flatten the nested vertices and faces arrays
    const vertices = new Float32Array(modelData.vertices.flat());
    const faces = new Uint32Array(modelData.faces.flat());

    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    geometry.setIndex(new THREE.BufferAttribute(faces, 1));

    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });
    const mesh = new THREE.Mesh(geometry, material);

    return mesh;
}

function animate() {
    requestAnimationFrame(animate);

    mesh.rotation.x += 0.01;
    mesh.rotation.y += 0.01;

    renderer.render(scene, camera);
}

init();