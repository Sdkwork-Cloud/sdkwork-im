const fs = require('fs');
const path = require('path');
const execSync = require('child_process').execSync;

const updateFile = (filePath, replacements) => {
  let content = fs.readFileSync(filePath, 'utf8');
  let original = content;
  for (const [from, to] of replacements) {
    if (typeof from === 'string') {
        content = content.replace(from, to);
    } else {
        content = content.replace(from, to);
    }
  }
  if (original !== content) {
    fs.writeFileSync(filePath, content);
    console.log('Updated: ' + filePath);
  }
}

// 1. Update calendar package
const calDir = 'packages/sdkwork-im-pc-calendar/src';
execSync(`find ${calDir} -type f -name '*.tsx'`).toString().trim().split('\n').filter(Boolean).forEach(file => {
  updateFile(file, [
    [/import \{([^}]*)\} from '@sdkwork\/im-pc-core'/g, "import {$1} from '../services/CalendarService'"]
  ]);
});

// Notary UI now lives in sdkwork-notary/apps/sdkwork-notary-pc (see bootstrapNotaryPcForIm).

// 2. Update workspace
const wsDir = 'packages/sdkwork-im-pc-workspace/src';
execSync(`find ${wsDir} -type f -name '*.tsx'`).toString().trim().split('\n').filter(Boolean).forEach(file => {
  updateFile(file, [
    [/import \{([^}]*)\} from '@sdkwork\/im-pc-core'/g, "import {$1} from './services/WorkspaceService'"]
  ]);
});

// 4. Update orders
const ordersDir = 'packages/sdkwork-im-pc-orders/src';
execSync(`find ${ordersDir} -type f -name '*.tsx'`).toString().trim().split('\n').filter(Boolean).forEach(file => {
  updateFile(file, [
    [/import \{([^}]*)\} from '@sdkwork\/im-pc-core'/g, "import {$1} from './services/OrdersService'"]
  ]);
});

// 5. Update drive
// Drive UI now lives in sdkwork-drive/apps/sdkwork-drive-pc (see bootstrapDrivePcForIm).
execSync(`find ${driveDir} -type f -name '*.tsx'`).toString().trim().split('\n').filter(Boolean).forEach(file => {
  updateFile(file, [
    [/import \{([^}]*)\} from '@sdkwork\/im-pc-core'/g, "import {$1} from './services/DriveService'"]
  ]);
});

// 6. Update mail
const mailDir = 'packages/sdkwork-im-pc-mail/src';
execSync(`find ${mailDir} -type f -name '*.tsx'`).toString().trim().split('\n').filter(Boolean).forEach(file => {
  updateFile(file, [
    [/import \{([^}]*)\} from '@sdkwork\/im-pc-core'/g, "import {$1} from './services/MailService'"]
  ]);
});

// 7. Update chat
const chatDir = 'packages/sdkwork-im-pc-chat/src';
execSync(`find ${chatDir} -type f -name '*.tsx'`).toString().trim().split('\n').filter(Boolean).forEach(file => {
  let content = fs.readFileSync(file, 'utf8');
  let original = content;
  
  // Replace references
  content = content.replace(/import\s+\{([^}]*)\}\s+from\s+'@sdkwork\/im-pc-core';/g, (match, importsStr) => {
      let imports = importsStr.split(',').map(s => s.trim()).filter(Boolean);
      let contactImports = [];
      let groupImports = [];
      let agentImports = [];
      let favoriteImports = [];
      let chatImports = [];
      let musicImports = [];
      
      imports.forEach(imp => {
          if (imp === 'contactService' || imp === 'OrgDepartment' || imp === 'FriendRequest' || imp === 'ContactTag' || imp === 'type OrgDepartment' || imp === 'type FriendRequest' || imp === 'type ContactTag') {
              contactImports.push(imp.replace('type ', ''));
          } else if (imp === 'groupService') {
              groupImports.push(imp);
          } else if (imp === 'agentService' || imp === 'AgentConfig' || imp === 'type AgentConfig') {
              agentImports.push(imp.replace('type ', ''));
          } else if (imp === 'favoriteService' || imp === 'FavoriteItem' || imp === 'type FavoriteItem') {
              favoriteImports.push(imp.replace('type ', ''));
          } else if (imp === 'chatService') {
              chatImports.push(imp);
          } else if (imp === 'musicService' || imp === 'PlayerState' || imp === 'MusicTrack') {
              musicImports.push(imp);
          }
      });
      
      let relativePath = file.split('/').length === 5 ? '../services' : '../../services';
      if (file.includes('pages/')) relativePath = '../services';
      if (file.includes('components/')) relativePath = '../services';
      
      let lines = [];
      if (contactImports.length) lines.push(`import { ${contactImports.join(', ')} } from '${relativePath}/ContactService';`);
      if (groupImports.length) lines.push(`import { ${groupImports.join(', ')} } from '${relativePath}/GroupService';`);
      if (agentImports.length) lines.push(`import { ${agentImports.join(', ')} } from '${relativePath}/AgentService';`);
      if (favoriteImports.length) lines.push(`import { ${favoriteImports.join(', ')} } from '${relativePath}/FavoriteService';`);
      if (chatImports.length) lines.push(`import { ${chatImports.join(', ')} } from '${relativePath}/ChatService';`);
      if (musicImports.length) lines.push(`import { ${musicImports.join(', ')} } from '${relativePath}/MusicService';`);
      
      return lines.join('\n');
  });

  if (original !== content) {
    fs.writeFileSync(file, content);
    console.log('Updated: ' + file);
  }
});
