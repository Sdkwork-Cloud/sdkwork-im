import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import App from './App.tsx';
import { bootstrapImDrivePcIntegration } from './bootstrap/drivePc';
import { bootstrapImKnowledgebasePcIntegration } from './bootstrap/knowledgebasePc';
import { bootstrapImNotaryPcIntegration } from './bootstrap/notaryPc';
import './index.css';

bootstrapImNotaryPcIntegration();
bootstrapImDrivePcIntegration();
bootstrapImKnowledgebasePcIntegration();

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
