"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import { ArrowLeft, Loader2, TrendingUp, TrendingDown, AlertTriangle, CheckCircle } from "lucide-react";
import Link from "next/link";
import {
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer,
  BarChart, Bar, Legend,
} from "recharts";

export default function CompanyPage() {
  const { ticker } = useParams();
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState("overview");

  useEffect(() => {
    async function fetchData() {
      try {
        const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/company/${ticker}`);
        const json = await res.json();
        setData(json);
      } catch (e) {
        console.error(e);
      }
      setLoading(false);
    }
    fetchData();
  }, [ticker]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loader2 className="w-8 h-8 text-accent animate-spin" />
      </div>
    );
  }

  if (!data || data.error) {
    return (
      <div className="min-h-screen flex flex-col items-center justify-center gap-4">
        <p className="text-xl text-gray-400">Company not found</p>
        <Link href="/" className="text-accent hover:underline">Back to search</Link>
      </div>
    );
  }

  const company = data.company;
  const ratios = data.ratios;
  const dcf = data.dcf_valuation;
  const comparable = data.comparable_analysis;
  const insights = data.insights || [];

  const tabs = ["overview", "ratios", "dcf", "comparable", "insights"];

  return (
    <main className="min-h-screen px-6 py-8 max-w-6xl mx-auto">
      {/* Header */}
      <div className="flex items-center gap-4 mb-8">
        <Link href="/" className="text-muted hover:text-white transition-colors">
          <ArrowLeft className="w-5 h-5" />
        </Link>
        <div>
          <h1 className="text-2xl font-bold">{company.ticker}</h1>
          <p className="text-gray-400">{company.name}</p>
        </div>
        <div className="ml-auto flex gap-2">
          <span className="text-xs bg-surface border border-border px-3 py-1 rounded-full text-gray-400">
            {company.sector}
          </span>
          <span className="text-xs bg-surface border border-border px-3 py-1 rounded-full text-gray-400">
            {company.exchange}
          </span>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 mb-8 overflow-x-auto">
        {tabs.map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`px-4 py-2 rounded-lg text-sm font-medium capitalize transition-colors ${
              activeTab === tab
                ? "bg-accent text-white"
                : "bg-surface text-gray-400 hover:text-white"
            }`}
          >
            {tab}
          </button>
        ))}
      </div>

      {/* Overview Tab */}
      {activeTab === "overview" && (
        <div className="space-y-8">
          {/* Key Metrics */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {[
              { label: "Market Cap", value: company.market_cap ? `$${(company.market_cap / 1e9).toFixed(1)}B` : "N/A" },
              { label: "P/E Ratio", value: ratios?.valuation_ratios?.pe_ratio?.toFixed(1) || "N/A" },
              { label: "ROE", value: ratios?.profitability?.roe?.slice(-1)[0]?.value ? `${(ratios.profitability.roe.slice(-1)[0].value * 100).toFixed(1)}%` : "N/A" },
              { label: "Net Margin", value: ratios?.profitability?.net_margin?.slice(-1)[0]?.value ? `${(ratios.profitability.net_margin.slice(-1)[0].value * 100).toFixed(1)}%` : "N/A" },
            ].map((m, i) => (
              <div key={i} className="glass-card p-4 text-center">
                <p className="text-2xl font-bold text-white">{m.value}</p>
                <p className="text-xs text-gray-400 mt-1">{m.label}</p>
              </div>
            ))}
          </div>

          {/* Revenue & Profit Chart */}
          {data.financials?.income_statement?.length > 0 && (
            <div className="glass-card p-6">
              <h3 className="text-lg font-semibold mb-4">Revenue & Net Income</h3>
              <ResponsiveContainer width="100%" height={300}>
                <BarChart data={data.financials.income_statement.map((is: any) => ({
                  year: is.fiscal_year,
                  Revenue: (is.revenue / 1e9).toFixed(1),
                  "Net Income": (is.net_income / 1e9).toFixed(1),
                }))}>
                  <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.05)" />
                  <XAxis dataKey="year" stroke="#6b6b6b" />
                  <YAxis stroke="#6b6b6b" />
                  <Tooltip contentStyle={{ background: "#111", border: "1px solid rgba(255,255,255,0.1)", borderRadius: "8px" }} />
                  <Legend />
                  <Bar dataKey="Revenue" fill="#dc2626" radius={[4, 4, 0, 0]} />
                  <Bar dataKey="Net Income" fill="#d4a017" radius={[4, 4, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </div>
          )}

          {/* Description */}
          {company.description && (
            <div className="glass-card p-6">
              <h3 className="text-lg font-semibold mb-2">About</h3>
              <p className="text-sm text-gray-400 leading-relaxed">{company.description}</p>
            </div>
          )}
        </div>
      )}

      {/* Ratios Tab */}
      {activeTab === "ratios" && ratios && (
        <div className="space-y-6">
          {/* Profitability */}
          <div className="glass-card p-6">
            <h3 className="text-lg font-semibold mb-4">Profitability Ratios</h3>
            {ratios.profitability?.net_margin?.length > 0 && (
              <ResponsiveContainer width="100%" height={250}>
                <LineChart data={ratios.profitability.net_margin.map((m: any) => ({
                  year: m.year,
                  "Net Margin": (m.value * 100).toFixed(1),
                  ROE: (ratios.profitability.roe.find((r: any) => r.year === m.year)?.value || 0) * 100,
                }))}>
                  <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.05)" />
                  <XAxis dataKey="year" stroke="#6b6b6b" />
                  <YAxis stroke="#6b6b6b" />
                  <Tooltip contentStyle={{ background: "#111", border: "1px solid rgba(255,255,255,0.1)", borderRadius: "8px" }} />
                  <Legend />
                  <Line type="monotone" dataKey="Net Margin" stroke="#dc2626" strokeWidth={2} dot={{ fill: "#dc2626" }} />
                  <Line type="monotone" dataKey="ROE" stroke="#d4a017" strokeWidth={2} dot={{ fill: "#d4a017" }} />
                </LineChart>
              </ResponsiveContainer>
            )}
          </div>

          {/* Liquidity & Leverage */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="glass-card p-6">
              <h3 className="text-lg font-semibold mb-3">Liquidity</h3>
              <div className="space-y-3">
                {ratios.liquidity?.current_ratio?.slice(-1)[0] && (
                  <div className="flex justify-between">
                    <span className="text-gray-400">Current Ratio</span>
                    <span className="font-mono text-white">{ratios.liquidity.current_ratio.slice(-1)[0].value.toFixed(2)}</span>
                  </div>
                )}
                {ratios.liquidity?.quick_ratio?.slice(-1)[0] && (
                  <div className="flex justify-between">
                    <span className="text-gray-400">Quick Ratio</span>
                    <span className="font-mono text-white">{ratios.liquidity.quick_ratio.slice(-1)[0].value.toFixed(2)}</span>
                  </div>
                )}
                {ratios.liquidity?.cash_ratio?.slice(-1)[0] && (
                  <div className="flex justify-between">
                    <span className="text-gray-400">Cash Ratio</span>
                    <span className="font-mono text-white">{ratios.liquidity.cash_ratio.slice(-1)[0].value.toFixed(2)}</span>
                  </div>
                )}
              </div>
            </div>

            <div className="glass-card p-6">
              <h3 className="text-lg font-semibold mb-3">Leverage</h3>
              <div className="space-y-3">
                {ratios.leverage?.debt_to_equity?.slice(-1)[0] && (
                  <div className="flex justify-between">
                    <span className="text-gray-400">Debt/Equity</span>
                    <span className="font-mono text-white">{ratios.leverage.debt_to_equity.slice(-1)[0].value.toFixed(2)}</span>
                  </div>
                )}
                {ratios.leverage?.debt_to_ebitda?.slice(-1)[0] && (
                  <div className="flex justify-between">
                    <span className="text-gray-400">Debt/EBITDA</span>
                    <span className="font-mono text-white">{ratios.leverage.debt_to_ebitda.slice(-1)[0].value.toFixed(2)}</span>
                  </div>
                )}
                {ratios.leverage?.interest_coverage?.slice(-1)[0] && (
                  <div className="flex justify-between">
                    <span className="text-gray-400">Interest Coverage</span>
                    <span className="font-mono text-white">{ratios.leverage.interest_coverage.slice(-1)[0].value.toFixed(1)}x</span>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* DCF Tab */}
      {activeTab === "dcf" && dcf && (
        <div className="space-y-6">
          <div className="glass-card p-6">
            <h3 className="text-lg font-semibold mb-4">DCF Valuation</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
              <div className="text-center">
                <p className="text-2xl font-bold text-gold">${dcf.fair_value_per_share?.toFixed(2) || "N/A"}</p>
                <p className="text-xs text-gray-400">Fair Value/Share</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-white">${dcf.current_price?.toFixed(2) || "N/A"}</p>
                <p className="text-xs text-gray-400">Current Price</p>
              </div>
              <div className="text-center">
                <p className={`text-2xl font-bold ${(dcf.upside_pct || 0) > 0 ? "text-green" : "text-red"}`}>
                  {dcf.upside_pct ? `${(dcf.upside_pct * 100).toFixed(1)}%` : "N/A"}
                </p>
                <p className="text-xs text-gray-400">Upside/Downside</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-accent">{(dcf.wacc * 100).toFixed(1)}%</p>
                <p className="text-xs text-gray-400">WACC</p>
              </div>
            </div>
            <div className="bg-background rounded-lg p-4">
              <p className="text-sm text-gray-400">
                <span className="text-white font-semibold">Recommendation:</span>{" "}
                {dcf.recommendation === "undervalued" && <span className="text-green">Undervalued</span>}
                {dcf.recommendation === "overvalued" && <span className="text-red">Overvalued</span>}
                {dcf.recommendation === "fairly_valued" && <span className="text-gold">Fairly Valued</span>}
              </p>
            </div>
          </div>

          {/* Monte Carlo */}
          {dcf.monte_carlo && (
            <div className="glass-card p-6">
              <h3 className="text-lg font-semibold mb-4">Monte Carlo Simulation (10,000 Iterations)</h3>
              <div className="grid grid-cols-2 md:grid-cols-5 gap-3 mb-4">
                <div className="text-center"><p className="text-lg font-bold text-white">${dcf.monte_carlo.mean_fair_value?.toFixed(2)}</p><p className="text-xs text-gray-400">Mean</p></div>
                <div className="text-center"><p className="text-lg font-bold text-white">${dcf.monte_carlo.median_fair_value?.toFixed(2)}</p><p className="text-xs text-gray-400">Median</p></div>
                <div className="text-center"><p className="text-lg font-bold text-white">${dcf.monte_carlo.std_dev?.toFixed(2)}</p><p className="text-xs text-gray-400">Std Dev</p></div>
                <div className="text-center"><p className="text-lg font-bold text-white">${dcf.monte_carlo.percentile_10?.toFixed(2)}</p><p className="text-xs text-gray-400">10th %ile</p></div>
                <div className="text-center"><p className="text-lg font-bold text-white">${dcf.monte_carlo.percentile_90?.toFixed(2)}</p><p className="text-xs text-gray-400">90th %ile</p></div>
              </div>
            </div>
          )}
        </div>
      )}

      {/* Comparable Tab */}
      {activeTab === "comparable" && comparable && (
        <div className="space-y-6">
          <div className="glass-card p-6">
            <h3 className="text-lg font-semibold mb-4">Comparable Company Analysis</h3>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-border">
                    <th className="text-left py-2 text-gray-400 font-medium">Company</th>
                    <th className="text-right py-2 text-gray-400 font-medium">Market Cap</th>
                    <th className="text-right py-2 text-gray-400 font-medium">EV/EBITDA</th>
                    <th className="text-right py-2 text-gray-400 font-medium">P/E</th>
                  </tr>
                </thead>
                <tbody>
                  {comparable.peer_group?.map((peer: any, i: number) => (
                    <tr key={i} className="border-b border-border/50">
                      <td className="py-2">
                        <span className="text-white font-medium">{peer.ticker}</span>
                        <span className="text-gray-500 ml-2 text-xs">{peer.name}</span>
                      </td>
                      <td className="text-right font-mono text-gray-300">${(peer.market_cap / 1e9).toFixed(1)}B</td>
                      <td className="text-right font-mono text-gray-300">{peer.ev_to_ebitda?.toFixed(1)}x</td>
                      <td className="text-right font-mono text-gray-300">{peer.pe_ratio?.toFixed(1)}x</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}

      {/* Insights Tab */}
      {activeTab === "insights" && (
        <div className="space-y-4">
          {insights.map((insight: any, i: number) => (
            <div key={i} className="glass-card p-4 flex items-start gap-3">
              {insight.category === "strength" || insight.category === "opportunity" ? (
                <CheckCircle className="w-5 h-5 text-green mt-0.5" />
              ) : insight.category === "risk" || insight.category === "weakness" ? (
                <AlertTriangle className="w-5 h-5 text-red mt-0.5" />
              ) : (
                <TrendingUp className="w-5 h-5 text-gold mt-0.5" />
              )}
              <div>
                <p className="text-sm text-white">{insight.message}</p>
                <div className="flex gap-2 mt-1">
                  <span className="text-xs text-gray-500 capitalize">{insight.category}</span>
                  <span className={`text-xs px-2 py-0.5 rounded-full ${
                    insight.severity === "high" ? "bg-red/10 text-red" :
                    insight.severity === "medium" ? "bg-gold/10 text-gold" :
                    "bg-green/10 text-green"
                  }`}>{insight.severity}</span>
                </div>
              </div>
            </div>
          ))}
          {insights.length === 0 && (
            <p className="text-gray-400 text-center py-8">No insights available for this company.</p>
          )}
        </div>
      )}
    </main>
  );
}
