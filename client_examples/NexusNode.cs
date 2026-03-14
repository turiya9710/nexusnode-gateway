using System;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;

namespace NexusNodeExample
{
    // The structure returned by the NexusNode Gateway
    public class NexusResponse
    {
        public string provider { get; set; }
        public string model_used { get; set; }
        public JsonElement response_payload { get; set; }
    }

    public class NexusClient
    {
        private readonly HttpClient _httpClient;
        private readonly string _gatewayUrl;

        // Initialize with your deployed Cloudflare Worker URL
        public NexusClient(string gatewayUrl = "https://your-nexusnode-mvp.workers.dev")
        {
            _httpClient = new HttpClient();
            _gatewayUrl = gatewayUrl;
        }

        public async Task<string> ChatAsync(string prompt)
        {
            // The unified payload expected by NexusNode MVP
            var payload = new
            {
                model_tier = "smart", // MVP doesn't use this yet, but it's defined
                messages = new[]
                {
                    new { role = "user", content = prompt }
                }
            };

            var jsonPayload = JsonSerializer.Serialize(payload);
            var content = new StringContent(jsonPayload, Encoding.UTF8, "application/json");

            try
            {
                // Note: Auth to the gateway (e.g., Bearer token) should be added here
                var response = await _httpClient.PostAsync(_gatewayUrl, content);
                response.EnsureSuccessStatusCode();

                var jsonResponse = await response.Content.ReadAsStringAsync();
                var nexusResult = JsonSerializer.Deserialize<NexusResponse>(jsonResponse);

                // For MVP, we just print which provider was used and return the raw text 
                // extracted from the specific payload structure (assuming OpenAI/Anthropic format)
                Console.WriteLine($"[NexusNode] Served by: {nexusResult.provider} using model: {nexusResult.model_used}");

                // Example extraction from OpenAI payload structure
                if (nexusResult.provider == "openai")
                {
                    return nexusResult.response_payload.GetProperty("choices")[0].GetProperty("message").GetProperty("content").GetString();
                }
                // Example extraction from Anthropic payload structure
                else if (nexusResult.provider == "anthropic")
                {
                    return nexusResult.response_payload.GetProperty("content")[0].GetProperty("text").GetString();
                }

                return "Unknown provider structure.";
            }
            catch (Exception ex)
            {
                return $"Error: {ex.Message}";
            }
        }
    }

    class Program
    {
        static async Task Main(string[] args)
        {
            Console.WriteLine("--- Starting NexusNode Client ---");
            var client = new NexusClient();
            var response = await client.ChatAsync("What is the main advantage of a WASM-native LLM gateway?");
            Console.WriteLine($"\nFinal Response:\n{response}");
        }
    }
}