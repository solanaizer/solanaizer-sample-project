const inputChunks = [];

process.stdin.resume();
process.stdin.setEncoding('utf8');

process.stdin.on('data', function(chunk) {
    inputChunks.push(chunk);
});

process.stdin.on('end', function() {
    const inputJSON = inputChunks.join();
    const inputData = JSON.parse(inputJSON);
    // Convert inputData to Markdown table
    const markdownTable = convertToMarkdownTable(inputData);
    console.log(markdownTable);
});

function convertToMarkdownTable(data) {
    let table = "| Severity | Message | Error Code | Lines |\n";
    table += "|----------|---------|------------|-------|\n";
    data.forEach(item => {
        table += `| ${item.severity} | ${item.message} | ${item.errorCode} | ${item.lines.join(', ')} |\n`;
    });
    return table;
}
