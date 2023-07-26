import * as THREE from './node_modules/three/build/three.module.js';
import { OrbitControls } from './node_modules/three/examples/jsm/controls/OrbitControls.js';

let scene, camera, renderer, mesh, socket, controls;


function updateInfo() {
    if(mesh) {
        const tris = mesh.geometry.index.count / 3;
        const infoText = `Triangles: ${tris}<br>Press M to toggle wireframe`;
        document.getElementById('info').innerHTML = infoText;
    }
}

async function init() {
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.z = 5;
    
    renderer = new THREE.WebGLRenderer();
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);

    // Adding bindings
    document.addEventListener('keydown', function(event) {
        if(event.key === 'm' || event.key === 'M') {
            if(mesh) {
                mesh.material.wireframe = !mesh.material.wireframe;
            }
        }
    });

    // Ambient light and directional lights
    var ambientLight = new THREE.AmbientLight(0xffffff, 0.2);
    scene.add(ambientLight);
    var directionalLight1 = new THREE.DirectionalLight(0xffffff, 0.5);
    directionalLight1.position.set(5, 5, 5);
    directionalLight1.target.position.set(0, 0, 0);
    scene.add(directionalLight1);
    scene.add(directionalLight1.target);
    var directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.5);
    directionalLight2.position.set(-5, -5, 5);
    directionalLight2.target.position.set(0, 0, 0);
    scene.add(directionalLight2);
    scene.add(directionalLight2.target);

    // Mouse drag to rotate the mesh
    controls = new OrbitControls(camera, renderer.domElement);

    await setupSocket();
    
    if(mesh) scene.add(mesh);

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
    console.log("Received new model!");

    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    geometry.setIndex(new THREE.BufferAttribute(faces, 1));
    geometry.computeVertexNormals();

    const material = new THREE.MeshPhongMaterial({ color: 0xaaaaaa, wireframe: true });
    const mesh = new THREE.Mesh(geometry, material);

    return mesh;
} 

function animate() {
    requestAnimationFrame(animate);

    // Update the controls before re-rendering the scene
    controls.update();

    updateInfo();  // Update the info text

    renderer.render(scene, camera);
}

init();