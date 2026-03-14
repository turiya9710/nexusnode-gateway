# NexusNode Gateway (MVP v0.1)

**A Rust-powered, WASM-Native Architectural Blueprint for LLM Resilience.**

---

### DISCLAIMER

> **This project is an Architectural MVP intended for educational and inspirational purposes.** It demonstrates a high-performance pattern for LLM routing and failover but is **NOT** yet "Enterprise-Ready" out of the box.
>
> **Use at your own risk.** The author assumes no liability for API costs, service interruptions, data handling, or security vulnerabilities. Serious engineers should review and harden the implementation before any production use.

---

## 1. The Philosophy

NexusNode was born from a simple realization: for a serious engineer building an "App Factory" (multiple AI-driven products), a direct dependency on a single LLM provider is a single point of failure.

This project is a **starting point**. It is meant to inspire developers to take control of their AI infrastructure, moving away from third-party middlemen and toward self-owned, edge-native gateways.

---

## 2. MVP Features

- **Edge-First Performance:** Written in Rust, compiled to WASM for Cloudflare Workers. 0ms cold starts.
- **Resilient Failover:** Built-in logic to handle `429` (Rate Limit) or `500` (Server Error) responses by instantly pivoting from a Primary to a Secondary provider.
- **Unified Interface:** A single endpoint for your apps, abstracting away provider-specific headers and payload formats.
- **Secure by Design:** API keys are managed via Cloudflare Secrets, never exposed to client-side code.

---

## 3. For the Serious Engineer

This MVP is intentionally lean so the "plumbing" is visible. It provides the foundation for you to build:

- **Custom Load Balancing:** Distribute traffic across 10+ providers based on latency.
- **Advanced Telemetry:** Hook into your own ELK or Grafana stack.
- **Cost Guardrails:** Implement per-app token quotas using Cloudflare KV.

---

## 4. Getting Started

### Prerequisites

- Rust & `wasm-pack`
- Cloudflare Wrangler CLI (`npm install -g wrangler`)
- OpenAI & Anthropic API Keys

### Configuration

Run the following commands to add your keys to the Cloudflare environment:

```bash
wrangler secret put OPENAI_API_KEY
wrangler secret put ANTHROPIC_API_KEY
```

### Deployment

```bash
wrangler deploy
```

---

## 5. Integration Example (C#)

The repository includes a `client_examples/` directory featuring a ready-to-use C# client for enterprise application integration.

---

## 6. Roadmap

| Phase | Feature |
|-------|---------|
| **Phase 2** | Integrate Cloudflare D1 for dynamic routing rules |
| **Phase 3** | Implement Semantic Caching with Supabase/pgvector |
| **Phase 4** | Management Dashboard (React/Vite) |

---

## 7. License

**MIT**

*Created as an exploration of the "App Factory" infrastructure model.*