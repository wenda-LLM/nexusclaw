import { useState, useEffect } from 'react';
import { Radio, FileText, Clock, Activity, RefreshCw } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { PresenceEntry, SessionsListResult, CronStatus, HealthSnapshot } from '@/types';

export function OverviewPage() {
  const [loading, setLoading] = useState(false);
  const [instances, setInstances] = useState<PresenceEntry[]>([]);
  const [sessions, setSessions] = useState<SessionsListResult | null>(null);
  const [cronStatus, setCronStatus] = useState<CronStatus | null>(null);
  const [health, setHealth] = useState<HealthSnapshot | null>(null);

  const loadData = async () => {
    setLoading(true);
    try {
      const [presenceRes, sessionsRes, cronRes, healthRes] = await Promise.all([
        gateway.request<PresenceEntry[]>('system-presence', {}),
        gateway.request<SessionsListResult>('sessions.list', { limit: 100 }),
        gateway.request<CronStatus>('cron.status', {}),
        gateway.request<HealthSnapshot>('system.health', {}),
      ]);
      setInstances(presenceRes || []);
      setSessions(sessionsRes);
      setCronStatus(cronRes);
      setHealth(healthRes);
    } catch (e) {
      console.error('Failed to load overview:', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const stats = [
    {
      label: t('overview.instances'),
      value: instances.length,
      icon: Radio,
      color: 'text-green-500',
    },
    {
      label: t('overview.sessions'),
      value: sessions?.count || 0,
      icon: FileText,
      color: 'text-blue-500',
    },
    {
      label: t('overview.cron'),
      value: cronStatus?.enabled ? '启用' : '禁用',
      icon: Clock,
      color: 'text-yellow-500',
    },
    {
      label: t('overview.status'),
      value: health?.status || '未知',
      icon: Activity,
      color: health?.status === 'ok' ? 'text-green-500' : 'text-red-500',
    },
  ];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('overview.title')}</h1>
        <Button variant="secondary" onClick={loadData} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        {stats.map((stat) => (
          <Card key={stat.label} className="flex items-center gap-4">
            <div className={`p-3 rounded-lg bg-[var(--bg-secondary)] ${stat.color}`}>
              <stat.icon className="w-6 h-6" />
            </div>
            <div>
              <p className="text-sm text-[var(--text-secondary)]">{stat.label}</p>
              <p className="text-2xl font-bold text-[var(--text-primary)]">{stat.value}</p>
            </div>
          </Card>
        ))}
      </div>

      {health && (
        <Card title="系统信息">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
            <div>
              <span className="text-[var(--text-secondary)]">状态: </span>
              <span className="text-[var(--text-primary)]">{health.status}</span>
            </div>
            <div>
              <span className="text-[var(--text-secondary)]">版本: </span>
              <span className="text-[var(--text-primary)]">{health.version}</span>
            </div>
            <div>
              <span className="text-[var(--text-secondary)]">运行时间: </span>
              <span className="text-[var(--text-primary)]">{Math.floor(health.uptime / 3600)}h</span>
            </div>
            <div>
              <span className="text-[var(--text-secondary)]">下次 Cron: </span>
              <span className="text-[var(--text-primary)]">
                {cronStatus?.nextWakeAtMs ? new Date(cronStatus.nextWakeAtMs).toLocaleString() : 'N/A'}
              </span>
            </div>
          </div>
        </Card>
      )}

      {instances.length > 0 && (
        <Card title="在线实例" className="mt-6">
          <div className="space-y-2">
            {instances.slice(0, 5).map((entry, i) => (
              <div key={i} className="flex items-center justify-between py-2 border-b border-[var(--border-base)] last:border-0">
                <div>
                  <p className="text-[var(--text-primary)]">{entry.host || '未知主机'}</p>
                  <p className="text-sm text-[var(--text-secondary)]">{entry.mode}</p>
                </div>
                <div className="flex gap-2">
                  {entry.platform && (
                    <span className="px-2 py-1 text-xs bg-[var(--bg-secondary)] rounded">{entry.platform}</span>
                  )}
                  {entry.roles?.map((role) => (
                    <span key={role} className="px-2 py-1 text-xs bg-[var(--brand-500)] text-white rounded">{role}</span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}
    </div>
  );
}