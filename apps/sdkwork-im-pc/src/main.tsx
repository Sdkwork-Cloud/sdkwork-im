import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import App from './App.tsx';
import { bootstrapHostAppearanceBridge } from '@sdkwork/im-pc-commons';
import { bootstrapImDrivePcIntegration } from './bootstrap/drivePc';
import { bootstrapImKnowledgebasePcIntegration } from './bootstrap/knowledgebasePc';
import { bootstrapImNotaryPcIntegration } from './bootstrap/notaryPc';
import './index.css';

bootstrapHostAppearanceBridge();
bootstrapImNotaryPcIntegration();
bootstrapImDrivePcIntegration();
bootstrapImKnowledgebasePcIntegration();

const root = createRoot(document.getElementById('root')!);
if (import.meta.env.DEV) {
  root.render(<App />);
} else {
  root.render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
