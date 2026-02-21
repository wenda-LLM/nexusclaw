import { useState, useEffect } from 'react';
import { RefreshCw, BarChart3 } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { UsageSummary } from '@/types';

export function UsagePage() {
  const [loading, setLoading] = useState(false);
  const [summary, setSummary] = useState<UsageSummary | null>(null);
  const [startDate, setStartDate] = useState(() => {
    const d = new Date();
    d.setDate(d.getDate() - 30);
    return d.toISOString().split('T')[0];
  });
  const [endDate, setEndDate] = useState(() => new Date().toISOString().split('T')[0]);

  const loadUsage = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<UsageSummary>('usage.summary', { startDate, endDate });
      setSummary(result);
    } catch (e) {
      console.error('Failed to load usage:', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadUsage();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('usage.title')}</h1>
        <Button variant="secondary" onClick={loadUsage} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <div className="flex gap-4 mb-6">
        <Input
          type="date"
          value={startDate}
          onChange={(e) => setStartDate(e.target.value)}
          label="开始日期"
        />
        <Input
          type="date"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
          label="结束日期"
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <div className="flex items-center gap-3">
            <div className="p-3 rounded-lg bg-[var(--color-primary)]/10 text-[var(--color-primary)]">
              <BarChart3 className="w-6 h-6" />
            </div>
            <div>
              <p className="text-sm text-[var(--text-secondary)]">{t('usage.totalTokens')}</p>
              <p className="text-2xl font-bold text-[var(--text-primary)]">
                {summary?.totalTokens.toLocaleString() || 0}
              </p>
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center gap-3">
            <div className="p-3 rounded-lg bg-[var(--color-success)]/10 text-[var(--color-success)]">
              <BarChart3 className="w-6 h-6" />
            </div>
            <div>
              <p className="text-sm text-[var(--text-secondary)]">{t('usage.totalCost')}</p>
              <p className="text-2xl font-bold text-[var(--text-primary)]">
                ${summary?.totalCost.toFixed(4) || 0}
              </p>
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center gap-3">
            <div className="p-3 rounded-lg bg-[var(--color-warning)]/10 text-[var(--color-warning)]">
              <BarChart3 className="w-6 h-6" />
            </div>
            <div>
              <p className="text-sm text-[var(--text-secondary)]">会话数</p>
              <p className="text-2xl font-bold text-[var(--text-primary)]">
                {summary?.sessionCount || 0}
              </p>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}