var THREE = require("three");
var v1 = new THREE.Vector3();
var v2 = new THREE.Vector3();
var start = process.hrtime();
for(i=0; i< 1000; i++) {
    var s = v1.multiply(v2);
    v1 = v1.multiplyScalar(s);
}
var elapsed = process.hrtime(start);
console.log("elapsed: " + elapsed[0] + "s " + elapsed[1] + "ns")
