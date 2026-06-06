import express from "express";
import path from "path";
import { createServer as createViteServer } from "vite";
import { GoogleGenAI } from "@google/genai";
import { handleSdkworkChatLocalApiRequest } from "./local-api";

const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });

async function startServer() {
  const app = express();
  const PORT = 3000;

  app.use(express.json());
  app.use((req, res, next) => {
    handleSdkworkChatLocalApiRequest(req, res, req.path)
      .then((handled) => {
        if (!handled) {
          next();
        }
      })
      .catch(next);
  });

  // AI Document Agent API
  app.post("/api/agent/doc", async (req, res) => {
    try {
      const { action, content, context, instruction } = req.body;

      let prompt = "";

      if (action === "rewrite") {
        prompt = `You are an expert document editor agent. Rewrite the following text to make it more professional, clear, and well-structured. Return ONLY the rewritten text without markdown fences, or extra commentary.\n\nText: ${content}`;
      } else if (action === "summarize") {
        prompt = `You are an expert document editor agent. Summarize the following text into key bullet points. Return ONLY the summarized points without markdown fences, or extra commentary.\n\nText: ${content}`;
      } else if (action === "expand") {
        prompt = `You are an expert document editor agent. Expand the following text, providing more details, examples, and context. Return ONLY the expanded text without markdown fences, or extra commentary.\n\nText: ${content}`;
      } else if (action === "translate") {
        prompt = `You are an expert document editor agent. Translate the following text into fluent English (if it's Chinese) or Chinese (if it's English). Return ONLY the translated text without markdown fences, or extra commentary.\n\nText: ${content}`;
      } else if (action === "instruct") {
        prompt = `You are an expert document editor agent. You are currently editing a document. 
        Context (the surrounding text or previous text): 
        ${context || "None"}
        
        Instruction from user:
        ${instruction}
        
        Current selected text (if any):
        ${content || "None"}
        
        Follow the user's instruction and generate the necessary Markdown text. Return ONLY the modified or generated text without any conversational filler or enclosing markdown fences (like \`\`\`markdown).`;
      } else {
        return res.status(400).json({ error: "Invalid action" });
      }

      const response = await ai.models.generateContent({
        model: "gemini-2.5-flash",
        contents: prompt,
      });

      res.json({ result: response.text });
    } catch (error) {
      console.error("AI Error:", error);
      res.status(500).json({ error: "AI processing failed" });
    }
  });

  app.post("/api/agent/icon", async (req, res) => {
    try {
      const { description } = req.body;
      const prompt = `A highly stylized, minimalist icon for a knowledge base about: ${description}. White or clear background, app icon style, modern, sleek, simple.`;

      const response = await ai.models.generateContent({
        model: "gemini-2.5-flash-image",
        contents: prompt,
      });

      let imageUrl = null;
      if (response?.candidates?.[0]?.content?.parts) {
        for (const part of response.candidates[0].content.parts) {
          if (part.inlineData) {
            imageUrl = `data:${part.inlineData.mimeType};base64,${part.inlineData.data}`;
            break;
          }
        }
      }

      if (imageUrl) {
        res.json({ result: imageUrl });
      } else {
        res.status(500).json({ error: "No image generated" });
      }
    } catch (error) {
      console.error("AI Icon Generating Error:", error);
      res.status(500).json({ error: "AI icon generation failed" });
    }
  });

  // Server-side module configuration center simulation
  app.get("/api/config/modules", async (req, res) => {
    // In a real app, this would check the user's role or plan from req.headers or session
    // and return the appropriate modules "千人千面"
    res.json({
      modules: [
        "chat",
        "workspace",
        "orders",
        "shop",
        "calendar",
        "notary",
        "knowledge",
        "enterprise",
        "devices",
        "community",
        "voice",
        "agent",
        "course",
        "contacts",
        "favorites",
      ],
    });
  });

  // Vite middleware for development
  if (process.env.NODE_ENV !== "production") {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: "spa",
    });
    app.use(vite.middlewares);
  } else {
    const distPath = path.join(process.cwd(), "dist");
    app.use(express.static(distPath));
    app.get("*", (req, res) => {
      res.sendFile(path.join(distPath, "index.html"));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();
