const fs = require('fs');
const text = fs.readFileSync('loc.txt', 'utf8');
const lines = text.split('\n').filter(Boolean);
const result = lines
  .map(line => {
    const lastColon = line.lastIndexOf(':');
    if (lastColon === -1) return null;
    const file = line.substring(0, lastColon);
    const num = parseInt(line.substring(lastColon + 1), 10);
    return { file, num };
  })
  .filter(item => item && item.file.endsWith('.tsx'))
  .sort((a, b) => b.num - a.num)
  .slice(0, 30);
console.log(result);
