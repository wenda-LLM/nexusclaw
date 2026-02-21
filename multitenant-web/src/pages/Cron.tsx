import { useState, useEffect } from 'react';
import { RefreshCw, Plus, Clock, Edit, Trash2, Play } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { Modal } from '@/components/common/Modal';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { CronJob } from '@/types';

export function CronPage() {
  const [loading, setLoading] = useState(false);
  const [jobs, setJobs] = useState<CronJob[]>([]);
  const [showModal, setShowModal] = useState(false);
  const [editingJob, setEditingJob] = useState<CronJob | null>(null);
  const [form, setForm] = useState({ name: '', schedule: '0 8 * * *', message: '', sessionKey: 'main' });

  const loadJobs = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<{ jobs: CronJob[] }>('cron.list', {});
      setJobs(result.jobs || []);
    } catch (e) {
      console.error('Failed to load cron jobs:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      if (editingJob) {
        await gateway.request('cron.update', { id: editingJob.id, ...form });
      } else {
        await gateway.request('cron.add', form);
      }
      setShowModal(false);
      setEditingJob(null);
      setForm({ name: '', schedule: '0 8 * * *', message: '', sessionKey: 'main' });
      loadJobs();
    } catch (e) {
      console.error('Failed to save cron job:', e);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('确定删除此定时任务？')) return;
    try {
      await gateway.request('cron.remove', { id });
      loadJobs();
    } catch (e) {
      console.error('Failed to delete cron job:', e);
    }
  };

  const handleRun = async (id: string) => {
    try {
      await gateway.request('cron.run', { id });
    } catch (e) {
      console.error('Failed to run cron job:', e);
    }
  };

  const handleToggle = async (job: CronJob) => {
    try {
      await gateway.request('cron.toggle', { id: job.id, enabled: !job.enabled });
      loadJobs();
    } catch (e) {
      console.error('Failed to toggle cron job:', e);
    }
  };

  useEffect(() => {
    loadJobs();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('cron.title')}</h1>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={loadJobs} loading={loading}>
            <RefreshCw className="w-4 h-4 mr-2" />
            刷新
          </Button>
          <Button onClick={() => setShowModal(true)}>
            <Plus className="w-4 h-4 mr-2" />
            {t('cron.add')}
          </Button>
        </div>
      </div>

      <Card>
        {jobs.length === 0 ? (
          <div className="text-center py-12 text-[var(--text-muted)]">
            <Clock className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>{t('common.noData')}</p>
          </div>
        ) : (
          <div className="space-y-4">
            {jobs.map((job) => (
              <div
                key={job.id}
                className="flex items-center justify-between py-3 border-b border-[var(--border-base)] last:border-0"
              >
                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-medium text-[var(--text-primary)]">{job.name}</span>
                    <span className={`text-xs px-2 py-0.5 rounded ${
                      job.enabled ? 'bg-[var(--color-success)]/10 text-[var(--color-success)]' : 'bg-[var(--text-muted)]/10 text-[var(--text-muted)]'
                    }`}>
                      {job.enabled ? t('cron.enabled') : t('cron.disabled')}
                    </span>
                  </div>
                  <p className="text-sm text-[var(--text-secondary)] mt-1">
                    {job.schedule} • {job.message}
                  </p>
                  {job.nextRunAt && (
                    <p className="text-xs text-[var(--text-muted)] mt-1">
                      下次执行: {new Date(job.nextRunAt).toLocaleString()}
                    </p>
                  )}
                </div>
                <div className="flex items-center gap-2">
                  <Button variant="ghost" size="sm" onClick={() => handleRun(job.id)}>
                    <Play className="w-4 h-4" />
                  </Button>
                  <Button variant="ghost" size="sm" onClick={() => handleToggle(job)}>
                    <Clock className="w-4 h-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => {
                      setEditingJob(job);
                      setForm({ name: job.name, schedule: job.schedule, message: job.message, sessionKey: job.sessionKey });
                      setShowModal(true);
                    }}
                  >
                    <Edit className="w-4 h-4" />
                  </Button>
                  <Button variant="ghost" size="sm" onClick={() => handleDelete(job.id)} className="text-[var(--color-danger)]">
                    <Trash2 className="w-4 h-4" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      <Modal
        isOpen={showModal}
        onClose={() => {
          setShowModal(false);
          setEditingJob(null);
          setForm({ name: '', schedule: '0 8 * * *', message: '', sessionKey: 'main' });
        }}
        title={editingJob ? t('cron.edit') : t('cron.add')}
        size="md"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowModal(false)}>
              {t('common.cancel')}
            </Button>
            <Button onClick={handleSave} disabled={!form.name || !form.message}>
              {t('common.save')}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t('cron.name')}
            value={form.name}
            onChange={(e) => setForm({ ...form, name: e.target.value })}
            placeholder="每日早安提醒"
          />
          <Input
            label={t('cron.schedule')}
            value={form.schedule}
            onChange={(e) => setForm({ ...form, schedule: e.target.value })}
            placeholder="0 8 * * *"
          />
          <Input
            label={t('cron.message')}
            value={form.message}
            onChange={(e) => setForm({ ...form, message: e.target.value })}
            placeholder="每天早上8点向我问好"
          />
          <Input
            label={t('cron.session')}
            value={form.sessionKey}
            onChange={(e) => setForm({ ...form, sessionKey: e.target.value })}
            placeholder="main"
          />
        </div>
      </Modal>
    </div>
  );
}