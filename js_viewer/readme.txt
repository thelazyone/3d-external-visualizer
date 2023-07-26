before running this, call

npm install three three-orbit-controls   

to deploy: 

npm install -g parcel-bundler
parcel ./index.html

This will generate a "dist" folder. 
Note that on Windows there might be a privileges problem: run Set-ExecutionPolicy RemoteSigned to fix it.

The "dist" folder contains all the necessary. 
However, you have to remove the / in the script line: 

<script src="/viewer.6f8112d9.js"></script> -> <script src="viewer.6f8112d9.js"></script>


