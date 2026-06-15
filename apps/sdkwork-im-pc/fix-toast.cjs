const fs = require('fs');
const path = require('path');

function walk(dir) {
  let results = [];
  const list = fs.readdirSync(dir);
  list.forEach(function(file) {
    file = path.join(dir, file);
    const stat = fs.statSync(file);
    if (stat && stat.isDirectory()) {
      if (!file.includes('node_modules') && !file.includes('dist')) {
        results = results.concat(walk(file));
      }
    } else if (file.endsWith('.tsx') || file.endsWith('.ts')) {
      results.push(file);
    }
  });
  return results;
}

const files = walk('./packages');
let changed = 0;
files.forEach(file => {
  const content = fs.readFileSync(file, 'utf8');
  if (content.includes("'info'") || content.includes('"info"')) {
    const lines = content.split('\n');
    let modified = false;
    const newLines = lines.map(line => {
      // Don't replace the toast parameter definition
      if (line.includes('toast(') && line.includes('info') && !line.includes("type: 'info' | 'success'")) {
        modified = true;
        return line.replace(/'info'/g, "'success'").replace(/"info"/g, '"success"');
      }
      return line;
    });
    if (modified) {
      fs.writeFileSync(file, newLines.join('\n'));
      changed++;
    }
  }
});
console.log(`Changed ${changed} files.`);
