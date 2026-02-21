import { useState, useEffect } from 'react';
import { RefreshCw, Trash2 } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { SessionsListResult, GatewaySessionRow } from '@/types';

export function SessionsPage() {
  const [loading, setLoading] = useState(false);
  const [sessions, setSessions] = useState<SessionsListResult | null>(null);

  const loadSessions = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<SessionsListResult>('sessions.list', { limit: 100 });
      setSessions(result);
    } catch (e) {
      console.error('Failed to load sessions:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (key: string) => {
    if (!confirm('确定删除此会话？')) return;
    try {
      await gateway.request('sessions.delete', { key });
      loadSessions();
    } catch (e) {
      console.error('Failed to delete session:', e);
    }
  };

  useEffect(() => {
    loadSessions();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('sessions.title')}</h1>
        <Button variant="secondary" onClick={loadSessions} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <Card>
        {!sessions?.sessions.length ? (
          <div className="text-center py-12 text-[var(--text-muted)]">
            <p>{t('common.noData')}</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-[var(--border-base)]">
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('sessions.key')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('sessions.label')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('sessions.type')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('sessions.tokens')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('sessions.updated')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('sessions.actions')}</th>
                </tr>
              </thead>
              <tbody>
                {sessions.sessions.map((row: GatewaySessionRow) => (
                  <tr key={row.key} className="border-b border-[var(--border-base)] last:border-0">
                    <td className="py-3 px-4 text-[var(--text-primary)] font-mono text-sm">{row.key}</td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">{row.label || '-'}</td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">{row.kind}</td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">
                      {(row.inputTokens || 0) + (row.outputTokens || 0)}
                    </td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">
                      {row.updatedAt ? new Date(row.updatedAt).toLocaleString() : '-'}
                    </td>
                    <td className="py-3 px-4">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDelete(row.key)}
                        className="text-[var(--color-danger)]"
                      >
                        <Trash2 className="w-4 h-4" />
                      </Button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </Card>
    </div>
  );
}