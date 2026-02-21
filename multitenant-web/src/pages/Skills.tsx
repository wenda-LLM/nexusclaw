import { useState, useEffect } from 'react';
import { RefreshCw, Zap, Save, ToggleLeft, ToggleRight } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { Modal } from '@/components/common/Modal';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { Skill } from '@/types';

export function SkillsPage() {
  const [loading, setLoading] = useState(false);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [editingKey, setEditingKey] = useState<string | null>(null);
  const [apiKey, setApiKey] = useState('');

  const loadSkills = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<{ skills: Skill[] }>('skills.list', {});
      setSkills(result.skills || []);
    } catch (e) {
      console.error('Failed to load skills:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleToggle = async (key: string, enabled: boolean) => {
    try {
      await gateway.request('skills.toggle', { key, enabled: !enabled });
      loadSkills();
    } catch (e) {
      console.error('Failed to toggle skill:', e);
    }
  };

  const handleSaveApiKey = async (key: string) => {
    try {
      await gateway.request('skills.setKey', { key, apiKey });
      setEditingKey(null);
      setApiKey('');
    } catch (e) {
      console.error('Failed to save API key:', e);
    }
  };

  useEffect(() => {
    loadSkills();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('skills.title')}</h1>
        <Button variant="secondary" onClick={loadSkills} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <Card>
        {skills.length === 0 ? (
          <div className="text-center py-12 text-[var(--text-muted)]">
            <Zap className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>{t('common.noData')}</p>
          </div>
        ) : (
          <div className="space-y-4">
            {skills.map((skill) => (
              <div
                key={skill.key}
                className="flex items-center justify-between py-3 border-b border-[var(--border-base)] last:border-0"
              >
                <div className="flex-1">
                  <div className="flex items-center gap-2">
                    <Zap className="w-5 h-5 text-[var(--brand-500)]" />
                    <span className="font-medium text-[var(--text-primary)]">{skill.name}</span>
                  </div>
                  <p className="text-sm text-[var(--text-secondary)] mt-1">{skill.description}</p>
                  {skill.apiKey && (
                    <p className="text-xs text-[var(--color-success)] mt-1">API Key 已配置</p>
                  )}
                </div>
                <div className="flex items-center gap-3">
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={() => {
                      setEditingKey(skill.key);
                      setApiKey(skill.apiKey || '');
                    }}
                  >
                    <Save className="w-4 h-4 mr-1" />
                    {t('skills.apiKey')}
                  </Button>
                  <button
                    onClick={() => handleToggle(skill.key, skill.enabled)}
                    className="text-[var(--text-secondary)]"
                  >
                    {skill.enabled ? (
                      <ToggleRight className="w-6 h-6 text-[var(--color-success)]" />
                    ) : (
                      <ToggleLeft className="w-6 h-6" />
                    )}
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      <Modal
        isOpen={!!editingKey}
        onClose={() => setEditingKey(null)}
        title={t('skills.apiKey')}
        size="md"
        footer={
          <>
            <Button variant="secondary" onClick={() => setEditingKey(null)}>
              {t('common.cancel')}
            </Button>
            <Button onClick={() => editingKey && handleSaveApiKey(editingKey)}>
              {t('common.save')}
            </Button>
          </>
        }
      >
        <Input
          label={t('skills.apiKey')}
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder="sk-..."
        />
      </Modal>
    </div>
  );
}