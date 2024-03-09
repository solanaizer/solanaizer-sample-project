const inputChunks = [];

process.stdin.resume();
process.stdin.setEncoding('utf8');

process.stdin.on('data', function(chunk) {
    inputChunks.push(chunk);
});

process.stdin.on('end', function() {
    const inputJSON = inputChunks.join();
    const inputData = JSON.parse(inputJSON);
    // console.table(inputData);
});

return "TESTTESTETST"
