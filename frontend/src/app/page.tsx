"use client";

import { useState } from "react";
import { Search, TrendingUp, BarChart3, PieChart, ArrowRight, Loader2 } from "lucide-react";
import Link from "next/link";

export default function Home() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSearch = async () => {
    if (!query.trim()) return;
    setLoading(true);
    try {
      const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/search?q=${query}`);
      const data = await res.json();
      setResults(data.results || []);
    } catch (e) {
      setResults([]);
    }
    setLoading(false);
  };

  return (
    <main className="min-h-screen">
      {/* Hero */}
      <section className="px-6 pt-32 pb-20 max-w-4xl mx-auto text-center">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight mb-4">
          <span className="gradient-text">Valytics</span>
        </h1>
        <p className="text-lg text-gray-400 max-w-2xl mx-auto mb-8">
          Institutional-grade financial analysis. Multi-stage DCF valuation, Monte Carlo simulation, 
          ratio analysis, comparable companies, and portfolio risk decomposition.
        </p>

        {/* Search */}
        <div className="max-w-xl mx-auto mb-12">
          <div className="flex gap-3">
            <div className="flex-1 flex items-center gap-3 bg-surface border border-border rounded-xl px-5 py-4 focus-within:border-accent transition-colors">
              <Search className="w-5 h-5 text-muted" />
              <input
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleSearch()}
                placeholder="Search by ticker or company name (e.g., AAPL, MSFT, Tesla)..."
                className="flex-1 bg-transparent text-white placeholder:text-muted outline-none text-sm"
              />
            </div>
            <button
              onClick={handleSearch}
              disabled={loading}
              className="bg-accent hover:bg-accent-hover disabled:opacity-50 text-white rounded-xl px-6 py-4 font-semibold transition-colors"
            >
              {loading ? <Loader2 className="w-5 h-5 animate-spin" /> : "Search"}
            </button>
          </div>
        </div>

        {/* Search Results */}
        {results.length > 0 && (
          <div className="max-w-xl mx-auto space-y-2 mb-12">
            {results.map((r, i) => (
              <Link
                key={i}
                href={`/company/${r.ticker}`}
                className="flex items-center justify-between glass-card p-4 hover:border-accent/50 transition-colors"
              >
                <div className="text-left">
                  <p className="font-semibold text-white">{r.ticker}</p>
                  <p className="text-sm text-gray-400">{r.name}</p>
                </div>
                <ArrowRight className="w-4 h-4 text-muted" />
              </Link>
            ))}
          </div>
        )}

        {/* Feature Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 max-w-3xl mx-auto">
          {[
            { icon: BarChart3, title: "Company Analysis", desc: "Full financials, ratios, DCF valuation, comparable companies, and key insights." },
            { icon: PieChart, title: "Portfolio Analysis", desc: "Returns, risk metrics, factor decomposition, efficient frontier, and scenario analysis." },
            { icon: TrendingUp, title: "Monte Carlo Simulation", desc: "10,000 iterations of DCF assumptions to generate probability-weighted fair value ranges." },
          ].map((f, i) => (
            <div key={i} className="glass-card p-6 text-center">
              <f.icon className="w-8 h-8 text-accent mx-auto mb-3" />
              <h3 className="font-semibold text-white mb-2">{f.title}</h3>
              <p className="text-sm text-gray-400">{f.desc}</p>
            </div>
          ))}
        </div>
      </section>
    </main>
  );
}
