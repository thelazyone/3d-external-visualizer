let scene, camera, renderer, mesh, socket;

async function init() {
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.z = 5;

    renderer = new THREE.WebGLRenderer();
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);

    await setupSocket();

    animate();
}

async function setupSocket() {
    socket = new WebSocket('ws://localhost:8765');
    socket.onopen = function() {
        socket.send('ready');
        console.log("socket ready.");
    };

    socket.onmessage = function(event) {
        const modelData = JSON.parse(event.data);
        // If a mesh already exists, remove it
        if (mesh) {
            scene.remove(mesh);
        }

        // Create and add new mesh
        mesh = createMeshFromModelData(modelData);
        scene.add(mesh);

        // Send an acknowledgment and close the socket
        socket.send('ACK');
        console.log("ACK sent.");
        socket.close();
    };

    socket.onclose = function() {
        console.log("socket closed. retrying.");
        // Try to set up a new socket connection after a delay
        setTimeout(setupSocket, 1000);
    };

    socket.addEventListener('close', function(event) {
        console.log('Lost connection to server. ');
    });
}

function createMeshFromModelData(modelData) {
    const geometry = new THREE.BufferGeometry();

    // Flatten the nested vertices and faces arrays
    const vertices = new Float32Array(modelData.vertices.flat());
    const faces = new Uint32Array(modelData.faces.flat());
    console.log(vertices);
    console.log(faces);

    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    geometry.setIndex(new THREE.BufferAttribute(faces, 1));

    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });
    const mesh = new THREE.Mesh(geometry, material);

    return mesh;
}

function animate() {
    requestAnimationFrame(animate);

    if(mesh) {
        mesh.rotation.x += 0.01;
        mesh.rotation.y += 0.01;
    }

    renderer.render(scene, camera);
}

init();