// server.js
const express = require('express');
const path = require('path');
const app = express();
const port = 8000;

let model = null;  // store the most recently received 3D model here
console.log("Script Started.");

app.use(express.json());

app.post('/model', (req, res) => {
    model = req.body;
    console.log(model);
    res.send('Received model');
});

app.get('/model', (req, res) => {
    console.log("Model Received");
    if (model === null) {
        res.status(404).send('No model has been received yet');
    } else {
        res.json(model);
    }
});

app.use(express.static(path.join(__dirname, 'public')));

app.listen(port, () => {
    console.log(`Server listening at http://localhost:${port}`);
});