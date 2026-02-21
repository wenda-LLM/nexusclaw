import { useState, useEffect } from 'react';
import { RefreshCw, Radio } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { PresenceEntry } from '@/types';

export function InstancesPage() {
  const [loading, setLoading] = useState(false);
  const [instances, setInstances] = useState<PresenceEntry[]>([]);

  const loadInstances = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<PresenceEntry[]>('system-presence', {});
      setInstances(result || []);
    } catch (e) {
      console.error('Failed to load instances:', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadInstances();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('instances.title')}</h1>
        <Button variant="secondary" onClick={loadInstances} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <Card>
        {instances.length === 0 ? (
          <div className="text-center py-12 text-[var(--text-muted)]">
            <Radio className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>{t('common.noData')}</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-[var(--border-base)]">
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('instances.host')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('instances.mode')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('instances.platform')}</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">角色</th>
                  <th className="text-left py-3 px-4 text-sm font-medium text-[var(--text-secondary)]">{t('instances.lastActive')}</th>
                </tr>
              </thead>
              <tbody>
                {instances.map((entry, i) => (
                  <tr key={i} className="border-b border-[var(--border-base)] last:border-0">
                    <td className="py-3 px-4 text-[var(--text-primary)]">{entry.host || '未知'}</td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">{entry.mode || '未知'}</td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">{entry.platform || '-'}</td>
                    <td className="py-3 px-4">
                      <div className="flex gap-1 flex-wrap">
                        {entry.roles?.map((role) => (
                          <span key={role} className="px-2 py-0.5 text-xs bg-[var(--brand-500)] text-white rounded">
                            {role}
                          </span>
                        ))}
                      </div>
                    </td>
                    <td className="py-3 px-4 text-[var(--text-secondary)]">
                      {entry.updatedAt ? new Date(entry.updatedAt).toLocaleString() : '-'}
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