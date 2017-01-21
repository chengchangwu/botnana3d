var THREE = require("three");
var v1 = new THREE.Vector3();
var v2 = new THREE.Vector3();
var start = console.time();
for(i=0; i< 10000; i++) {
    var s = v1.multiply(v2);
    v1 = v1.multiplyScalar(s);
}
console.timeEnd(start);
